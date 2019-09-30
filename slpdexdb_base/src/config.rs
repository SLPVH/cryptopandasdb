use cashcontracts::tx_hex_to_hash;

#[derive(Clone, Debug)]
pub struct SLPDEXConfig {
    pub fee_address: cashcontracts::Address,
    pub fee_divisor: u64,
    pub dust_limit: u64,
    pub exch_lokad: &'static str,
    pub exch_lokad_b64: String,
    pub exch_version: i32,
    pub panda_token_hash: [u8; 32],
    pub panda_fee: u64,
}

impl Default for SLPDEXConfig {
    fn default() -> Self {
        SLPDEXConfig {
            fee_address: cashcontracts::Address::from_cash_addr(
                "bitcoincash:qr4tqy4xye3y7cxtwxskr0l445lf55tnnchv8474jd".to_string()
            ).unwrap(),
            fee_divisor: 500,
            dust_limit: 0x222,
            exch_lokad: "EXCH",
            exch_lokad_b64: base64::encode("EXCH"),
            exch_version: 2,
            panda_token_hash: tx_hex_to_hash("511801e55bf03a93b55654d9f3123c3291a33d11dccdeafe23123daf802f2f84").unwrap(),
            panda_fee: 100_000,
        }
    }
}
