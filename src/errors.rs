use switchboard_solana::prelude::*;

#[sb_error]
pub enum CustomError {
    ParseError,
    FetchError,
}
