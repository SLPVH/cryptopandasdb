use diesel::data_types::PgNumeric;

#[derive(Debug)]
pub enum SLPError {
    NotSLPSafe,  // SLP ops must be bigger than 0x00 and less than 0x4f
    TooFewPushops(usize),  // SLP output must have at least 6 pushops
    TooManyAmounts(usize),  // SLP output must have at most 19 outputs
    InvalidSLPType(String),
    InvalidTokenTypeLength(String),
    InvalidTokenHashLength(String),
    NoMatch,
}

#[derive(Debug)]
pub enum NumericError {
    NaN,
    NotInteger(PgNumeric),  // expected integer numeric, got fractional
    TooManyDigits(String),  // decimal number has too many fractional digits
}

#[derive(Debug)]
pub enum TradeOfferError {
    InvalidPrice(Vec<u8>),
    InvalidPower(Vec<u8>),
}

#[derive(Debug)]
pub enum TokenError {
    TokenNotMinedYet(String),
    UnknownTokenId(String),
    InvalidTokenHex(String),
    UnsupportedTokenType(i16),
    InvalidNftParent,
}

#[derive(Debug)]
pub enum PNDError {
    InvalidNumberOfPushops(usize),
    InvalidName(String, std::string::FromUtf8Error),
    InvalidFatherHash(String),
    InvalidMotherHash(String),
    FatherIsMother,
    WrongFatherToken(String),
    WrongMotherToken(String),
    InvalidPubkey(String),
    InvalidSignature(String),
    SignatureVerifyFail(String, String, String),
    InvalidFatherOutputIdx(String),
    InvalidMotherOutputIdx(String),
    FatherDoesntExist(String),
    MotherDoesntExist(String),
    InvalidFatherUTXO(String),
    InvalidMotherUTXO(String),
    FatherNotOwnedByAddress(String, String),
    MotherNotOwnedByAddress(String, String),
    DoesntPayPandaFee,
}

error_chain! {
    foreign_links {
        Fmt(std::fmt::Error);
        Io(std::io::Error);
        VarError(std::env::VarError);
        Query(diesel::result::Error);
        DbConnection(diesel::ConnectionError);
        Request(reqwest::Error);
        ParseInt(std::num::ParseIntError);
        FromHex(hex::FromHexError);
        Mailbox(actix::MailboxError);
    }

    errors {
        NumericError(num_error: NumericError) {
            description("Numeric error")
            display("Numeric error {:?}", num_error)
        }

        TokenError(token_error: TokenError) {
            description("Invalid token")
            display("Invalid token: {:?}", token_error)
        }

        InvalidSLPOutput(script_hex: String, slp_error: SLPError) {
            description("Invalid SLP Output")
            display("Invalid SLP Output: {} {:?}", script_hex, slp_error)
        }

        InvalidTradeOffer(trade_offer_error: TradeOfferError) {
            description("Invalid trade offer")
            display("Invalid trade offer: {:?}", trade_offer_error)
        }

        InvalidPND(pnd_error: PNDError) {
            description("Invalid PND")
            display("Invalid PND: {:?}", pnd_error)
        }
    }
}
