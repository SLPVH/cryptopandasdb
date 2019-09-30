use actix::prelude::*;
use tokio_tcp::TcpStream;
use tokio_io::io::WriteHalf;
use tokio_codec::FramedRead;
use tokio_io::AsyncRead;
use std::io;
use std::sync::Arc;
use std::net::SocketAddr;

use slpdexdb_base::Error;

use crate::codec::MessageCodec;
use crate::message::NodeMessage;
use crate::messages::{VersionMessage, VerackMessage, InvMessage, HeadersMessage, TxMessage, BlockMessage};
use crate::message_packet::MessagePacket;
use crate::actors::{VersionActor, InvActor, BlockHeaderActor};
use crate::msg::{Subscribe, HandshakeSuccess};
use crate::db_query::DbActor;

pub struct IncomingMsg<M: NodeMessage>(pub Arc<M>);

impl<M: NodeMessage> Message for IncomingMsg<M> {
    type Result = Result<(), Error>;
}

pub struct OutgoingMsg(pub MessagePacket);

impl Message for OutgoingMsg {
    type Result = ();
}

pub struct NodeActor {
    framed: actix::io::FramedWrite<WriteHalf<TcpStream>, MessageCodec>,

    subscribers_version: Vec<Recipient<IncomingMsg<VersionMessage>>>,
    subscribers_verack: Vec<Recipient<IncomingMsg<VerackMessage>>>,
    subscribers_inv: Vec<Recipient<IncomingMsg<InvMessage>>>,
    subscribers_headers: Vec<Recipient<IncomingMsg<HeadersMessage>>>,
    subscribers_tx: Vec<Recipient<IncomingMsg<TxMessage>>>,
    subscribers_handshake: Vec<Recipient<HandshakeSuccess>>,
    subscribers_block: Vec<Recipient<IncomingMsg<BlockMessage>>>,
}

impl NodeActor {
    pub fn create_from_stream_db(stream: TcpStream, db_actor: Addr<DbActor>) -> Addr<Self> {
        let local_addr = stream.local_addr().unwrap(); // TODO: handle error
        //println!("local addr {}", local_addr);
        //let local_addr = SocketAddr::new("185.220.70.238".parse().unwrap(), 8333);
        let peer_addr = stream.peer_addr().unwrap(); // TODO: handle error
        let addr = NodeActor::create(|ctx| {
            let (r, w) = stream.split();
            ctx.add_stream(FramedRead::new(r, MessageCodec));
            NodeActor {
                framed: actix::io::FramedWrite::new(
                    w,
                    MessageCodec,
                    ctx,
                ),
                subscribers_handshake: Vec::new(),
                subscribers_inv: Vec::new(),
                subscribers_version: Vec::new(),
                subscribers_verack: Vec::new(),
                subscribers_headers: Vec::new(),
                subscribers_tx: Vec::new(),
                subscribers_block: Vec::new(),
            }
        });
        InvActor::start(InvActor { node: addr.clone() });
        VersionActor::start(VersionActor { node: addr.clone(), local_addr, peer_addr });
        BlockHeaderActor::start(BlockHeaderActor { node: addr.clone(), db: db_actor });
        addr
    }

    fn _broadcast<M: NodeMessage + Send + Sync>(msg: MessagePacket, subs: &[Recipient<IncomingMsg<M>>]) {
        let mut cur = io::Cursor::new(msg.payload());
        let msg = Arc::new(M::from_stream(&mut cur).unwrap());  // TODO: handle error
        for sub in subs.iter() {
            sub.do_send(IncomingMsg(Arc::clone(&msg)));
        }
    }
}

impl Actor for NodeActor {
    type Context = Context<Self>;
}

impl actix::io::WriteHandler<io::Error> for NodeActor {
    fn error(&mut self, err: io::Error, _ctx: &mut Self::Context) -> Running {
        eprintln!("error: {}", err);
        Running::Continue
    }
}

impl StreamHandler<MessagePacket, io::Error> for NodeActor {
    fn handle(&mut self, msg: MessagePacket, ctx: &mut Context<Self>) {
        //println!("msg: {}", msg);
        match msg.header().command_name() {
            b"version" => Self::_broadcast(msg, &self.subscribers_version),
            b"verack" => Self::_broadcast(msg, &self.subscribers_verack),
            b"inv" => Self::_broadcast(msg, &self.subscribers_inv),
            b"headers" => Self::_broadcast(msg, &self.subscribers_headers),
            b"verack" => Self::_broadcast(msg, &self.subscribers_verack),
            b"tx" => Self::_broadcast(msg, &self.subscribers_tx),
            b"block" => {
                println!("block msg: {}", msg);
                Self::_broadcast(msg, &self.subscribers_block)
            },
            _ => {
            },
        }
    }
}

impl Handler<Subscribe> for NodeActor {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, ctx: &mut Self::Context) -> Self::Result {
        match msg {
            Subscribe::Version(recipient) => self.subscribers_version.push(recipient),
            Subscribe::Verack(recipient) => self.subscribers_verack.push(recipient),
            Subscribe::Inv(recipient) => self.subscribers_inv.push(recipient),
            Subscribe::HandshakeSuccess(recipient) => self.subscribers_handshake.push(recipient),
            Subscribe::Headers(recipient) => self.subscribers_headers.push(recipient),
            Subscribe::Tx(recipient) => self.subscribers_tx.push(recipient),
            Subscribe::Block(recipient) => self.subscribers_block.push(recipient),
        }
    }
}

impl Handler<HandshakeSuccess> for NodeActor {
    type Result = Result<(), Error>;

    fn handle(&mut self, msg: HandshakeSuccess, ctx: &mut Self::Context) -> Self::Result {
        for sub in self.subscribers_handshake.iter() {
            sub.do_send(msg.clone());
        }
        Ok(())
    }
}

impl Handler<OutgoingMsg> for NodeActor {
    type Result = ();

    fn handle(&mut self, msg: OutgoingMsg, _: &mut Self::Context) -> Self::Result {
        self.framed.write(msg.0);
    }
}
