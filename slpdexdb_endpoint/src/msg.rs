use actix::prelude::*;
use cashcontracts::Address;
use slpdexdb_base::Error;
use std::net;
use slpdexdb_base::{SLPDEXConfig, BlockHeader};
use slpdexdb_db::{Db, Utxo, SpentUtxo, TxDelta, TradeOfferFilter, TradeOffer, TxHistory};
use slpdexdb_node::actors::{OutgoingMsg};
use std::collections::{HashSet, HashMap};
use std::sync::{Arc, Mutex};
use crate::actors::TxSubscribers;


pub struct ConnectToPeer {
    pub socket_addr: net::SocketAddr,
}

impl Message for ConnectToPeer {
    type Result = Result<(), Error>;
}

pub struct ActivateAddress(pub Address);

impl Message for ActivateAddress {
    type Result = Result<(), Error>;
}

pub struct DeactivateAddress(pub Address);

impl Message for DeactivateAddress {
    type Result = Result<(), Error>;
}

pub struct ResyncAddress(pub Address);

impl Message for ResyncAddress {
    type Result = Result<(), Error>;
}

pub struct FetchTradeOfferUtxos(pub TradeOfferFilter);

impl Message for FetchTradeOfferUtxos {
    type Result = Result<Vec<TradeOffer>, Error>;
}

pub struct FetchAddressUtxos(pub Address);

impl Message for FetchAddressUtxos {
    type Result = Result<Vec<Utxo>, Error>;
}

pub struct FetchAddressTxDeltas(pub Address);

impl Message for FetchAddressTxDeltas {
    type Result = Result<Vec<TxDelta>, Error>;
}

pub enum SubscribeToEvent {
    Address(Address, Recipient<TxEvent>),
    Tokens(Vec<[u8; 32]>, Recipient<TxEvent>),
}

impl Message for SubscribeToEvent {
    type Result = ();
}

pub enum UnsubscribeFromEvent {
    Address(Address, Recipient<TxEvent>),
}

impl Message for UnsubscribeFromEvent {
    type Result = ();
}

#[derive(Clone)]
pub enum TxEvent {
    AddressUtxoDelta {
        add_utxos: Arc<Vec<Utxo>>,
        remove_utxos: Arc<Vec<SpentUtxo>>,
    },
    TradeOfferUtxoDelta {
        token_hash: [u8; 32],
        add_utxos: Arc<Vec<TradeOffer>>,
        remove_utxos: Arc<Vec<SpentUtxo>>,
    },
    AddressNewTxDeltas {
        tx_deltas: Arc<Vec<TxDelta>>,
    },
}

impl Message for TxEvent {
    type Result = Result<(), Error>;
}

type SyncTxSubscribers = Arc<Mutex<TxSubscribers>>;

pub enum TxBroadcastEvent {
    AddressUtxoDelta {
        add_utxos: HashMap<Address, Vec<Utxo>>,
        remove_utxos: HashMap<Address, Vec<SpentUtxo>>,
        subscribers: SyncTxSubscribers,
    },
    TradeOfferUtxoDelta {
        add_utxos: HashMap<[u8; 32], Vec<TradeOffer>>,
        remove_utxos: HashMap<[u8; 32], Vec<SpentUtxo>>,
        subscribers: SyncTxSubscribers,
    },
    AddressNewTxDeltas {
        tx_deltas: HashMap<Address, Vec<TxDelta>>,
        subscribers: SyncTxSubscribers,
    },
}

impl Message for TxBroadcastEvent {
    type Result = Result<(), Error>;
}

#[derive(Clone)]
pub struct NewTransactions {
    pub now: i64,
    pub db: Arc<Mutex<Db>>,
    pub tx_history: Arc<TxHistory>,
    pub relevant_addresses: Arc<HashSet<Address>>,
    pub subscribers: SyncTxSubscribers,
}

impl Message for NewTransactions {
    type Result = Result<(), Error>;
}

pub struct ProcessTransactions {
    pub txs: Vec<cashcontracts::Tx>,
    pub db: Arc<Mutex<Db>>,
    pub config: SLPDEXConfig,
    pub subscribers: Arc<Mutex<TxSubscribers>>,
    pub broadcasts: Vec<Recipient<NewTransactions>>,
}

impl Message for ProcessTransactions {
    type Result = Result<(), Error>;
}

pub struct ProcessBlock {
    pub header: BlockHeader,
    pub tx_hashes: Vec<[u8; 32]>,
    pub db: Arc<Mutex<Db>>,
    pub config: SLPDEXConfig,
    pub subscribers: Arc<Mutex<TxSubscribers>>,
    pub broadcasts: Vec<Recipient<NewTransactions>>,
}

impl Message for ProcessBlock {
    type Result = Result<(), Error>;
}

pub struct RegisterOutgoing {
    pub recipient: Recipient<OutgoingMsg>,
}

impl Message for RegisterOutgoing {
    type Result = ();
}
