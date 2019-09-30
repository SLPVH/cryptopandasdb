use actix::prelude::*;
use slpdexdb_base::Error;
use crate::actors::{NodeActor, IncomingMsg};
use crate::messages::{VersionMessage, VerackMessage, InvMessage, HeadersMessage, TxMessage, BlockMessage};

pub enum Subscribe {
    HandshakeSuccess(Recipient<HandshakeSuccess>),
    Version(Recipient<IncomingMsg<VersionMessage>>),
    Verack(Recipient<IncomingMsg<VerackMessage>>),
    Inv(Recipient<IncomingMsg<InvMessage>>),
    Headers(Recipient<IncomingMsg<HeadersMessage>>),
    Tx(Recipient<IncomingMsg<TxMessage>>),
    Block(Recipient<IncomingMsg<BlockMessage>>),
}

impl Message for Subscribe {
    type Result = ();
}

#[derive(Clone)]
pub struct HandshakeSuccess;

impl Message for HandshakeSuccess {
    type Result = Result<(), Error>;
}
