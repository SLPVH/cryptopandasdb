use cashcontracts::Address;
use slpdexdb_base::SLPAmount;

#[derive(Clone, Debug)]
pub struct Utxo {
    pub tx_hash: [u8; 32],
    pub vout: i32,
    pub value_satoshis: u64,
    pub value_token: SLPAmount,
    pub token_hash: Option<[u8; 32]>,
}

#[derive(Clone, Debug)]
pub struct SpentUtxo {
    pub tx_hash: [u8; 32],
    pub vout: i32,
}

#[derive(Clone, Debug)]
pub enum NewUtxo {
    Address {
        tx_hash: [u8; 32],
        vout: i32,
        address: Address,
    },
    TradeOffer {
        tx_hash: [u8; 32],
        vout: i32,
    },
}

#[derive(Clone, Debug)]
pub enum TradeOfferFilter {
    TokenHash([u8; 32]),
    ReceivingAddress(Address),
}

#[derive(Clone, Debug)]
pub struct TxDelta {
    pub tx_hash: [u8; 32],
    pub delta_satoshis: i64,
    pub delta_token: SLPAmount,
    pub token_hash: Option<[u8; 32]>,
    pub timestamp: i64,
}

pub fn tx_hash_from_slice(slice: &[u8]) -> [u8; 32] {
    let mut hash = [0; 32];
    hash.copy_from_slice(&slice);
    hash
}

pub fn tx_hash_from_le_slice(slice: &[u8]) -> [u8; 32] {
    let mut hash = [0; 32];
    for i in 0..32 {
        hash[i] = slice[31 - i];
    }
    hash
}

pub fn address_hash_from_slice(slice: &[u8]) -> [u8; 20] {
    let mut hash = [0; 20];
    hash.copy_from_slice(&slice);
    hash
}
