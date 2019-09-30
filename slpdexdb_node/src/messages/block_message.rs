use crate::message_packet::MessagePacket;
use crate::message::NodeMessage;
use slpdexdb_base::BlockHeader;
use cashcontracts::{Tx, serialize};
use std::io;

#[derive(Clone, Debug)]
pub struct BlockMessage {
    pub header: BlockHeader,
    pub hashes: Vec<[u8; 32]>,
}

impl NodeMessage for BlockMessage {
    fn command() -> &'static [u8] {
        b"block"
    }

    fn packet(&self) -> MessagePacket {
        unimplemented!()
    }

    fn from_stream(stream: &mut impl io::Read) -> io::Result<Self> {
        let header = BlockHeader::from_stream(stream)?;
        let mut hashes = Vec::new();
        let n_hashes = serialize::read_var_int(stream)?;
        for _ in 0..n_hashes {
            let mut hash = [0; 32];
            stream.read_exact(&mut hash);
            hashes.push(hash);
        }
        Ok(BlockMessage { header, hashes })
    }
}

/*impl std::fmt::Display for TxMessage {
    fn fmt<'a>(&self, f: &mut std::fmt::Formatter<'a>) -> Result<(), std::fmt::Error> {
        writeln!(f, "num of invs: {}", self.inv_vectors.len())?;
        for inv_vector in self.inv_vectors.iter() {
            writeln!(f, "{:?}\t{}", inv_vector.type_id, tx_hash_to_hex(&inv_vector.hash))?;
        }
        Ok(())
    }
}*/
