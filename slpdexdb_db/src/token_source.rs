use crate::endpoint::Endpoint;
use crate::tx_source::{TxFilter, SortKey};
use cashcontracts::tx_hash_to_hex;
use json::{JsonValue, object, object::Object};

pub struct TokenSource {
    endpoint: Endpoint
}

pub mod token_result {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct TokenDetails {
        pub decimals: i32,
        #[serde(rename = "tokenIdHex")]
        pub token_id_hex: String,
        pub timestamp: Option<String>,
        pub timestamp_unix: Option<i64>,
        #[serde(rename = "transactionType")]
        pub transaction_type: String,
        #[serde(rename = "versionType")]
        pub version_type: i16,
        #[serde(rename = "documentUri")]
        pub document_uri: String,
        #[serde(rename = "documentSha256Hex")]
        pub document_sha256_hex: Option<String>,
        pub symbol: String,
        pub name: String,
        #[serde(rename = "batonVout")]
        pub baton_vout: Option<i32>,
        #[serde(rename = "containsBaton")]
        pub contains_baton: bool,
        #[serde(rename = "genesisOrMintQuantity")]
        pub genesis_or_mint_quantity: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct TokenStats {
        pub block_created: Option<i32>,
        pub block_last_active_send: Option<i32>,
        pub block_last_active_mint: Option<i32>,
        pub qty_valid_txns_since_genesis: i32,
        pub qty_valid_token_utxos: i32,
        pub qty_valid_token_addresses: i32,
        pub qty_token_minted: String,
        pub qty_token_burned: String,
        pub qty_token_circulating_supply: String,
        pub qty_satoshis_locked_up: i32,
        pub minting_baton_status: String,
    }

    #[derive(Deserialize, Debug)]
    pub struct TokenEntry {
        pub schema_version: i32,
        #[serde(rename = "lastUpdatedBlock")]
        pub last_updated_block: i32,
        #[serde(rename = "mintBatonUtxo")]
        pub mint_baton_utxo: String,
        #[serde(rename = "tokenStats")]
        pub token_stats: TokenStats,
        #[serde(rename = "tokenDetails")]
        pub token_details: TokenDetails,
        #[serde(rename = "nftParentId")]
        pub nft_parent_id: Option<String>,
    }

    #[derive(Deserialize, Debug)]
    pub struct TokenResult {
        pub t: Vec<TokenEntry>,
    }
}

impl TokenSource {
    pub fn new() -> Self {
        TokenSource {
            endpoint: Endpoint::new(),
        }
    }

    fn _conditions(filters: &[TxFilter]) -> Vec<(&'static str, JsonValue)> {
        filters.iter()
            .filter_map(|filter| match filter {
                TxFilter::MinTxHash(tx_hash) => Some(
                    ("tokenDetails.tokenIdHex",
                     object!{"$gt" => tx_hash_to_hex(tx_hash)})
                ),
                TxFilter::MinBlockHeight(height) => Some(
                    ("tokenStats.block_created", object!{"$gte" => *height})
                ),
                TxFilter::TokenId(token_hash) => Some(
                    ("tokenDetails.tokenIdHex",
                     JsonValue::String(tx_hash_to_hex(token_hash)))
                ),
                _ => None,
            })
            .collect()
    }

    fn _sort_by(filters: &[TxFilter]) -> JsonValue {
        filters.iter()
            .find_map(|filter| match filter {
                TxFilter::SortBy(sort) => {
                    match sort {
                        SortKey::TxHash => Some(object!{"tokenDetails.tokenIdHex" => 1}),
                    }
                },
                _ => None,
            })
            .unwrap_or(object!{})
    }

    pub fn request_tokens(&self, filters: &[TxFilter])
            -> reqwest::Result<Vec<token_result::TokenEntry>> {
        let mut condition_json = Object::new();
        for (key, json) in Self::_conditions(filters) {
            condition_json.insert(key, json);
        }
        let sort = Self::_sort_by(filters);
        let query_json = json::stringify(object!{
            "v" => 3,
            "q" => object!{
                "db" => "t",
                "find" => JsonValue::Object(condition_json),
                "sort" => sort,
            },
        });
        println!("{}", query_json);
        let query_b64 = base64::encode(&query_json);
        let text = reqwest
            ::get(&format!("{}{}", self.endpoint.slpdb_endpoint_url, query_b64))?
            .text()?;
        println!("{}", text);
        let result: token_result::TokenResult = serde_json::from_str(&text).unwrap();
        Ok(result.t)
    }
}
