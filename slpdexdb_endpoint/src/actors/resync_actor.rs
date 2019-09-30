use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;
use std::collections::HashSet;
use actix::prelude::*;
use cashcontracts::{Address, tx_hex_to_hash};
use slpdexdb_base::{Error, SLPDEXConfig};
use slpdexdb_db::tx_hash_from_slice;
use slpdexdb_db::{Db, TxSource, TokenSource, UpdateSubject, UpdateSubjectType, UpdateHistory,
                  TxHistory, TxFilter, Token, OutputType, Confirmedness, TxType, panda_tools};
use crate::msg::{ResyncAddress, ProcessTransactions, NewTransactions, ProcessBlock};
use cryptopandas_base::genomics::{create_seed, mix_genes};
use std::collections::HashMap;


fn _resync(db: &Db, config: &SLPDEXConfig) -> Result<(), Error> {
    _init_panda_token(db, config)?;
    //_resync_tokens(db)?;
    //_resync_trade_offers(db, config, true)?;
    //_resync_trade_offers(db, config, false)?;
    Ok(())
}

fn _init_panda_token(db: &Db, config: &SLPDEXConfig) -> Result<(), Error> {
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let token_source = TokenSource::new();
    let tx_source = TxSource::new();
    let token_hash = tx_hex_to_hash("16668131b2563dd32ef7b098056fd696b010233f66bcab5cc22bdf2b2a60f294").unwrap();
    let token_entries = token_source.request_tokens(&[TxFilter::TokenId(token_hash.clone())])?;
    let tokens = token_entries.into_iter()
        .filter_map(|token_entry| {
            Token::from_entry(token_entry).map_err(|err| eprintln!("token error: {}", err)).ok()
        })
        .collect::<Vec<_>>();
    let tx_entries = tx_source.request_txs(&[TxFilter::TxHash(token_hash)], config, Confirmedness::Confirmed)?;
    let history = TxHistory::from_entries(&tx_entries, timestamp as i64, config);
    db.add_tokens(&tokens)?;
    db.add_tx_history(&history)?;
    Ok(())
}

fn _resync_tokens(db: &Db) -> Result<(), Error> {
    let token_source = TokenSource::new();
    loop {
        let current_height = db.header_tip()?.map(|(_, height)| height).unwrap_or(0);
        let subject = UpdateSubject {
            subject_type: UpdateSubjectType::Token,
            hash: None,
            is_confirmed: true,
        };
        let last_update = db.last_update(subject.clone())?
            .unwrap_or(UpdateHistory::initial(subject));
        println!("last update: {:?}", last_update);
        let token_entries = token_source.request_tokens(&last_update.next_filters())?;
        let tokens = token_entries.into_iter()
            .filter_map(|token_entry| {
                Token::from_entry(token_entry).map_err(|err| eprintln!("token error: {}", err)).ok()
            })
            .collect::<Vec<_>>();
        if tokens.len() == 0 {
            db.add_update_history(&UpdateHistory::from_tokens(&tokens, current_height))?;
            break
        }
        for token in tokens.iter() {
            println!("try adding token {:?}", token);
            println!("document_uri: {:?}", token.document_uri.as_ref().map(|x| hex::encode(x.as_bytes())));
            db.add_tokens(&[token.clone()])?;
        }
        db.add_update_history(&UpdateHistory::from_tokens(&tokens, current_height))?;
    }
    Ok(())
}

fn _resync_trade_offers(db: &Db, config: &SLPDEXConfig, is_confirmed: bool) -> Result<(), Error> {
    let tx_source = TxSource::new();
    loop {
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let current_height = db.header_tip()?.map(|(_, height)| height).unwrap_or(0);
        let confirmedness = if is_confirmed { Confirmedness::Confirmed }
                            else { Confirmedness::Unconfirmed };
        let subject = UpdateSubject {
            subject_type: UpdateSubjectType::Exch,
            hash: None,
            is_confirmed,
        };
        let last_update = db.last_update(subject.clone())?
                .unwrap_or_else(|| UpdateHistory::initial(subject.clone()));
        let tx_entries = tx_source.request_txs(&last_update.next_filters(), config, confirmedness)?;
        let history = TxHistory::from_entries(&tx_entries, timestamp as i64, config);
        if history.txs.len() == 0 {
            break
        }
        db.add_tx_history(&history)?;
        db.add_update_history(
            &UpdateHistory::from_tx_history(&history, subject, current_height)
        )?;
    }
    db.update_utxo_set_exch()?;
    Ok(())
}

fn _resync_address(db: &Db, config: &SLPDEXConfig, address: &Address, is_confirmed: bool) -> Result<(), Error> {
    loop {
        let tx_source = TxSource::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let current_height = db.header_tip()?.map(|(_, height)| height).unwrap_or(0);
        let confirmedness = if is_confirmed { Confirmedness::Confirmed }
                            else { Confirmedness::Unconfirmed };
        let subject = UpdateSubject {
            subject_type: UpdateSubjectType::AddressHistory,
            hash: Some(address.bytes().to_vec()),
            is_confirmed,
        };
        let last_update = db.last_update(subject.clone())?
            .unwrap_or(UpdateHistory::initial(subject.clone()));
        println!("last update: {}", last_update);
        let tx_entries = tx_source.request_txs(&last_update.next_filters(), config, confirmedness)?;
        let history = TxHistory::from_entries(&tx_entries, timestamp as i64, config);
        if history.txs.len() > 0 {
            db.add_tx_history(&history)?;
        }
        db.add_update_history(
            &UpdateHistory::from_tx_history(
                &history,
                subject,
                current_height,
            )
        )?;
        if history.txs.len() == 0 {
            break
        }
    }
    db.update_utxo_set(address)?;
    Ok(())
}

pub struct ResyncActor {
    db: Db,
    config: SLPDEXConfig,
    secret: Vec<u8>,
}

impl ResyncActor {
    pub fn new(db: Db, config: SLPDEXConfig, secret: Vec<u8>) -> Self {
        ResyncActor { db, config, secret }
    }
}

impl Actor for ResyncActor {
    type Context = SyncContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        _resync(&self.db, &self.config)
            .map_err(|err| eprintln!("resync failed: {}", err))
            .unwrap_or(());
    }
}

impl Handler<ResyncAddress> for ResyncActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: ResyncAddress, _ctx: &mut Self::Context) -> Self::Result {
        let address = msg.0;
        _resync_address(&self.db, &self.config, &address, true)?;
        _resync_address(&self.db, &self.config, &address, false)?;
        Ok(())
    }
}

impl Handler<ProcessTransactions> for ResyncActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: ProcessTransactions, _ctx: &mut Self::Context) -> Self::Result {
        let tx_source = TxSource::new();
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
        let db = msg.db.lock().unwrap();
        let mut history = TxHistory::from_txs(&msg.txs, timestamp, &msg.config, &*db);
        let addresses = history.txs.iter()
            .flat_map(|tx| {
                tx.outputs.iter()
                    .map(|output| output.output.clone())
                    .chain(tx.inputs.iter().map(|input| input.output.clone()))
                    .filter_map(|output| match output {
                        OutputType::Address(address) => Some(address),
                        _ => None,
                    })
            })
            .collect::<Vec<_>>();
        let subscribers_addresses = &msg.subscribers.lock().unwrap().subscribers_address;
        let relevant_addresses = addresses.into_iter()
            .filter(|address| subscribers_addresses.contains_key(address))
            .collect::<HashSet<_>>();
        if history.txs.iter().filter(|tx| match tx.tx_type {
                TxType::SLP {..} => true,
                TxType::Default => false,
            }).count() == 0 &&
            relevant_addresses.len() == 0 {
            return Ok(())
        }
        history.validate_slp(&tx_source, &*db, &msg.config)?;
        if history.txs.iter().filter(|tx| match tx.tx_type {
            TxType::SLP {..} => true,
            TxType::Default => false,
        }).count() == 0 &&
            relevant_addresses.len() == 0 {
            return Ok(())
        }
        db.add_tx_history(&history)?;
        for tx in history.txs.iter() {
            println!("{}", tx);
        }
        println!("txs valid.");
        let new_transactions = NewTransactions {
            now: timestamp,
            subscribers: msg.subscribers.clone(),
            tx_history: Arc::new(history),
            db: msg.db.clone(),
            relevant_addresses: Arc::new(relevant_addresses),
        };
        for broadcast in msg.broadcasts.iter() {
            broadcast.do_send(new_transactions.clone()).unwrap();  // TODO: handle error
        }
        Ok(())
    }
}

impl Handler<ProcessBlock> for ResyncActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: ProcessBlock, _ctx: &mut Self::Context) -> Self::Result {
        let db = msg.db.lock().unwrap();
        let tx_set = msg.tx_hashes.into_iter().collect::<HashSet<_>>();
        let pending_pnd = db.pending_pnd()?;
        let mut born_pnds = Vec::new();
        for (pnd, tx) in pending_pnd {
            let hash = tx_hash_from_slice(&tx.hash);
            if tx_set.contains(&hash) {
                born_pnds.push((pnd, tx, hash));
            }
        }
        let block_hash = msg.header.hash();
        let pandas = panda_tools::get_pandas_by_ids(
            born_pnds.iter()
                .flat_map(|(pnd, _, _)| vec![pnd.father, pnd.mother])
                .collect::<Vec<_>>(),
            db.connection(),
        )?;
        let pandas = pandas.into_iter()
            .map(|panda| (panda.id, panda))
            .collect::<HashMap<_, _>>();
        for (pnd, tx, tx_hash) in born_pnds {
            let seed = create_seed(&block_hash, &tx_hash);
            let father = &pandas[&pnd.father];
            let mother = &pandas[&pnd.mother];
            let mut father_genes = [0; 48];
            father_genes.copy_from_slice(&father.genes);
            let mut mother_genes = [0; 48];
            mother_genes.copy_from_slice(&mother.genes);
            let new_genes = mix_genes(father_genes, mother_genes, seed);


        }
        Ok(())
    }
}
