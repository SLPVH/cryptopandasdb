use diesel::pg::PgConnection;
use diesel::data_types::PgNumeric;
use diesel::prelude::*;

use cashcontracts::{Address, AddressType};
use slpdexdb_base::{BlockHeader, GENESIS};
use slpdexdb_base::SLPAmount;
use slpdexdb_base::convert_numeric::{rational_to_pg_numeric, pg_numeric_to_rational};
use crate::tx_history::{TxHistory, TxType, TradeOffer, TokenType};
use crate::update_history::{UpdateHistory, UpdateSubject};
use crate::token::Token;
use crate::{models, schema::*};
use crate::convert::pg_safe_string;
use crate::data::{Utxo, NewUtxo, SpentUtxo, TxDelta, tx_hash_from_slice, address_hash_from_slice,
                  TradeOfferFilter};

use std::collections::{HashMap, HashSet, BTreeSet};

const PRICE_DIGITS: u16 = 26;

pub struct Db {
    connection: PgConnection,
}

impl Db {
    pub fn new(connection: PgConnection) -> Self {
        Db { connection }
    }

    pub fn connection(&self) -> &PgConnection {
        &self.connection
    }

    pub fn add_headers(&self, headers: &[BlockHeader]) -> QueryResult<()> {
        let mut remaining_visit = (0..headers.len()).into_iter().collect::<BTreeSet<_>>();
        let mut heights = self.header_tips(10)?
            .iter()
            .map(|(header, height)| (header.hash(), *height))
            .collect::<HashMap<_, _>>();
        if heights.len() == 0 {
            diesel::insert_into(blocks::table)
                .values(&models::Block::from_block_header(&GENESIS, 0))
                .execute(&self.connection)?;
            heights.insert(GENESIS.hash(), 0);
        }
        loop {
            let tuple = remaining_visit
                .iter().cloned()
                .find_map(|i|
                    if headers[i].prev_block != [0; 32] {
                        heights.get(&headers[i].prev_block).map(|i| *i + 1)
                    } else {
                        Some(0)
                    }.map(|height| (i, &headers[i], height))
                );
            match tuple {
                Some((i, header, height)) => {
                    heights.insert(header.hash(), height);
                    remaining_visit.remove(&i);
                },
                None => break,
            };
        }
        let db_blocks = headers.iter().map(|header| {
            models::Block::from_block_header(header, heights[&header.hash()])
        }).collect::<Vec<_>>();
        diesel::insert_into(blocks::table)
            .values(&db_blocks)
            .execute(&self.connection)?;
        Ok(())
    }

    pub fn header_tips(&self, n_recent: i64) -> QueryResult<Vec<(BlockHeader, i32)>> {
        Ok(blocks::table
            .order(blocks::height.desc())
            .limit(n_recent)
            .load::<models::Block>(&self.connection)?
            .into_iter()
            .map(|block: models::Block| (block.to_block_header(), block.height))
            .collect())
    }

    pub fn header_tip(&self) -> QueryResult<Option<(BlockHeader, i32)>> {
        let mut tips = self.header_tips(1)?;
        if tips.len() == 0 {
            Ok(None)
        } else {
            Ok(Some(tips.remove(0)))
        }
    }

    pub fn set_address_active(&self, address: &Address, is_active: bool) -> QueryResult<()> {
        if is_active {
            diesel::insert_into(active_address::table)
                .values(models::ActiveAddress { address: address.bytes().to_vec() })
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
        } else {
            diesel::delete(active_address::table)
                .filter(
                    active_address::address.eq(address.bytes().to_vec())
                )
                .execute(&self.connection)?;
        }
        Ok(())
    }

    pub fn add_tx_history(&self, tx_history: &TxHistory) -> QueryResult<()> {
        self.connection.transaction(|| {
            let token_hashes = tx_history.txs.iter()
                .filter_map(|tx| {
                    match tx.tx_type {
                        TxType::SLP {token_hash, ..} => Some(token_hash.to_vec()),
                        TxType::Default => None,
                    }
                })
                .collect::<HashSet<_>>();
            let new_txs = tx_history.txs.iter().map(|tx| {
                models::NewTx {
                    hash: tx.hash.to_vec(),
                    height: tx.height,
                    tx_type: tx.tx_type.id(),
                    timestamp: tx.timestamp,
                }
            }).collect::<Vec<_>>();
            let tokens: Vec<(Vec<u8>, i32)> = token::table
                .select((token::hash, token::id))
                .filter(token::hash.eq_any(token_hashes))
                .load(&self.connection)?;
            let token_ids = tokens.into_iter().collect::<HashMap<_, _>>();
            let tx_ids = diesel::insert_into(tx::table)
                .values(&new_txs)
                .on_conflict(tx::hash)
                .do_update().set((tx::height.eq(tx::height),
                                  tx::tx_type.eq(tx::tx_type),
                                  tx::timestamp.eq(tx::timestamp)))
                .returning(tx::id)
                .get_results::<i64>(&self.connection)?;
            let new_slp_txs = tx_history.txs
                .iter()
                .zip(tx_ids.iter().cloned())
                .filter_map(|(tx, id)| {
                    match &tx.tx_type {
                        TxType::SLP {token_hash, token_type, slp_type} => Some(models::SlpTx {
                            tx: id,
                            slp_type: String::from_utf8_lossy(slp_type.to_bytes()).to_string(),
                            token: *token_ids.get(&token_hash[..])?,
                            version: *token_type as i32,
                        }),
                        TxType::Default => None,
                    }
                }).collect::<Vec<_>>();
            diesel::insert_into(slp_tx::table)
                .values(&new_slp_txs)
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
            let new_outputs = tx_history.txs.iter()
                .zip(tx_ids.iter().cloned())
                .flat_map(|(tx, id)| {
                    tx.outputs.iter().enumerate().map(move |(output_idx, output)|
                        models::TxOutput {
                            tx: id,
                            idx: output_idx as i32,
                            value_satoshis: output.value_satoshis as i64,
                            value_token_base: output.value_token.into(),
                            address: output.output.address()
                                .map(|addr| addr.bytes().to_vec()),
                            output_type: output.output.id(),
                        })
                })
                .collect::<Vec<_>>();
            diesel::insert_into(tx_output::table)
                .values(&new_outputs)
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
            let new_inputs = tx_history.txs.iter()
                .zip(tx_ids.iter().cloned())
                .flat_map(|(tx, id)| {
                    tx.inputs.iter().enumerate().map(move |(input_idx, input)| {
                        models::TxInput {
                            tx: id,
                            idx: input_idx as i32,
                            output_tx: input.output_tx.to_vec(),
                            output_idx: input.output_idx,
                            address: input.output.address()
                                .map(|addr| addr.bytes().to_vec()),
                        }
                    })
                })
                .collect::<Vec<_>>();
            diesel::insert_into(tx_input::table)
                .values(&new_inputs)
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
            let new_trade_offers = tx_history.trade_offers
                .iter()
                .map(|(tx_idx, trade_offer)| {
                    models::NewTradeOffer {
                        tx: tx_ids[*tx_idx],
                        output_idx: trade_offer.output_idx,
                        input_idx: trade_offer.input_idx,
                        input_tx: trade_offer.input_tx.to_vec(),
                        price_per_token: rational_to_pg_numeric(trade_offer.price_per_token.clone(),
                                                                PRICE_DIGITS),
                        is_inverted: trade_offer.is_inverted,
                        script_price: trade_offer.script_price,
                        sell_amount_token_base: trade_offer.sell_amount_token.into(),
                        receiving_address: trade_offer.receiving_address.bytes().to_vec(),
                    }
                })
                .collect::<Vec<_>>();
            diesel::insert_into(trade_offer::table)
                .values(&new_trade_offers)
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
            let new_pending_pnd1_txs = tx_history.pnd_txs
                .iter()
                .map(|(tx_idx, pnd_tx)| {
                    models::PND1Tx {
                        name: pnd_tx.name.clone(),
                        tx: tx_ids[*tx_idx],
                        father: pnd_tx.father_id,
                        mother: pnd_tx.mother_id,
                        owner_address: pnd_tx.owner_address.bytes().to_vec(),
                    }
                })
                .collect::<Vec<_>>();
            diesel::insert_into(pending_pnd1_tx::table)
                .values(&new_pending_pnd1_txs)
                .on_conflict_do_nothing()
                .execute(&self.connection)?;
            Ok(())
        })
    }

    pub fn last_update(&self, subject: UpdateSubject) -> QueryResult<Option<UpdateHistory>> {
        let query = update_history::table
            .filter(update_history::subject_type.eq(subject.subject_type as i32))
            .filter(update_history::is_confirmed.eq(subject.is_confirmed))
            .order(update_history::timestamp.desc())
            .limit(1);
        let update: Option<models::UpdateHistory> = match subject.hash.clone() {
            Some(subject_hash) => query
                .filter(update_history::subject_hash.eq(subject_hash))
                .first::<models::UpdateHistory>(&self.connection)
                .optional()?,
            None => query
                .first::<models::UpdateHistory>(&self.connection)
                .optional()?,
        };
        Ok(update.map(|update| {
            UpdateHistory {
                last_height: update.last_height,
                last_tx_hash: update.last_tx_hash,
                completed: update.completed,
                subject,
            }
        }))
    }

    pub fn add_update_history(&self, update_history: &UpdateHistory) -> QueryResult<()> {
        diesel::insert_into(update_history::table)
            .values(&models::NewUpdateHistory {
                last_height: update_history.last_height,
                last_tx_hash: update_history.last_tx_hash.clone(),
                last_tx_hash_be: update_history.last_tx_hash.as_ref().map(|hash| {
                    let mut hash = hash.clone();
                    hash.reverse();
                    hash
                }),
                subject_type: update_history.subject.subject_type as i32,
                subject_hash: update_history.subject.hash.clone(),
                completed: update_history.completed,
                is_confirmed: update_history.subject.is_confirmed,
            })
            .execute(&self.connection)?;
        Ok(())
    }

    pub fn add_tokens(&self, tokens: &[Token]) -> QueryResult<()> {
        diesel::insert_into(token::table)
            .values(&tokens.iter()
                .map(|token| {
                    models::NewToken {
                        hash: token.hash.to_vec(),
                        decimals: token.decimals,
                        timestamp: token.timestamp,
                        version_type: token.version_type as i16,
                        document_uri: token.document_uri.clone().map(pg_safe_string),
                        symbol: token.symbol.clone().map(pg_safe_string),
                        name: token.name.clone().map(pg_safe_string),
                        document_hash: token.document_hash.clone(),
                        initial_supply: token.initial_supply.into(),
                        current_supply: token.current_supply.into(),
                        block_created_height: token.block_created_height,
                        parent_token: None,
                        parent_token_hash: token.parent_hash.clone().map(|hash| hash.to_vec()),
                    }
                })
                .collect::<Vec<_>>()
            )
            .on_conflict(token::hash)
            .do_update().set((
                token::current_supply.eq(token::current_supply),
                token::parent_token.eq(token::parent_token),
                token::parent_token_hash.eq(token::parent_token_hash),
            ))
            .execute(&self.connection)?;
        Ok(())
    }

    pub fn token(&self, token_hash: &[u8; 32]) -> QueryResult<Option<Token>> {
        let token: Option<models::Token> = token::table
            .filter(token::hash.eq(token_hash.to_vec()))
            .first::<models::Token>(&self.connection)
            .optional()?;
        Ok(token.and_then(|token| {
            Some(Token {
                hash: tx_hash_from_slice(&token.hash),
                parent_hash: token.parent_token_hash.as_ref().map(|h| tx_hash_from_slice(h)),
                decimals: token.decimals,
                timestamp: token.timestamp,
                version_type: num::FromPrimitive::from_u16(token.version_type as u16)?,
                document_uri: token.document_uri,
                symbol: token.symbol,
                name: token.name,
                document_hash: token.document_hash,
                initial_supply: SLPAmount::from_numeric_decimals(&token.initial_supply,
                                                                 token.decimals as u32),
                current_supply: SLPAmount::from_numeric_decimals(&token.current_supply,
                                                                 token.decimals as u32),
                block_created_height: token.block_created_height,
            })
        }))
    }

    pub fn update_utxo_set(&self, address: &cashcontracts::Address) -> QueryResult<()> {
        self.connection.transaction(|| {
            diesel::delete(utxo_address::table)
                .filter(
                    utxo_address::address.eq(address.bytes().to_vec())
                )
                .execute(&self.connection)?;
            diesel::insert_into(utxo_address::table)
                .values(
                    tx_output::table
                        .left_join(tx::table)
                        .left_outer_join(tx_input::table.on(
                            tx::hash.eq(tx_input::output_tx)
                                .and(tx_output::idx.eq(tx_input::output_idx))
                        ))
                        .filter(tx_input::tx.is_null())
                        .filter(tx_output::address.eq(address.bytes().to_vec()))
                        .select((tx_output::tx, tx_output::idx, tx_output::address))
                )
                .execute(&self.connection)?;
            Ok(())
        })
    }

    pub fn update_utxo_set_exch(&self) -> QueryResult<()> {
        use diesel::dsl::*;
        self.connection.transaction(|| {
            diesel::delete(utxo_trade_offer::table)
                .execute(&self.connection)?;
            diesel::insert_into(utxo_trade_offer::table)
                .values(
                    tx_output::table
                        .left_join(tx::table)
                        .inner_join(trade_offer::table.on(
                            tx_output::tx.eq(trade_offer::tx)
                                .and(tx_output::idx.nullable().eq(trade_offer::output_idx))
                                .and(not(trade_offer::output_idx.is_null()))
                        ))
                        .left_outer_join(tx_input::table.on(
                            tx::hash.eq(tx_input::output_tx)
                                .and(tx_output::idx.eq(tx_input::output_idx))
                        ))
                        .filter(tx_input::tx.is_null())
                        .select((tx_output::tx, tx_output::idx))
                )
                .on_conflict_do_nothing()  // this shouldn't happen
                .execute(&self.connection)?;
            Ok(())
        })
    }

    pub fn utxos_address(&self, address: &Address) -> QueryResult<Vec<Utxo>> {
        let result = tx_output::table
            .inner_join(utxo_address::table.on(
                tx_output::tx.eq(utxo_address::tx).and(tx_output::idx.eq(utxo_address::idx))
            ))
            .inner_join(tx::table)
            .left_join(slp_tx::table.on(tx::id.eq(slp_tx::tx)))
            .left_join(token::table.on(slp_tx::token.eq(token::id)))
            .filter(utxo_address::address.eq(address.bytes().to_vec()))
            .select((tx::hash,
                     tx_output::idx,
                     tx_output::value_satoshis,
                     tx_output::value_token_base,
                     token::hash.nullable(),
                     token::decimals.nullable()))
            .load::<(Vec<u8>, i32, i64, PgNumeric, Option<Vec<u8>>, Option<i32>)>(&self.connection)?;
        Ok(result.into_iter()
            .map(|(tx_hash, vout, value_satoshis, value_token_base, token_hash, decimals)| {
                let slp_amount = decimals.map(
                    |decimals| SLPAmount::from_numeric_decimals(&value_token_base, decimals as u32)
                ).unwrap_or(SLPAmount::new(0, 0));
                Utxo {
                    tx_hash: tx_hash_from_slice(&tx_hash),
                    vout,
                    value_satoshis: value_satoshis as u64,
                    value_token: slp_amount,
                    token_hash: token_hash
                        .filter(|_| slp_amount.base_amount() > 0)
                        .map(|token_hash| tx_hash_from_slice(&token_hash)),
                }
            })
            .collect())
    }

    pub fn address_tx_deltas(&self, address: &Address) -> QueryResult<Vec<TxDelta>> {
        use diesel::sql_types::Binary;
        let input_query = diesel::sql_query("\
            SELECT
                tx.id AS tx_id,
                tx.hash AS tx_hash,
                tx.timestamp AS timestamp,
                SUM(tx_input_output.value_satoshis)::NUMERIC AS input_value_satoshis,
                SUM(tx_input_output.value_token_base) AS input_value_token_base,
                token.hash AS token_hash,
                token.decimals AS decimals
            FROM tx
                LEFT JOIN slp_tx                       ON (tx.id = slp_tx.tx)
                LEFT JOIN token                        ON (token.id = slp_tx.token)
                LEFT JOIN tx_input                     ON (tx.id = tx_input.tx AND
                                                           tx_input.address = $1)
                LEFT JOIN tx        AS tx_input_tx     ON (tx_input_tx.hash = tx_input.output_tx)
                LEFT JOIN tx_output AS tx_input_output ON (tx_input_tx.id = tx_input_output.tx AND
                                                           tx_input.output_idx = tx_input_output.idx)
            WHERE
                tx_input.address = $1
            GROUP BY tx.id, tx.hash, token.hash, token.decimals
        ").bind::<Binary, _>(address.bytes().to_vec());
        let output_query = diesel::sql_query("\
            SELECT
                tx.id AS tx_id,
                tx.hash AS tx_hash,
                tx.timestamp AS timestamp,
                SUM(tx_output.value_satoshis)::NUMERIC AS output_value_satoshis,
                SUM(tx_output.value_token_base) AS output_value_token_base,
                token.hash AS token_hash,
                token.decimals AS decimals
            FROM tx
                LEFT JOIN slp_tx                       ON (tx.id = slp_tx.tx)
                LEFT JOIN token                        ON (token.id = slp_tx.token)
                LEFT JOIN tx_output                    ON (tx.id = tx_output.tx AND
                                                           tx_output.address = $1)
            WHERE
                tx_output.address = $1
            GROUP BY tx.id, tx.hash, token.hash, token.decimals
        ").bind::<Binary, _>(address.bytes().to_vec());
        let mut result_input = input_query
            .load::<models::TxDeltaInput>(&self.connection)?
            .into_iter()
            .map(|delta_input| (delta_input.tx_id, delta_input))
            .collect::<HashMap<_, _>>();
        let mut result_output = output_query
            .load::<models::TxDeltaOutput>(&self.connection)?
            .into_iter()
            .map(|delta_output| (delta_output.tx_id, delta_output))
            .collect::<HashMap<_, _>>();
        let tx_ids = result_input.keys().cloned()
            .chain(result_output.keys().cloned())
            .collect::<HashSet<_>>();
        Ok(tx_ids.into_iter()
            .map(|tx_id| {
                let delta_input = result_input.remove(&tx_id);
                let delta_output = result_output.remove(&tx_id);
                let (decimals, token_hash) = delta_input.as_ref()
                    .and_then(
                        |delta_input| Some((delta_input.decimals?,
                                            Some(delta_input.token_hash.clone()?)))
                    )
                    .or_else(|| delta_output.as_ref().and_then(|delta_output| {
                        Some((delta_output.decimals?, Some(delta_output.token_hash.clone()?)))
                    }))
                    .unwrap_or((0, None));
                let decimals = decimals as u32;
                let zero = SLPAmount::new(0, decimals);
                let map_numeric = |numeric: &Option<PgNumeric>| numeric.as_ref()
                    .map(|numeric| SLPAmount::from_numeric_decimals(numeric, decimals))
                    .unwrap_or(zero);
                let (input_value_satoshis, input_value_token) = delta_input.as_ref()
                    .map(|delta_input| (
                        map_numeric(&delta_input.input_value_satoshis).base_amount(),
                        map_numeric(&delta_input.input_value_token_base),
                    ))
                    .unwrap_or((0, zero));
                let (output_value_satoshis, output_value_token) = delta_output.as_ref()
                    .map(|delta_output| (
                        map_numeric(&delta_output.output_value_satoshis).base_amount(),
                        map_numeric(&delta_output.output_value_token_base),
                    ))
                    .unwrap_or((0, zero));
                let delta_satoshis = output_value_satoshis - input_value_satoshis;
                let delta_token = output_value_token - input_value_token;
                TxDelta {
                    tx_hash: tx_hash_from_slice(
                        &delta_output.as_ref()
                            .map(|delta_output| delta_output.tx_hash.clone())
                            .unwrap_or_else(|| delta_input.as_ref().unwrap().tx_hash.clone())
                    ),
                    token_hash: token_hash.filter(|_| delta_token.base_amount() != 0)
                        .map(|token_hash| tx_hash_from_slice(&token_hash)),
                    delta_satoshis: delta_satoshis as i64,
                    delta_token,
                    timestamp: delta_output.as_ref()
                        .map(|delta_output| delta_output.timestamp)
                        .unwrap_or_else(|| delta_input.as_ref().unwrap().timestamp),
                }
            })
            .collect())
    }

    pub fn trade_offer_utxos(&self, filter: TradeOfferFilter) -> QueryResult<Vec<TradeOffer>> {
        use super::schema::trade_offer as t;
        type Q = (Vec<u8>, Option<i32>,   Vec<u8>,     i32,          i64,
                  PgNumeric,                Vec<u8>,              PgNumeric,          bool,
                  i32);
        let s = (tx::hash, t::output_idx, t::input_tx, t::input_idx, t::script_price,
                 t::sell_amount_token_base, t::receiving_address, t::price_per_token, t::is_inverted,
                 token::decimals);
        let tables = trade_offer::table
            .inner_join(tx::table)
            .inner_join(utxo_trade_offer::table.on(tx::id.eq(utxo_trade_offer::tx)))
            .inner_join(slp_tx::table.on(tx::id.eq(slp_tx::tx)))
            .inner_join(token::table.on(slp_tx::token.eq(token::id)))
            .select(s);
        let result = match filter {
            TradeOfferFilter::TokenHash(token_hash) => tables
                .filter(tx::hash.eq(token_hash.to_vec()))
                .load::<Q>(&self.connection)?,
            TradeOfferFilter::ReceivingAddress(address) => tables
                .filter(trade_offer::receiving_address.eq(address.bytes().to_vec()))
                .load::<Q>(&self.connection)?,
        };
        Ok(result
            .into_iter()
            .filter_map(|(tx_hash, output_idx, input_tx, input_idx, script_price,
                          sell_amount_token_base, receiving_address, price_per_token, is_inverted,
                          decimals)| {
                Some(TradeOffer {
                    tx: tx_hash_from_slice(&tx_hash),
                    output_idx,
                    input_tx: tx_hash_from_slice(&input_tx),
                    input_idx,
                    price_per_token: pg_numeric_to_rational(&price_per_token).ok()?,
                    script_price,
                    is_inverted,
                    sell_amount_token: SLPAmount
                        ::from_numeric_decimals(&sell_amount_token_base, decimals as u32),
                    receiving_address: Address
                        ::from_bytes(AddressType::P2PKH, address_hash_from_slice(&receiving_address)),
                })
            })
            .collect())
    }

    pub fn txs(&self, tx_hashes: impl Iterator<Item=[u8; 32]>)
            -> QueryResult<HashMap<[u8; 32], models::Tx>> {
        Ok(tx::table
            .filter(tx::hash.eq_any(
                tx_hashes.map(|tx_hash| tx_hash.to_vec()).collect::<Vec<_>>()
            ))
            .load::<models::Tx>(&self.connection)?
            .into_iter()
            .map(|tx| (tx_hash_from_slice(&tx.hash), tx))
            .collect())
    }

    pub fn slp_txs(&self, tx_hashes: impl Iterator<Item=[u8; 32]>)
               -> QueryResult<HashMap<[u8; 32], (models::Tx, models::SlpTx, models::Token)>> {
        Ok(tx::table
            .inner_join(slp_tx::table)
            .inner_join(token::table.on(slp_tx::token.eq(token::id)))
            .filter(tx::hash.eq_any(
                tx_hashes.map(|tx_hash| tx_hash.to_vec()).collect::<Vec<_>>()
            ))
            .load::<(models::Tx, models::SlpTx, models::Token)>(&self.connection)?
            .into_iter()
            .map(|(tx, slp_tx, token)| (tx_hash_from_slice(&tx.hash), (tx, slp_tx, token)))
            .collect())
    }

    pub fn tx_outputs(&self, tx_hashes: impl Iterator<Item=[u8; 32]>)
               -> QueryResult<HashMap<([u8; 32], i32), models::TxOutput>> {
        Ok(tx_output::table
            .inner_join(tx::table)
            .filter(tx::hash.eq_any(
                tx_hashes.map(|tx_hash| tx_hash.to_vec()).collect::<Vec<_>>()
            ))
            .select((tx::hash, tx_output::tx, tx_output::idx, tx_output::value_satoshis,
                     tx_output::value_token_base, tx_output::address, tx_output::output_type))
            .load::<(Vec<u8>, i64, i32, i64, PgNumeric, Option<Vec<u8>>, i32)>(&self.connection)?
            .into_iter()
            .map(|(hash, tx, idx, value_satoshis, value_token_base, address, output_type)|
                ((tx_hash_from_slice(&hash), idx),
                 models::TxOutput {tx, idx, value_satoshis, value_token_base, address, output_type})
            )
            .collect())
    }

    pub fn remove_utxos(&self, utxos: &[SpentUtxo]) -> QueryResult<()> {
        let txs = self.txs(utxos.iter().map(|utxo| utxo.tx_hash))?;
        for utxo in utxos {
            let tx = &txs[&utxo.tx_hash];
            diesel::delete(utxo_address::table)
                .filter(utxo_address::tx.eq(tx.id).and(utxo_address::idx.eq(utxo.vout)))
                .execute(&self.connection)?;
            diesel::delete(utxo_trade_offer::table)
                .filter(utxo_trade_offer::tx.eq(tx.id).and(utxo_trade_offer::idx.eq(utxo.vout)))
                .execute(&self.connection)?;
        }
        Ok(())
    }

    pub fn add_utxos(&self, utxos: &[NewUtxo]) -> QueryResult<()> {
        let txs = self.txs(utxos.iter().map(|utxo| match utxo {
            NewUtxo::Address {tx_hash, ..} => tx_hash.clone(),
            NewUtxo::TradeOffer {tx_hash, ..} => tx_hash.clone(),
        }))?;
        diesel::insert_into(utxo_address::table)
            .values(utxos.iter()
                .filter_map(|utxo| match utxo {
                    NewUtxo::Address {tx_hash, vout, address} => Some(models::UtxoAddress {
                        tx: txs[tx_hash].id,
                        idx: *vout,
                        address: Some(address.bytes().to_vec()),
                    }),
                    NewUtxo::TradeOffer {..} => None,
                })
                .collect::<Vec<_>>()
            )
            .execute(&self.connection)?;
        diesel::insert_into(utxo_trade_offer::table)
            .values(utxos.iter()
                .filter_map(|utxo| match utxo {
                    NewUtxo::Address {..} => None,
                    NewUtxo::TradeOffer {tx_hash, vout} => Some(models::Utxo {
                        tx: txs[tx_hash].id,
                        idx: *vout,
                    }),
                })
                .collect::<Vec<_>>()
            )
            .execute(&self.connection)?;
        Ok(())
    }

    pub fn pending_pnd(&self) -> QueryResult<Vec<(models::PND1Tx, models::Tx)>> {
        pending_pnd1_tx::table
            .inner_join(tx::table)
            .load(&self.connection)
    }

    pub fn get_some_pandaop_utxo(&self) -> QueryResult<Option<models::PandaopUtxo>> {
        let utxo = pandaop_utxo::table
            .limit(1)
            .get_result::<models::PandaopUtxo>(&self.connection)
            .optional()?;
        match utxo {
            Some(utxo) => {
                diesel::delete(pandaop_utxo::table)
                    .filter(pandaop_utxo::tx_hash.eq(utxo.tx_hash.clone()))
                    .filter(pandaop_utxo::vout.eq(utxo.vout))
                    .execute(&self.connection)?;
                Ok(Some(utxo))
            },
            None => Ok(None)
        }
    }
}
