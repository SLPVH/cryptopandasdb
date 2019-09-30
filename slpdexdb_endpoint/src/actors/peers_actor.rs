use actix::prelude::*;
use tokio_tcp::TcpStream;
use std::convert::identity;
use std::sync::Arc;
use slpdexdb_base::Error;
use slpdexdb_node::actors::{NodeActor, IncomingMsg};
use slpdexdb_node::DbActor;
use slpdexdb_node::msg::Subscribe;
use slpdexdb_node::messages::{TxMessage, BlockMessage};
use slpdexdb_node::NodeMessage;


use crate::actors::TxActor;
use crate::msg::ConnectToPeer;


pub struct PeersActor {
    tx_actor: Addr<TxActor>,
    db_actor: Addr<DbActor>,
    nodes: Vec<Addr<NodeActor>>,
}

impl PeersActor {
    pub fn new(tx_actor: Addr<TxActor>, db_actor: Addr<DbActor>) -> Self {
        PeersActor {
            tx_actor,
            db_actor,
            nodes: Vec::new(),
        }
    }
}

pub struct PeerConnected {
    pub node: Addr<NodeActor>,
}

impl Message for PeerConnected {
    type Result = ();
}

impl Actor for PeersActor {
    type Context = Context<Self>;
}

impl Handler<ConnectToPeer> for PeersActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: ConnectToPeer, ctx: &mut Self::Context) -> Self::Result {
        let own_addr = ctx.address();
        let own_addr2 = ctx.address();
        let db_addr = self.db_actor.clone();
        println!("connecting on {}", msg.socket_addr);
        Response::fut(
            TcpStream::connect(&msg.socket_addr)
                .from_err()
                .and_then(move |stream| {
                    println!("connected");
                    let node = NodeActor::create_from_stream_db(stream, db_addr);
                    let node2 = node.clone();
                    node.send(Subscribe::Tx(own_addr.clone().recipient())).from_err()
                        .and_then(move |_| node2.send(Subscribe::Block(own_addr2.clone().recipient())).from_err())
                        .and_then(move |_| own_addr.send(PeerConnected { node }).from_err())
                })
                .map_err(|err| {
                    println!("{}", err);
                    err
                })
        )
    }
}

impl Handler<PeerConnected> for PeersActor {
    type Result = ();

    fn handle(&mut self, msg: PeerConnected, _ctx: &mut Self::Context) -> Self::Result {
        self.nodes.push(msg.node);
    }
}

impl Handler<IncomingMsg<TxMessage>> for PeersActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: IncomingMsg<TxMessage>, _ctx: &mut Self::Context) -> Self::Result {
        Response::fut(self.tx_actor.send(msg).from_err().and_then(identity))
    }
}

impl Handler<IncomingMsg<BlockMessage>> for PeersActor {
    type Result = Response<(), Error>;

    fn handle(&mut self, msg: IncomingMsg<BlockMessage>, _ctx: &mut Self::Context) -> Self::Result {
        Response::fut(self.tx_actor.send(msg).from_err().and_then(identity))
    }
}
