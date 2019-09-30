use crate::tx_source::{tx_result, TxSource, TxFilter, Confirmedness};
use slpdexdb_base::{SLPDEXConfig, SLPAmount, Result, Error, ErrorKind, SLPError, TokenError, TradeOfferError, PNDError};
use crate::token::Token;
use crate::db::Db;
use crate::data::{tx_hash_from_slice, tx_hash_from_le_slice};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io;
use std::collections::{HashSet, HashMap};
use cashcontracts::{Output, P2PKHOutput, AddressType, Address, tx_hash_to_hex, tx_hex_to_hash, single_sha256, double_sha256};
use rug::Rational;
use crate::panda_tools::get_panda_by_owner_utxo;

#[derive(Clone, Debug)]
pub struct TxHistory {
    pub txs: Vec<HistoricTx>,
    pub trade_offers: HashMap<usize, TradeOffer>,
    pub pnd_txs: HashMap<usize, PND1Tx>,
}

#[derive(Clone, Debug)]
pub struct HistoricTx {
    pub hash: [u8; 32],
    pub height: Option<i32>,
    pub timestamp: i64,
    pub tx_type: TxType,
    pub inputs: Vec<HistoricTxInput>,
    pub outputs: Vec<HistoricTxOutput>,
}

#[derive(Clone, Debug)]
pub enum SLPTxType {
    Genesis,
    Mint,
    Send,
    Commit,
}

#[derive(Clone, Debug)]
pub enum TxType {
    Default,
    SLP {
        token_hash: [u8; 32],
        token_type: TokenType,
        slp_type: SLPTxType,
    },
}

#[derive(Copy, Clone, Debug, FromPrimitive, Eq, PartialEq)]
pub enum TokenType {
    Standard = 1,
    NFT1Parent = 0x81,
    NFT1Child = 0x41,
}

#[derive(Clone, Debug)]
pub enum OutputType {
    OpReturn,
    Unknown,
    Address(Address),
    Burned,
}

#[derive(Clone, Debug)]
pub struct HistoricTxOutput {
    pub value_satoshis: u64,
    pub value_token: SLPAmount,
    pub output: OutputType,
}

#[derive(Clone, Debug)]
pub struct HistoricTxInput {
    pub output_tx: [u8; 32],
    pub output_idx: i32,
    pub output: OutputType,
}

#[derive(Clone, Debug)]
pub struct PND1Tx {
    pub father_id: i64,
    pub mother_id: i64,
    pub name: String,
    pub father_tx_hash: [u8; 32],
    pub father_output_idx: u32,
    pub mother_tx_hash: [u8; 32],
    pub mother_output_idx: u32,
    pub public_key: Vec<u8>,
    pub signature: Vec<u8>,
}

#[derive(Clone, Debug)]
pub struct TradeOffer {
    pub tx: [u8; 32],
    pub output_idx: Option<i32>,
    pub input_tx: [u8; 32],
    pub input_idx: i32,
    pub price_per_token: Rational,
    pub script_price: i64,
    pub is_inverted: bool,
    pub sell_amount_token: SLPAmount,
    pub receiving_address: Address,
}

impl SLPTxType {
    pub fn from_bytes(s: &[u8]) -> Option<Self> {
        match s {
            b"GENESIS" => Some(SLPTxType::Genesis),
            b"SEND"    => Some(SLPTxType::Send),
            b"MINT"    => Some(SLPTxType::Mint),
            b"COMMIT"  => Some(SLPTxType::Commit),
            _         => None,
        }
    }

    pub fn to_bytes(&self) -> &[u8] {
        match self {
            SLPTxType::Genesis => b"GENESIS",
            SLPTxType::Send    => b"SEND",
            SLPTxType::Mint    => b"MINT",
            SLPTxType::Commit  => b"COMMIT",
        }
    }
}

impl OutputType {
    pub fn address(&self) -> Option<&Address> {
        match self {
            OutputType::Address(addr) => Some(addr),
            _ => None,
        }
    }

    pub fn id(&self) -> i32 {
        match self {
            OutputType::Unknown => 0,
            OutputType::OpReturn => 1,
            OutputType::Address(addr) => match addr.addr_type() {
                AddressType::P2SH => 2,
                AddressType::P2PKH => 3,
            },
            OutputType::Burned => 4,
        }
    }
}

impl TxType {
    pub fn token_hash(&self) -> Option<&[u8; 32]> {
        match self {
            TxType::Default => None,
            TxType::SLP {token_hash, ..} => Some(token_hash),
        }
    }

    pub fn id(&self) -> i32 {
        match self {
            TxType::Default => 1,
            TxType::SLP {..} => 2,
        }
    }
}

impl TxHistory {
    fn _process_address(address: &Option<String>) -> OutputType {
        match address {
            Some(addr) => {
                let address = if addr.contains(":") {
                    Address::from_cash_addr(addr.to_string())
                } else {
                    Address::from_cash_addr("bitcoincash:".to_string() + addr)
                };
                address.map(OutputType::Address).unwrap_or(OutputType::Unknown)
            },
            None => OutputType::Unknown
        }
    }

    pub fn from_entries(entries: &[tx_result::TxEntry],
                        now: i64,
                        config: &SLPDEXConfig) -> Self {
        let mut historic_txs = Vec::with_capacity(entries.len());
        let mut trade_offers = HashMap::new();
        let mut pnd_txs = HashMap::new();
        for entry in entries.iter() {
            let inputs = entry.inputs.iter()
                .map(|input| {
                    HistoricTxInput {
                        output_tx: cashcontracts::tx_hex_to_hash(&input.e.h).unwrap(),
                        output_idx: input.e.i,
                        output: Self::_process_address(&input.e.a)
                    }
                })
                .collect::<Vec<_>>();
            let outputs = entry.outputs.iter()
                .enumerate()
                .map(|(i, output)| {
                    HistoricTxOutput {
                        value_satoshis: output.e.v,
                        value_token: match &entry.slp {
                            Some(slp) if i > 0 => {
                                let decimals = slp.detail.decimals as u32;
                                slp.detail.outputs
                                    .get(i - 1)
                                    .and_then(|output|
                                        SLPAmount::from_str_decimals(&output.amount, decimals).ok()
                                    )
                                    .unwrap_or(SLPAmount::new(0, decimals))
                            },
                            _ => SLPAmount::new(0, 0),
                        },
                        output: if output.b0 == (tx_result::StackItem::Op {op: 0x6a}) {
                            OutputType::OpReturn
                        } else {
                            Self::_process_address(&output.e.a)
                        },
                    }
                })
                .collect::<Vec<_>>();
            let historic_tx = HistoricTx {
                hash: cashcontracts::tx_hex_to_hash(&entry.tx.h).unwrap(),
                height: entry.blk.as_ref().map(|blk| blk.i),
                timestamp: entry.blk.as_ref().map(|blk| blk.t as i64).unwrap_or(now),
                tx_type: entry.slp.as_ref()
                    .and_then(|slp| Some(TxType::SLP {
                        token_type: num::traits::FromPrimitive::from_u16(slp.detail.version_type as u16)?,
                        token_hash: tx_hex_to_hash(&slp.detail.token_id)?,
                        slp_type: SLPTxType::from_bytes(slp.detail.transaction_type.as_bytes())?,
                    }))
                    .unwrap_or(TxType::Default),
                inputs,
                outputs,
            };
            entry.slp.as_ref().and_then(|slp| {
                trade_offers.insert(
                    historic_txs.len(),
                    TradeOffer::from_entry(&historic_tx, entry, config, slp.detail.decimals as u32)?,
                );
                Some(())
            });
            historic_txs.push(historic_tx);
        }
        TxHistory {
            txs: historic_txs,
            trade_offers,
            pnd_txs,
        }
    }

    pub fn _process_input_script(script: &cashcontracts::Script) -> OutputType {
        use cashcontracts::{Op, OpCodeType::*};
        let ops = script.ops();
        if ops.len() == 0 { return OutputType::Unknown; }
        if ops[0] == Op::Code(OpReturn) { return OutputType::OpReturn; }
        if ops.len() != 2 { return OutputType::Unknown }
        match &ops[1] {
            Op::Push(pubkey) if pubkey.len() == 33 => {
                OutputType::Address(
                    Address::from_serialized_pub_key("bitcoincash", AddressType::P2PKH, pubkey)
                )
            },
            _ => OutputType::Unknown
        }
    }

    pub fn _process_output_script(script: &cashcontracts::Script) -> OutputType {
        use cashcontracts::{Op::*, OpCodeType::*};
        match script.ops() {
            &[Code(OpDup), Code(OpHash160), Push(ref address), Code(OpEqualVerify), Code(OpCheckSig)] => {
                let mut address_bytes = [0; 20];
                address_bytes.copy_from_slice(address);
                OutputType::Address(Address::from_bytes(AddressType::P2PKH, address_bytes))
            },
            &[Code(OpHash160), Push(ref address), Code(OpEqual)] => {
                let mut address_bytes = [0; 20];
                address_bytes.copy_from_slice(address);
                OutputType::Address(Address::from_bytes(AddressType::P2SH, address_bytes))
            },
            ops if ops.len() > 0 && ops[0] == Code(OpReturn) => OutputType::OpReturn,
            _ => OutputType::Unknown
        }
    }

    pub fn _process_slp_output(tx: &cashcontracts::Tx, db: &Db)
            -> Result<Option<(TxType, Vec<SLPAmount>, Token)>> {
        use cashcontracts::{Op::*, OpCodeType::*, serialize};
        let script = tx.outputs()[0].script();
        let script_hex = || hex::encode(script.to_vec());
        let ops = script.ops();
        match ops.get(1) {
            Some(Push(lokad_id)) if lokad_id == b"SLP\0" => {},
            _ => return Ok(None),
        }
        if ops.len() < 6 {
            return Err(ErrorKind::InvalidSLPOutput(script_hex(),
                                                   SLPError::TooFewPushops(ops.len())).into());
        }
        if !script.is_slp_safe() {
            return Err(ErrorKind::InvalidSLPOutput(script_hex(),
                                                   SLPError::NotSLPSafe).into());
        }
        match &ops {
            &[Code(OpReturn), Push(_), Push(token_type), Push(tx_type), Push(token_ticker),
              Push(token_name), Push(token_document_url), Push(token_document_hash), Push(decimals),
              Push(mint_baton_vout), Push(initial_token_mint_quantity)] if tx_type == b"GENESIS" => {
                let decimals = decimals[0];
                return Ok(Some((
                    TxType::SLP {
                        slp_type: SLPTxType::Genesis,
                        token_type: num::traits::FromPrimitive::from_u8(token_type[0]).unwrap(),
                        token_hash: tx.hash(),
                    },
                    vec![SLPAmount::new(
                        io::Cursor::new(&initial_token_mint_quantity).read_u64::<BigEndian>()? as i128,
                        decimals as u32,
                    )],
                    Self::_fetch_token(&tx.hash(), db)?,
                )))
            },
            _ => {},
        }
        match (&ops[0], &ops[1], &ops[2], &ops[3], &ops[4]) {
            (Code(OpReturn), Push(_), Push(token_type), Push(tx_type), Push(token_id))
                    if tx_type == b"SEND" => {
                if token_type.len() > 2 || token_type.len() == 0 {
                    return Err(ErrorKind::InvalidSLPOutput(
                        script_hex(),
                        SLPError::InvalidTokenTypeLength(hex::encode(token_type)),
                    ).into());
                }
                if token_id.len() != 32 {
                    return Err(ErrorKind::InvalidSLPOutput(
                        script_hex(),
                        SLPError::InvalidTokenHashLength(hex::encode(token_id))
                    ).into())
                }
                let mut token_hash = [0; 32];
                token_hash.copy_from_slice(&token_id.iter().rev().cloned().collect::<Vec<_>>());
                let token = Self::_fetch_token(&token_hash, db)?;
                let decimals = token.decimals as u32;
                let token_type = serialize::vec_to_int(token_type);
                let amounts = ops[5..].iter()
                    .map(|op| {
                        match op {
                            Push(vec) => Ok(SLPAmount::from_slice(&vec, decimals)?),
                            _ => unreachable!(),  // handled by is_slp_safe
                        }
                    })
                    .collect::<Result<Vec<_>>>()?;
                if amounts.len() > 19 {
                    return Err(ErrorKind::InvalidSLPOutput(
                        hex::encode(script.to_vec()),
                        SLPError::TooManyAmounts(amounts.len()),
                    ).into())
                }
                if let Some(token_type) = num::traits::FromPrimitive::from_u16(token_type as u16) {
                    Ok(Some((
                        TxType::SLP {
                            slp_type: SLPTxType::from_bytes(tx_type)
                                .ok_or_else(|| ErrorKind::InvalidSLPOutput(
                                    script_hex(),
                                    SLPError::InvalidSLPType(
                                        format!(
                                            "{} ({})",
                                            String::from_utf8_lossy(tx_type),
                                            hex::encode(tx_type),
                                        )
                                    )
                                ))?,
                            token_type,
                            token_hash,
                        },
                        amounts,
                        token,
                    )))
                } else {
                    Ok(None)
                }
            },
            _ => { Err(ErrorKind::InvalidSLPOutput(script_hex(), SLPError::NoMatch).into()) }
        }
    }

    pub fn _process_pnd1_tx(tx: &cashcontracts::Tx, db: &Db, config: &SLPDEXConfig) -> Result<Option<PND1Tx>> {
        use cashcontracts::{Op::*, OpCodeType::*, serialize};
        if tx.outputs().len() < 2 {
            return Ok(None);
        }
        let op_return_script = tx.outputs()[0].script();
        let ops = op_return_script.ops();
        match ops.get(1) {
            Some(Push(lokad_id)) if lokad_id == b"PND1" => {},
            _ => return Ok(None),
        }
        let fee_output = &tx.outputs()[1];
        if fee_output.value < config.panda_fee {
            return Err(ErrorKind::InvalidPND(PNDError::DoesntPayPandaFee).into())
        }
        if fee_output.script.ops() != (P2PKHOutput {
            value: 0,
            address: config.fee_address.clone(),
        }).script().ops() {
            return Err(ErrorKind::InvalidPND(PNDError::DoesntPayPandaFee).into())
        }
        match &ops {
            &[Code(OpReturn), Push(_), Push(name), Push(father_hash), Push(father_output_idx),
                   Push(mother_hash), Push(mother_output_idx), Push(pubkey), Push(signature)] => {
                if father_hash.len() != 32 {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::InvalidFatherHash(hex::encode(father_hash))
                    ).into());
                }
                if mother_hash.len() != 32 {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::InvalidMotherHash(hex::encode(mother_hash))
                    ).into());
                }
                let name = String::from_utf8(name.to_vec())
                    .map_err(|err| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidName(hex::encode(name), err)
                        ).into()
                    })?;
                let father_hash = tx_hash_from_le_slice(father_hash);
                let mother_hash = tx_hash_from_le_slice(mother_hash);
                if father_hash == mother_hash {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::FatherIsMother
                    ).into())
                }
                let father_output_idx = io::Cursor::new(father_output_idx).read_i32::<LittleEndian>()
                    .map_err(|_| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidFatherOutputIdx(hex::encode(father_output_idx))
                        ).into()
                    })?;
                let mother_output_idx = io::Cursor::new(mother_output_idx).read_i32::<LittleEndian>()
                    .map_err(|_| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidMotherOutputIdx(hex::encode(mother_output_idx))
                        ).into()
                    })?;
                let mut slp_txs = db.slp_txs(vec![father_hash.clone(), mother_hash.clone()].into_iter())?;
                let (father_tx, father_slp_tx, father_token) = slp_txs.remove(&father_hash)
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::FatherDoesntExist(hex::encode(father_hash))
                        ).into()
                    })?;
                let (mother_tx, mother_slp_tx, mother_token) = slp_txs.remove(&mother_hash)
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::MotherDoesntExist(hex::encode(mother_hash))
                        ).into()
                    })?;
                println!("{:?} {:?} {:?}", father_tx, father_slp_tx, father_token);
                if father_token.parent_token_hash != Some(config.panda_token_hash.to_vec()) {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::WrongFatherToken(
                            format!("{:?}", father_token.parent_token_hash
                                .map(|token| tx_hash_to_hex(&tx_hash_from_slice(&token))))
                        )
                    ).into())
                }
                if mother_token.parent_token_hash != Some(config.panda_token_hash.to_vec()) {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::WrongMotherToken(
                            format!("{:?}", mother_token.parent_token_hash
                                .map(|token| tx_hash_to_hex(&tx_hash_from_slice(&token))))
                        )
                    ).into())
                }
                let mut tx_outputs = db.tx_outputs(vec![father_hash.clone(), mother_hash.clone()].into_iter())?;
                let father_output = tx_outputs.remove(&(father_hash, father_output_idx))
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::FatherDoesntExist(format!("{}:{}", hex::encode(father_hash), father_output_idx))
                        ).into()
                    })?;
                let mother_output = tx_outputs.remove(&(mother_hash, mother_output_idx))
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::MotherDoesntExist(format!("{}:{}", hex::encode(mother_hash), mother_output_idx))
                        ).into()
                    })?;
                let father = get_panda_by_owner_utxo(father_tx.id, father_output_idx as i32, db.connection())?
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidFatherUTXO(format!("{}:{}", hex::encode(father_hash), father_output_idx))
                        ).into()
                    })?;
                let mother = get_panda_by_owner_utxo(mother_tx.id, mother_output_idx as i32, db.connection())?
                    .ok_or_else(|| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidFatherUTXO(format!("{}:{}", hex::encode(mother_hash), mother_output_idx))
                        ).into()
                    })?;
                let address = Address::from_serialized_pub_key("simpleledger", AddressType::P2PKH, &pubkey);
                if father_output.address != Some(address.bytes().to_vec()) {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::FatherNotOwnedByAddress(hex::encode(father_hash), address.cash_addr().to_string())
                    ).into());
                }
                if mother_output.address != Some(address.bytes().to_vec()) {
                    return Err(ErrorKind::InvalidPND(
                        PNDError::MotherNotOwnedByAddress(hex::encode(mother_hash), address.cash_addr().to_string())
                    ).into());
                }
                let pubkey_decoded = secp256k1::PublicKey::from_slice(pubkey)
                    .map_err(|_| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidPubkey(hex::encode(pubkey))
                        ).into()
                    })?;
                let signature_decoded = secp256k1::Signature::from_der(signature)
                    .map_err(|_| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::InvalidSignature(hex::encode(signature))
                        ).into()
                    })?;
                let message = format!("PANDA S3X:{}+{}",
                                      tx_hash_to_hex(&tx_hash_from_slice(&father_token.hash)),
                                      tx_hash_to_hex(&tx_hash_from_slice(&mother_token.hash)));
                let message_hash = secp256k1::Message::from_slice(&single_sha256(message.as_bytes())).unwrap();
                let curve = secp256k1::Secp256k1::new();
                curve.verify(&message_hash, &signature_decoded, &pubkey_decoded)
                    .map_err(|_| -> Error {
                        ErrorKind::InvalidPND(
                            PNDError::SignatureVerifyFail(message,
                                                          hex::encode(signature),
                                                          hex::encode(pubkey))
                        ).into()
                    })?;
                return Ok(Some(PND1Tx {
                    father_id: father.id,
                    mother_id: mother.id,
                    name,
                    father_tx_hash: tx_hash_from_slice(&father_tx.hash),
                    father_output_idx: father_output_idx as u32,
                    mother_tx_hash: tx_hash_from_slice(&mother_tx.hash),
                    mother_output_idx: mother_output_idx as u32,
                    public_key: pubkey.to_vec(),
                    signature: signature.to_vec(),
                }))
            },
            _ => {
                return Err(ErrorKind::InvalidPND(PNDError::InvalidNumberOfPushops(ops.len() - 1)).into());
            }
        }
        Ok(None)
    }

    pub fn from_txs(txs: &[cashcontracts::Tx], now: i64, config: &SLPDEXConfig, db: &Db) -> Self {
        let mut historic_txs = Vec::new();
        let mut trade_offers = HashMap::new();
        let mut pnd_txs = HashMap::new();
        for tx in txs.iter() {
            let inputs = tx.inputs().iter()
                .map(|input| {
                    HistoricTxInput {
                        output_tx: input.outpoint.tx_hash.clone(),
                        output_idx: input.outpoint.vout as i32,
                        output: Self::_process_input_script(&input.script),
                    }
                })
                .collect::<Vec<_>>();
            let (tx_type, slp_amounts, token) = tx.outputs()
                .get(0)
                .and_then(|output| {
                    match Self::_process_slp_output(tx, db) {
                        Ok(slp_output) => slp_output,
                        Err(err) => {
                            eprintln!("Invalid SLP output: {} in {}", err, tx_hash_to_hex(&tx.hash()));
                            None
                        },
                    }
                })
                .map(|(tx_type, slp_amounts, token)| {
                    (tx_type, slp_amounts, Some(token))
                })
                .unwrap_or((TxType::Default, vec![], None));
            let outputs = tx.outputs().iter().enumerate()
                .map(|(output_idx, output)| {
                    HistoricTxOutput {
                        value_satoshis: output.value,
                        value_token: if output_idx > 0 {
                            slp_amounts.get(output_idx - 1).cloned()
                        } else {
                            None
                        }.unwrap_or(SLPAmount::new(
                            0,
                            token.as_ref().map(|token| token.decimals as u32).unwrap_or(0),
                        )),
                        output: Self::_process_output_script(&output.script),
                    }
                })
                .chain(
                    slp_amounts.iter().skip(tx.outputs().len()).map(|amount| {
                        HistoricTxOutput {
                            value_satoshis: 0,
                            value_token: *amount,
                            output: OutputType::Burned,
                        }
                    })
                )
                .collect::<Vec<_>>();
            let historic_tx = HistoricTx {
                hash: tx.hash(),
                height: None,
                timestamp: now,
                tx_type,
                inputs,
                outputs,
            };
            let trade_offer = match &historic_tx.tx_type {
                TxType::SLP { .. } => token.and_then(
                    |token| TradeOffer::from_tx(&historic_tx, tx, config, &token)
                ),
                _ => None,
            };
            match Self::_process_pnd1_tx(tx, db, config) {
                Ok(Some(pnd)) => {pnd_txs.insert(historic_txs.len(), pnd);},
                Ok(None) => {},
                Err(err) => eprintln!("PND error: {}", err),
            };
            if let Some(trade_offer) = trade_offer {
                trade_offers.insert(historic_txs.len(), trade_offer);
            }
            historic_txs.push(historic_tx);
        }
        TxHistory {
            txs: historic_txs,
            trade_offers,
            pnd_txs,
        }
    }

    pub fn _fetch_token(token_hash: &[u8; 32], db: &Db) -> Result<Token> {
        match db.token(token_hash)? {
            Some(token) => Ok(token),
            None => {
                return Err(   // CryptoPandas only cares about known tokens
                    ErrorKind::TokenError(
                        TokenError::UnknownTokenId(tx_hash_to_hex(token_hash))
                    ).into()
                );
                let mut token_entries = crate::token_source::TokenSource::new()
                    .request_tokens(&[TxFilter::TokenId(token_hash.clone())])?;
                println!("token entry: {:?}", token_entries);
                if token_entries.len() == 0 {
                    return Err(
                        ErrorKind::TokenError(
                            TokenError::UnknownTokenId(tx_hash_to_hex(token_hash))
                        ).into()
                    )
                }
                let token = Token::from_entry(token_entries.remove(0))?;
                println!("new token: {:?}", token);
                db.add_tokens(&[token.clone()])?;
                Ok(token)
            },
        }
    }

    pub fn validate_slp(&mut self, tx_source: &TxSource, db: &Db, config: &SLPDEXConfig)
                        -> Result<()> {
        let tx_to_check = self.txs.iter()
            .flat_map(|tx| {
                tx.inputs.iter()
                    .map(|input| input.output_tx)
                    .take(match tx.tx_type {
                        TxType::SLP {..} => tx.inputs.len(),
                        TxType::Default => 0,
                    })
            })
            .collect::<HashSet<_>>();
        if tx_to_check.len() == 0 { return Ok(()); }
        let tx_outputs = db.tx_outputs(tx_to_check.iter().cloned())?;
        let slp_txs = db.slp_txs(tx_to_check.iter().cloned())?;
        for i in 0..self.txs.len() {
            let tx = &mut self.txs[i];
            let (token_hash, token_type) = match tx.tx_type {
                TxType::SLP {ref token_hash, token_type, ..} => (token_hash, token_type),
                TxType::Default => continue,
            };
            println!("validating {}", cashcontracts::tx_hash_to_hex(&tx.hash));
            println!("token found: ");
            let output_sum = tx.outputs.iter()
                .map(|output| output.value_token)
                .sum::<SLPAmount>();
            let input_sum = tx.inputs.iter()
                .filter_map(|input| {
                    let (tx, slp_tx, token) = slp_txs.get(&input.output_tx)?;
                    Some((
                        input,
                        tx,
                        slp_tx,
                        tx_outputs.get(&(input.output_tx.clone(), input.output_idx))?,
                        token,
                    ))
                })
                .filter(|(input, tx, slp_tx, output, token)| {
                    input.output_idx > 0 &&
                        &token.hash == token_hash &&
                        slp_tx.version == token_type as i32
                })
                .map(|(input, tx, slp_tx, output, token)|
                    SLPAmount::from_numeric_decimals(&output.value_token_base, token.decimals as u32)
                )
                .sum::<SLPAmount>();
            println!("input sum: {}", input_sum);
            println!("output sum: {}", output_sum);
            if input_sum < output_sum {
                tx.tx_type = TxType::Default;
                tx.outputs.iter_mut().for_each(|output| {
                    output.value_token = SLPAmount::new(0, 0);
                });
                self.trade_offers.remove(&i);
            }
        }
        Ok(())
    }

    pub fn validate_slp_remote(&mut self, tx_source: &TxSource, db: &Db, config: &SLPDEXConfig)
            -> Result<()> {
        let tx_to_check = self.txs.iter()
            .flat_map(|tx| {
                tx.inputs.iter()
                    .map(|input| input.output_tx)
                    .take(match tx.tx_type {
                        TxType::SLP {..} => tx.inputs.len(),
                        TxType::Default => 0,
                    })
            })
            .collect::<HashSet<_>>();
        if tx_to_check.len() == 0 { return Ok(()); }
        let tx_to_check = tx_to_check.into_iter()
            .map(TxFilter::TxHash)
            .collect::<Vec<_>>();
        let validity_map = tx_source
            .request_slp_tx_validity(&tx_to_check, config, Confirmedness::Both)?
            .into_iter()
            .map(|validity| (cashcontracts::tx_hex_to_hash(&validity.tx.h).unwrap(), validity))
            .collect::<HashMap<_, _>>();
        for i in 0..self.txs.len() {
            let tx = &mut self.txs[i];
            let (token_hash, token_type) = match &tx.tx_type {
                TxType::SLP {token_hash, token_type, ..} => (token_hash, token_type),
                TxType::Default => continue,
            };
            println!("validating {}", cashcontracts::tx_hash_to_hex(&tx.hash));
            println!("token found: ");
            let decimals = tx.outputs.iter()
                .map(|output| output.value_token.decimals())
                .next();
            let output_sum = tx.outputs.iter()
                .map(|output| output.value_token)
                .sum::<SLPAmount>();
            let input_sum = tx.inputs.iter()
                .filter_map(|input| Some((input, validity_map.get(&input.output_tx)?)))
                .filter(|(tx_input, validity)|
                    validity.slp.valid &&
                        tx_input.output_idx > 0 &&
                        tx_hex_to_hash(&validity.slp.detail.token_id).as_ref() == Some(token_hash) &&
                        validity.slp.detail.version_type == *token_type as i32
                )
                .filter_map(|(tx_input, validity)|
                    validity.slp.detail.outputs.get((tx_input.output_idx - 1) as usize)
                )
                .filter_map(|slp_output: &tx_result::TxSLPOutput| {
                    Some(SLPAmount::from_str_decimals(&slp_output.amount, decimals?).ok()?)
                })
                .sum::<SLPAmount>();
            println!("input sum: {}", input_sum);
            println!("output sum: {}", output_sum);
            if input_sum < output_sum {
                tx.tx_type = TxType::Default;
                tx.outputs.iter_mut().for_each(|output| {
                    output.value_token = SLPAmount::new(0, 0);
                });
                self.trade_offers.remove(&i);
            }
        }
        Ok(())
    }
}

struct _Price {
    script_price: u32,
    price_per_token: Rational,
    power: u8,
    is_inverted: bool,
}

impl TradeOffer {
    const _FACTORS: [u64; 10] = [
        1,
        10,
        100,
        1_000,
        10_000,
        100_000,
        1_000_000,
        10_000_000,
        100_000_000,
        1_000_000_000,
    ];

    fn _decode_price(slp_decimals: i32, power_bytes: &[u8], price_bytes: &[u8]) -> Result<_Price> {
        let is_inverted = power_bytes.get(1) == Some(&1);
        let script_price = io::Cursor::new(price_bytes)
            .read_u32::<BigEndian>()
            .map_err(|_| Error::from(ErrorKind::InvalidTradeOffer(
                TradeOfferError::InvalidPrice(price_bytes.to_vec())
            )))?;
        let factor = Self::_FACTORS[slp_decimals as usize];
        let factor_rational = rug::Rational::from((factor, 1));
        let price_per_token = if is_inverted {
            if script_price == 0 {
                return Err(ErrorKind::InvalidTradeOffer(
                    TradeOfferError::InvalidPrice(price_bytes.to_vec())
                ).into())
            }
            rug::Rational::from((1, script_price)) * factor_rational
        } else {
            rug::Rational::from((script_price, 1)) * factor_rational
        };
        Ok(_Price {
            script_price,
            price_per_token,
            power: *power_bytes.get(0).ok_or_else(|| ErrorKind::InvalidTradeOffer(
                TradeOfferError::InvalidPower(power_bytes.to_vec()),
            ))?,
            is_inverted,
        })
    }

    fn _contract_hash(output: &HistoricTxOutput,
                      price: &_Price,
                      tx_type: &TxType,
                      config: &SLPDEXConfig,
                      receiving_address: &cashcontracts::Address) -> Option<SLPAmount> {
        let (token_hash, token_type) = match tx_type {
            TxType::SLP {token_hash, token_type, ..} => (token_hash, *token_type),
            TxType::Default => return None,
        };
        let address = output.output.address()?;
        if address.addr_type() == AddressType::P2SH {
            /*let hash = cashcontracts::hash160(
                &cashcontracts::AdvancedTradeOffer {
                    value: output.value_satoshis,
                    lokad_id: b"EXCH".to_vec(),
                    version: 2,
                    power: price.power,
                    is_inverted: price.is_inverted,
                    token_id: token_hash.clone(),
                    token_type: token_type as u8,
                    sell_amount_token: output.value_token.base_amount() as u64,
                    price: price.script_price,
                    dust_amount: config.dust_limit,
                    address: receiving_address.clone(),
                    fee_address: Some(config.fee_address.clone()),
                    fee_divisor: Some(config.fee_divisor.clone()),
                    spend_params: None,
                }.script().to_vec()
            );
            if address.bytes() == &hash {
                Some(output.value_token)
            } else {
                None
            }*/
            None
        } else {
            None
        }
    }

    pub fn from_entry(tx: &HistoricTx,
                      entry: &tx_result::TxEntry,
                      config: &SLPDEXConfig,
                      decimals: u32)
            -> Option<Self> {
        entry.inputs.iter().find_map(|input| {
            if input.b0 == tx_result::StackItem::Str(base64::encode("EXCH")) &&
                    input.b1 == (tx_result::StackItem::Op {op: 0x52}) {
                let price = entry.slp.as_ref()
                    .and_then(|slp| {
                        Self::_decode_price(
                            slp.detail.decimals,
                            &base64::decode(input.b2.get_str()?).ok()?,
                            &base64::decode(input.b3.get_str()?).ok()?,
                        ).map_err(|err| {
                            eprintln!("Trade offer error {}", err);
                        }).ok()
                    })?;
                let receiving_address = Address::from_slice(
                    AddressType::P2PKH,
                    &base64::decode(input.b4.get_str()?).ok()?,
                )?;
                let output_idx: i32 = 1;
                let contract_vals = tx.outputs.get(output_idx as usize)
                    .and_then(|output: &HistoricTxOutput| {
                        Some((
                            output_idx,
                            Self::_contract_hash(output,
                                                 &price,
                                                 &tx.tx_type,
                                                 config,
                                                 &receiving_address)?,
                        ))
                    });
                Some(TradeOffer {
                    tx: cashcontracts::tx_hex_to_hash(&entry.tx.h).unwrap(),
                    output_idx: contract_vals.map(|(idx, _)| idx),
                    input_tx: cashcontracts::tx_hex_to_hash(&input.e.h).unwrap(),
                    input_idx: input.e.i,
                    price_per_token: price.price_per_token,
                    is_inverted: price.is_inverted,
                    script_price: price.script_price as i64,
                    sell_amount_token: contract_vals
                        .map(|(_, amount)| amount)
                        .unwrap_or(SLPAmount::new(0, decimals)),
                    receiving_address,
                })
            } else {
                None
            }
        })
    }

    pub fn from_tx(historic_tx: &HistoricTx,
                   tx: &cashcontracts::Tx,
                   config: &SLPDEXConfig,
                   token: &Token) -> Option<Self> {
        use cashcontracts::{Op::*, OpCodeType::*};
        println!("validating trade offer {}", tx_hash_to_hex(&historic_tx.hash));
        if let TxType::Default = &historic_tx.tx_type {
            return None
        }
        tx.inputs().iter().find_map(|input| {
            let ops = input.script.ops();
            if ops.len() < 5 { return None; }
            match &input.script.ops()[..5] {
                &[Push(ref exch), Code(Op2), Push(ref power), Push(ref price), Push(ref address)]
                        if exch.as_slice() == config.exch_lokad.as_bytes() => {
                    let price = Self::_decode_price(token.decimals, power, price)
                        .map_err(|err| {
                            eprintln!("Trade offer error {}", err);
                        }).ok()?;
                    println!("succeed price decoding");
                    let receiving_address = Address::from_slice(
                        AddressType::P2PKH,
                        address,
                    )?;
                    println!("succeed address decoding");
                    let output_idx: i32 = 1;
                    let contract_vals = historic_tx.outputs.get(output_idx as usize)
                        .and_then(|output: &HistoricTxOutput| {
                            Some((
                                output_idx,
                                Self::_contract_hash(output,
                                                     &price,
                                                     &historic_tx.tx_type,
                                                     config,
                                                     &receiving_address)?,
                            ))
                        });
                    println!("contract vals {:?}", contract_vals);
                    Some(TradeOffer {
                        tx: historic_tx.hash.clone(),
                        output_idx: contract_vals.map(|(idx, _)| idx),
                        input_tx: input.outpoint.tx_hash.clone(),
                        input_idx: input.outpoint.vout as i32,
                        price_per_token: price.price_per_token,
                        is_inverted: price.is_inverted,
                        script_price: price.script_price as i64,
                        sell_amount_token: contract_vals
                            .map(|(_, amount)| amount)
                            .unwrap_or(SLPAmount::new(0, token.decimals as u32)),
                        receiving_address,
                    })
                }
                _ => { println!("bad stack {}", input.script); None }
            }
        })
    }
}

impl std::fmt::Display for HistoricTx {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "hash: {}", tx_hash_to_hex(&self.hash))?;
        writeln!(f, "height: {:?}", self.height)?;
        writeln!(f, "timestamp: {}", self.timestamp)?;
        writeln!(f, "tx_type: {:?}", self.tx_type)?;
        writeln!(f, "inputs:")?;
        for (i, input) in self.inputs.iter().enumerate() {
            writeln!(f, "{}:\n{}", i, input)?;
        }
        writeln!(f, "outputs:")?;
        for (i, output) in self.outputs.iter().enumerate() {
            writeln!(f, "{}:\n{}", i, output)?;
        }
        Ok(())
    }
}

impl std::fmt::Display for HistoricTxInput {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "  output_tx: {}", tx_hash_to_hex(&self.output_tx))?;
        writeln!(f, "  output_idx: {:?}", self.output_idx)?;
        writeln!(f, "  output: {}", self.output)?;
        Ok(())
    }
}

impl std::fmt::Display for HistoricTxOutput {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error> {
        writeln!(f, "  value_satoshis: {}", self.value_satoshis)?;
        writeln!(f, "  value_token: {}", self.value_token)?;
        writeln!(f, "  output: {}", self.output)?;
        Ok(())
    }
}

impl std::fmt::Display for OutputType {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> std::result::Result<(), std::fmt::Error> {
        match self {
            OutputType::Address(address) => write!(f, "Address({})", address.cash_addr())?,
            _ => write!(f, "{:?}", self)?,
        }
        Ok(())
    }
}
