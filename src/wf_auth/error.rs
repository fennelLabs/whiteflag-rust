use hex::FromHexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthenticationError {
    #[error("error")]
    HexDecode(FromHexError),
    #[error("error")]
    General(&'static str),
}
