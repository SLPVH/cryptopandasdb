use crate::schema::*;
use slpdexdb_base::BlockHeader;
use diesel::data_types::PgNumeric;
use diesel::sql_types::{Binary, BigInt, Numeric, Integer, Nullable};
use panda_base::traits::*;

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="blocks"]
pub struct Block {
    pub hash: Vec<u8>,
    pub height: i32,
    pub version: i32,
    pub prev_block: Vec<u8>,
    pub merkle_root: Vec<u8>,
    pub timestamp: i64,
    pub bits: i64,
    pub nonce: i64,
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct Token {
    pub id:                   i32,    // SERIAL PRIMARY KEY,
    pub hash:                 Vec<u8>, // BYTEA    NOT NULL,
    pub decimals:             i32,    // INT      NOT NULL,
    pub timestamp:            i64,    // BIGINT   NOT NULL,
    pub version_type:         i16,    // SMALLINT NOT NULL,
    pub document_uri:         Option<String>, // VARCHAR(200),
    pub symbol:               Option<String>, // VARCHAR(200),
    pub name:                 Option<String>, // VARCHAR(200),
    pub document_hash:        Option<String>, // VARCHAR(200),
    pub initial_supply:       PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub current_supply:       PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub block_created_height: i32,    // INT NOT NULL
    pub parent_token:         Option<i32>, // "parent_token" INT REFERENCES token (id) ON DELETE CASCADE
    pub parent_token_hash:    Option<Vec<u8>>, // "parent_token" BYTEA REFERENCES token (id) ON DELETE CASCADE
}

#[derive(Insertable)]
#[table_name="token"]
pub struct NewToken {
    pub hash:                 Vec<u8>, // BYTEA    NOT NULL,
    pub decimals:             i32,    // INT      NOT NULL,
    pub timestamp:            i64,    // BIGINT   NOT NULL,
    pub version_type:         i16,    // SMALLINT NOT NULL,
    pub document_uri:         Option<String>, // VARCHAR(200),
    pub symbol:               Option<String>, // VARCHAR(200),
    pub name:                 Option<String>, // VARCHAR(200),
    pub document_hash:        Option<String>, // VARCHAR(200),
    pub initial_supply:       PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub current_supply:       PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub block_created_height: i32,    // INT NOT NULL
}

#[derive(Debug)]
#[derive(Queryable)]
pub struct Tx {
    pub id:        i64, // BIGSERIAL PRIMARY KEY,
    pub hash:      Vec<u8>, // BYTEA NOT NULL,
    pub height:    Option<i32>, // INT NOT NULL,
    pub timestamp: i64, // BIGINT NOT NULL,
    pub tx_type:   i32, // INT NOT NULL
}

#[derive(Insertable)]
#[table_name="tx"]
pub struct NewTx {
    pub hash:      Vec<u8>, // BYTEA NOT NULL,
    pub height:    Option<i32>, // INT NOT NULL,
    pub timestamp: i64, // BIGINT NOT NULL,
    pub tx_type:   i32, // INT NOT NULL
}

#[derive(Debug)]
#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="slp_tx"]
pub struct SlpTx {
    pub tx:       i64, // BIGINT PRIMARY KEY REFERENCES tx ("id") ON DELETE CASCADE,
    pub token:    i32, // INT REFERENCES token ("id") ON DELETE RESTRICT,
    pub version:  i32, // INT NOT NULL,
    pub slp_type: String, // VARCHAR(14) NOT NULL
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="tx_output"]
pub struct TxOutput {
    pub tx:               i64, // BIGINT REFERENCES tx (id) ON DELETE CASCADE,
    pub idx:              i32, // INT NOT NULL,
    pub value_satoshis:   i64, // BIGINT NOT NULL,
    pub value_token_base: PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub address:          Option<Vec<u8>>, // BYTEA,
    pub output_type:      i32, // INT NOT NULL,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="tx_input"]
pub struct TxInput {
    pub tx:         i64, // BIGINT REFERENCES tx (id) ON DELETE CASCADE,
    pub idx:        i32, // INT NOT NULL,
    pub output_tx:  Vec<u8>, // BIGINT,  -- can be null
    pub output_idx: i32, // INT,
    pub address:    Option<Vec<u8>>, // BYTEA
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="pending_pnd1_tx"]
pub struct PND1Tx {
    pub tx:     i64, // BIGINT NOT NULL PRIMARY KEY REFERENCES tx (id) ON DELETE CASCADE,
    pub father: i64, // BIGINT NOT NULL REFERENCES panda (id) ON DELETE CASCADE,
    pub mother: i64, // BIGINT NOT NULL REFERENCES panda (id) ON DELETE CASCADE,
    pub name: String, // TEXT NOT NULL
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="pandaop_utxo"]
pub struct PandaopUtxo {
    pub tx_hash: Vec<u8>,
    pub vout: i32,
}

#[derive(Queryable)]
pub struct TradeOffer {
    pub id:                     i64, // SERIAL PRIMARY KEY,
    pub tx:                     i64, // BIGINT REFERENCES tx (id) ON DELETE CASCADE,
    pub output_idx:             Option<i32>, // INT NOT NULL,
    pub input_tx:               Vec<u8>, // BYTEA NOT NULL,
    pub input_idx:              i32, // INT NOT NULL,
    pub price_per_token:        PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub script_price:           i64, // BIGINT NOT NULL,
    pub is_inverted:            bool, // BOOL NOT NULL,
    pub sell_amount_token_base: PgNumeric, // NUMERIC(26) NOT NULL,
    pub receiving_address:      Vec<u8>, // BYTEA NOT NULL,
    pub spent:                  bool, // BOOL NOT NULL
}

#[derive(Insertable)]
#[table_name="trade_offer"]
pub struct NewTradeOffer {
    pub tx:                     i64, // BIGINT REFERENCES tx (id) ON DELETE CASCADE,
    pub output_idx:             Option<i32>, // INT NOT NULL,
    pub input_tx:               Vec<u8>, // BYTEA NOT NULL,
    pub input_idx:              i32, // INT NOT NULL,
    pub price_per_token:        PgNumeric, // NUMERIC(52, 26) NOT NULL,
    pub script_price:           i64, // BIGINT NOT NULL,
    pub is_inverted:            bool, // BOOL NOT NULL,
    pub sell_amount_token_base: PgNumeric, // NUMERIC(26) NOT NULL,
    pub receiving_address:      Vec<u8>, // BYTEA NOT NULL
}

#[derive(Queryable)]
pub struct UpdateHistory {
    pub id:              i64, // BIGSERIAL PRIMARY KEY,
    pub last_height:     i32, // INT NOT NULL,
    pub last_tx_hash:    Option<Vec<u8>>, // BYTEA NOT NULL,
    pub last_tx_hash_be: Option<Vec<u8>>, // BYTEA NOT NULL,
    pub subject_type:    i32, // INT NOT NULL,
    pub subject_hash:    Option<Vec<u8>>, // INT NOT NULL,
    pub timestamp:       chrono::DateTime<chrono::Utc>, // TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    pub completed:       bool, // BOOL NOT NULL
    pub is_confirmed:    bool, // BOOL NOT NULL
}

#[derive(Insertable)]
#[table_name="update_history"]
pub struct NewUpdateHistory {
    pub last_height:     i32, // INT NOT NULL,
    pub last_tx_hash:    Option<Vec<u8>>, // BYTEA NOT NULL,
    pub last_tx_hash_be: Option<Vec<u8>>, // BYTEA NOT NULL,
    pub subject_type:    i32, // INT NOT NULL,
    pub subject_hash:    Option<Vec<u8>>, // BYTEA NOT NULL,
    pub completed:       bool, // BOOL NOT NULL
    pub is_confirmed:    bool, // BOOL NOT NULL
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="active_address"]
pub struct ActiveAddress {
    pub address: Vec<u8>,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="utxo_address"]
pub struct UtxoAddress {
    pub tx: i64,
    pub idx: i32,
    pub address: Option<Vec<u8>>,
}

#[derive(Queryable)]
#[derive(Insertable)]
#[table_name="utxo_trade_offer"]
pub struct Utxo {
    pub tx: i64,
    pub idx: i32,
}

#[derive(Debug)]
#[derive(QueryableByName)]
pub struct TxDeltaInput {
    #[sql_type="BigInt"]
    pub tx_id: i64,
    #[sql_type="Binary"]
    pub tx_hash: Vec<u8>,
    #[sql_type="BigInt"]
    pub timestamp: i64,
    #[sql_type="Nullable<Numeric>"]
    pub input_value_satoshis: Option<PgNumeric>,
    #[sql_type="Nullable<Numeric>"]
    pub input_value_token_base: Option<PgNumeric>,
    #[sql_type="Nullable<Binary>"]
    pub token_hash: Option<Vec<u8>>,
    #[sql_type="Nullable<Integer>"]
    pub decimals: Option<i32>,
}

#[derive(Debug)]
#[derive(QueryableByName)]
pub struct TxDeltaOutput {
    #[sql_type="BigInt"]
    pub tx_id: i64,
    #[sql_type="Binary"]
    pub tx_hash: Vec<u8>,
    #[sql_type="BigInt"]
    pub timestamp: i64,
    #[sql_type="Nullable<Numeric>"]
    pub output_value_satoshis: Option<PgNumeric>,
    #[sql_type="Nullable<Numeric>"]
    pub output_value_token_base: Option<PgNumeric>,
    #[sql_type="Nullable<Binary>"]
    pub token_hash: Option<Vec<u8>>,
    #[sql_type="Nullable<Integer>"]
    pub decimals: Option<i32>,
}

impl Block {
    pub fn from_block_header(header: &BlockHeader, height: i32) -> Block {
        Block {
            hash: header.hash().to_vec(),
            height,
            version: header.version,
            prev_block: header.prev_block.to_vec(),
            merkle_root: header.merkle_root.to_vec(),
            timestamp: header.timestamp as i64,
            bits: header.bits as i64,
            nonce: header.nonce as i64,
        }
    }

    pub fn to_block_header(&self) -> BlockHeader {
        BlockHeader {
            version: self.version,
            prev_block: {
                let mut prev_block = [0; 32];
                prev_block.copy_from_slice(&self.prev_block);
                prev_block
            },
            merkle_root: {
                let mut merkle_root = [0; 32];
                merkle_root.copy_from_slice(&self.merkle_root);
                merkle_root
            },
            timestamp: self.timestamp as u32,
            bits: self.bits as u32,
            nonce: self.nonce as u32,
        }
    }
}

#[derive(Insertable)]
#[table_name = "panda"]
pub struct NewPanda<'a> {
    pub genesis_tx: &'a i64,
    pub owner_tx: &'a i64,
    pub owner_tx_idx: &'a i32,
    pub physique: &'a PhysiqueTrait,
    pub pattern: &'a PatternTrait,
    pub eye_color: &'a EyeColorTrait,
    pub eye_shape: &'a EyeShapeTrait,
    pub base_color: &'a BaseColorTrait,
    pub highlight_color: &'a HighlightColorTrait,
    pub accent_color: &'a AccentColorTrait,
    pub wild_element: &'a WildElementTrait,
    pub mouth: &'a MouthTrait,
    pub genes: &'a [u8]
}

#[derive(Queryable)]
pub struct DbPanda {
    pub id: i64,
    pub genesis_tx: i64,
    pub owner_tx: i64,
    pub owner_tx_idx: i32,
    pub genes: Vec<u8>
}

#[derive(Queryable)]
pub struct DbPandaFull {
    pub hash: Vec<u8>,
    pub address: Option<Vec<u8>>,
    pub physique: PhysiqueTrait,
    pub pattern: PatternTrait,
    pub eye_color: EyeColorTrait,
    pub eye_shape: EyeShapeTrait,
    pub base_color: BaseColorTrait,
    pub highlight_color: HighlightColorTrait,
    pub accent_color: AccentColorTrait,
    pub wild_element: WildElementTrait,
    pub mouth: MouthTrait,
    pub genes: Vec<u8>
}

impl DbPanda {
    pub fn genes(&self) -> [u8; 48] {
        let mut genes: [u8; 48] = [0; 48];
        genes.copy_from_slice(&self.genes);
        genes
    }

    pub fn get_attributes(&self) -> Result<PandaAttributes, InvalidGeneInteger> {
        PandaAttributes::from_genes(&self.genes())
    }
}