use actix::prelude::*;
use slpdexdb_base::Error;
use crate::messages::{HeadersMessage, GetDataMessage, GetHeadersMessage};
use crate::message::NodeMessage;
use crate::message_packet::MessagePacket;
use crate::actors::{NodeActor, IncomingMsg, OutgoingMsg};
use crate::db_query::{DbActor, HeaderTipQuery, AddHeadersQuery};
use crate::msg::{Subscribe, HandshakeSuccess};

pub struct BlockHeaderActor {
    pub db: Addr<DbActor>,
    pub node: Addr<NodeActor>,
}

impl Actor for BlockHeaderActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.node.do_send(Subscribe::HandshakeSuccess(ctx.address().recipient()));
        self.node.do_send(Subscribe::Headers(ctx.address().recipient()));
    }
}

fn dump_err(err: Error) {
    eprintln!("Error: {}", err);
}

impl BlockHeaderActor {
    fn _fetch_headers(db: Addr<DbActor>, node: Addr<NodeActor>)
            -> impl futures::Future<Item=(), Error=Error> {
        db.send(HeaderTipQuery).from_err()
            .and_then(|x| x)
            .and_then(move |tip| {
                node.send(OutgoingMsg(GetHeadersMessage {
                    version: 70015,
                    block_locator_hashes: vec![tip.header.hash()],
                    hash_stop: [0; 32],
                }.packet())).from_err()
            })
    }
}

impl Handler<HandshakeSuccess> for BlockHeaderActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: HandshakeSuccess, ctx: &mut Self::Context) -> Self::Result {
        let node = self.node.clone();
        let db = self.db.clone();
        Response::fut(
            Self::_fetch_headers(db, node)
            /*self.node
                .send(OutgoingMsg(MessagePacket::from_payload(b"sendheaders", vec![]))).from_err()
                .and_then(move |_| Self::_fetch_headers(db, node))*/
        )
    }
}

impl Handler<IncomingMsg<HeadersMessage>> for BlockHeaderActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: IncomingMsg<HeadersMessage>, ctx: &mut Self::Context) -> Self::Result {
        let node = self.node.clone();
        let db = self.db.clone();
        if msg.0.headers.len() == 0 {
            return Response::reply(Ok(()));
        }
        Response::fut(
            self.db.send(AddHeadersQuery(msg.0.headers.clone())).from_err()
                .and_then(|r| r)
                .and_then(move |_| Self::_fetch_headers(db, node))
        )
    }
}
