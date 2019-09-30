use actix::prelude::*;
use std::collections::{HashSet, HashMap};
use std::convert::identity;
use slpdexdb_base::{Error, SLPDEXConfig};
use slpdexdb_db::{Db, Utxo, TxDelta, TradeOffer};
use slpdexdb_node::actors::IncomingMsg;
use slpdexdb_node::messages::{TxMessage, BlockMessage};
use crate::msg::{ActivateAddress, DeactivateAddress, ResyncAddress, FetchAddressUtxos,
                 FetchAddressTxDeltas, FetchTradeOfferUtxos, SubscribeToEvent, UnsubscribeFromEvent,
                 TxEvent, NewTransactions, ProcessTransactions, ProcessBlock};
use crate::actors::ResyncActor;
use crate::actors::broadcast_actor::{UpdateDbUtxosActor, BroadcastAddressUtxosActor,
                                     BroadcastTradeOfferUtxosActor, BroadcastTxHistoryActor,
                                     BroadcastActor};
use slpdexdb_node::NodeMessage;

use cashcontracts::Address;
use std::sync::{Mutex, Arc};

pub struct TxSubscribers {
    pub subscribers_address: HashMap<Address, HashSet<Recipient<TxEvent>>>,
    pub subscribers_token: HashMap<[u8; 32], HashSet<Recipient<TxEvent>>>,
}

pub struct TxActor {
    db: Arc<Mutex<Db>>,
    config: SLPDEXConfig,
    resync: Addr<ResyncActor>,
    subscribers: Arc<Mutex<TxSubscribers>>,
    broadcasts: Vec<Recipient<NewTransactions>>,
}

impl TxActor {
    pub fn start_with(db: Arc<Mutex<Db>>,
                      config: SLPDEXConfig,
                      resync: Addr<ResyncActor>) -> Addr<Self> {
        let broadcast = BroadcastActor::start(BroadcastActor);
        let broadcasts = vec![
            UpdateDbUtxosActor::start(UpdateDbUtxosActor).recipient(),
            BroadcastAddressUtxosActor::start(BroadcastAddressUtxosActor::new(broadcast.clone())).recipient(),
            BroadcastTradeOfferUtxosActor::start(BroadcastTradeOfferUtxosActor::new(broadcast.clone())).recipient(),
            BroadcastTxHistoryActor::start(BroadcastTxHistoryActor::new(broadcast.clone())).recipient(),
        ];
        Self::start(TxActor {
            db, config, resync,
            subscribers: Arc::new(Mutex::new(TxSubscribers {
                subscribers_address: HashMap::new(),
                subscribers_token: HashMap::new(),
            })),
            broadcasts,
        })
    }
}

impl Actor for TxActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
//        let tx_bytes = hex::decode("0100000002f7cf2ac976eb7ff1435cebe7f634f15d0e91e8afa227741106f72a4f2a963d92010000006a473044022014f382515b206c87313fa43b7a744a73adc62c6bf38983d6aa3f7c2b4e49821202200bc1c6d73e4462ac800daa39702098075e497fe4804991a48906e10c87b4354e4121031162a9a8f307b8e1efcafe3ce76b35ad293fad97ec885bcb8dbc6756d18ca941ffffffff1ca7052b7bf8e941aeeaaa3c10a783af0a31d1d4b5ba758f3694a2275b40faaf020000006a47304402206e38e36193f527d0679b49d56a84728fb473d981b229dd6d26fb4636c1b7d13e02203996d8c705d93f3f5b57c7d289d41ea17cb30c88931649271f07b295b13388ef4121031162a9a8f307b8e1efcafe3ce76b35ad293fad97ec885bcb8dbc6756d18ca941ffffffff030000000000000000896a04534c500001410747454e45534953065450414e4441044164616d4c5c68747470733a2f2f70616e642e61732e636173682f67656e6f6d652f303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030304c0001004c0008000000000000000122020000000000001976a9141431a2d4241cf1aa9df855cfd329304935a0383488acfa220000000000001976a9142cb677ece4990b3f587e90130f99660bfe4554f488ac00000000")
//            .unwrap();
//        let tx_msg = TxMessage::from_stream(&mut std::io::Cursor::new(tx_bytes)).unwrap();
//        ctx.address().do_send(IncomingMsg(Arc::new(tx_msg)));
    }
}

impl Handler<IncomingMsg<TxMessage>> for TxActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: IncomingMsg<TxMessage>, _ctx: &mut Self::Context) -> Self::Result {
        let tx = msg.0.tx.clone();
        Response::fut(
            self.resync
                .send(ProcessTransactions {
                    db: self.db.clone(),
                    subscribers: self.subscribers.clone(),
                    txs: vec![tx],
                    config: self.config.clone(),
                    broadcasts: self.broadcasts.clone(),
                })
                .from_err()
                .and_then(identity)
        )
    }
}


impl Handler<IncomingMsg<BlockMessage>> for TxActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: IncomingMsg<BlockMessage>, _ctx: &mut Self::Context) -> Self::Result {
        let hashes = msg.0.hashes.clone();
        let header = msg.0.header.clone();
        Response::fut(
            self.resync
                .send(ProcessBlock {
                    db: self.db.clone(),
                    subscribers: self.subscribers.clone(),
                    tx_hashes: hashes,
                    header,
                    config: self.config.clone(),
                    broadcasts: self.broadcasts.clone(),
                })
                .from_err()
                .and_then(identity)
        )
    }
}

impl Handler<ActivateAddress> for TxActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: ActivateAddress, _ctx: &mut Self::Context) -> Self::Result {
        let ActivateAddress(address) = msg;
        let resync = self.resync.clone();
        Response::fut(
            futures::future::result(self.db.lock().unwrap().set_address_active(&address, true)).from_err()
                .and_then(move |_| resync.send(ResyncAddress(address)).from_err())
                .and_then(identity)
        )
    }
}

impl Handler<DeactivateAddress> for TxActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: DeactivateAddress, _ctx: &mut Self::Context) -> Self::Result {
        let DeactivateAddress(address) = msg;
        Ok(self.db.lock().unwrap().set_address_active(&address, false)?)
    }
}

impl Handler<FetchAddressUtxos> for TxActor {
    type Result = Result<Vec<Utxo>, Error>;

    fn handle(&mut self, msg: FetchAddressUtxos, _ctx: &mut Self::Context) -> Self::Result {
        let FetchAddressUtxos(address) = msg;
        Ok(self.db.lock().unwrap().utxos_address(&address)?)
    }
}

impl Handler<FetchAddressTxDeltas> for TxActor {
    type Result = Result<Vec<TxDelta>, Error>;

    fn handle(&mut self, msg: FetchAddressTxDeltas, _ctx: &mut Self::Context) -> Self::Result {
        let FetchAddressTxDeltas(address) = msg;
        Ok(self.db.lock().unwrap().address_tx_deltas(&address)?)
    }
}

impl Handler<SubscribeToEvent> for TxActor {
    type Result = ();

    fn handle(&mut self, msg: SubscribeToEvent, _ctx: &mut Self::Context) -> Self::Result {
        let mut subscribers = self.subscribers.lock().unwrap();
        match msg {
            SubscribeToEvent::Address(address, recipient) => {
                subscribers.subscribers_address
                    .entry(address)
                    .or_insert_with(HashSet::new)
                    .insert(recipient);
            },
            SubscribeToEvent::Tokens(token_hashes, recipient) => {
                for (_, subs) in subscribers.subscribers_token.iter_mut() {
                    subs.remove(&recipient);
                }
                for token_hash in token_hashes {
                    subscribers.subscribers_token
                        .entry(token_hash)
                        .or_insert_with(HashSet::new)
                        .insert(recipient.clone());
                }
            },
        };
    }
}

impl Handler<UnsubscribeFromEvent> for TxActor {
    type Result = ();

    fn handle(&mut self, msg: UnsubscribeFromEvent, _ctx: &mut Self::Context) -> Self::Result {
        let mut subscribers = self.subscribers.lock().unwrap();
        match &msg {
            UnsubscribeFromEvent::Address(address, recipient) => {
                subscribers.subscribers_address.get_mut(address).map(|subs| subs.remove(recipient));
            },
        }
    }
}

impl Handler<FetchTradeOfferUtxos> for TxActor {
    type Result = Result<Vec<TradeOffer>, Error>;

    fn handle(&mut self, msg: FetchTradeOfferUtxos, _ctx: &mut Self::Context) -> Self::Result {
        let FetchTradeOfferUtxos(filter) = msg;
        Ok(self.db.lock().unwrap().trade_offer_utxos(filter)?)
    }
}
