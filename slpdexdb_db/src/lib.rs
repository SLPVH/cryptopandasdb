#[macro_use]
extern crate diesel;
#[macro_use]
extern crate num_derive;

mod db;
pub mod models;
pub mod schema;
mod endpoint;
mod token;
mod token_source;
mod tx_source;
mod tx_history;
mod update_history;
mod convert;
mod data;
pub mod panda_tools;
pub mod panda;
pub mod fan_out;

pub use db::*;
pub use endpoint::*;
pub use token::*;
pub use token_source::*;
pub use tx_source::*;
pub use tx_history::*;
pub use update_history::*;
pub use data::*;

//use slpdexdb_base::Result;

/*fn main() -> Result<()> {
    //let fast = fast_utxo::FastUtxoSet::new("/Users/tobiasruck/workspace/bitcoin/slpdex-backend/data/QmXkBQJrMKkCKNbwv4m5xtnqwU9Sq7kucPigvZW8mWxcrv")?;
    //for utxo in fast.take(10) {
    //    let utxo = utxo?;
    //    println!("{}", utxo);
    //}
    use diesel::prelude::*;
    use diesel::pg::PgConnection;

    let connection_str = std::env::var("DATABASE_STR")?;

    let address = cashcontracts::Address::from_cash_addr(
        "bitcoincash:qq5lzj2p3kznpdsm06ms7la9g6d8hezkkg4mgq9rdh".to_string()
    ).unwrap();

    let db_conn = PgConnection::establish(&connection_str)?;

    for s in &["10", "2.00101", "30020000.010", "400.100", "500.0100", "60.0", "701.100"] {
        println!("{}", s);
        let amount = slpdexdb_base::SLPAmount::from_str_decimals(s, 5).unwrap();
        println!("{}", amount);
        println!("_------");
    }

    return Ok(());
    let db = db::Db::new(db_conn);
    /*let subject_type = update_history::UpdateSubjectType::AddressHistory;

    db.update_utxo_set(&address)?;
    db.update_utxo_set_exch()?;

    return Ok(());
    /*

    let tokens = token_source::TokenSource::new()
        .request_tokens(
            &db.last_update(subject_type)?
                .unwrap_or(update_history::UpdateHistory::initial(subject_type, None))
                .next_filters()
        )?
        .into_iter()
        .filter_map(|entry| token::Token::from_entries(entry))
        .collect::<Vec<_>>();

    println!("{:?}", tokens);

    db.add_tokens(&tokens)?;
    db.add_update_history(&update_history::UpdateHistory::from_tokens(&tokens, height))?;

    return Ok(());*/
    let height = db.header_tip()?.unwrap().1;

    let config = config::SLPDEXConfig::default();
    let subject_hash = Some(address.bytes().to_vec());
    let last_update = db.last_update(subject_type)?
        .unwrap_or(update_history::UpdateHistory::initial(subject_type, subject_hash.clone()));
    let entries = tx_source::TxSource::new().request_txs(
        &last_update.next_filters(),
        &config,

    )?;

    use std::time::{SystemTime, UNIX_EPOCH};
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let history = tx_history::TxHistory::from_entries(&entries, timestamp as i64, &config);
    for tx in history.txs.iter() {
        println!("txhash={}", cashcontracts::tx_hash_to_hex(&tx.hash));
    }
    db.add_tx_history(&history)?;
    db.add_update_history(&update_history::UpdateHistory::from_tx_history(
        &history,
        subject_type,
        subject_hash.clone(),
        height,
    ))?;

    return Ok(());*/
    /*let mut node = node::Node::new(db);
    node.connect("100.1.209.114:8333")?;
    node.run_forever()?;
    Ok(())*/
}
*/