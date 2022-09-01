use thiserror::Error;
pub type WhiteflagResult<T> = Result<T, WhiteflagError>;

#[derive(Error, Debug)]
pub enum WhiteflagError {
    #[error("error")]
    Serde(serde_json::Error),
    #[error("issue decoding hex string")]
    HexDecode(#[from] hex::FromHexError),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
