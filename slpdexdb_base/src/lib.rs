#![recursion_limit="128"]

#[macro_use]
extern crate error_chain;

mod config;
pub mod convert_numeric;
mod errors;
mod slp_amount;
mod block;

pub use config::*;
pub use errors::{Error, ErrorKind, TradeOfferError, NumericError, SLPError, TokenError, Result, PNDError, PandaError};
pub use slp_amount::*;
pub use block::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
