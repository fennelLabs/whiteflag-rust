use hex::FromHexError;
use thiserror::Error;

pub type WhiteflagResult<T> = Result<T, WhiteflagError>;

#[derive(Error, Debug)]
pub enum WhiteflagError {
    #[error("error")]
    Serde(serde_json::Error),
    #[error("issue decoding hex string")]
    HexDecode(#[from] FromHexError),
}
