use thiserror::Error;

#[derive(Error, Debug)]
pub enum WhiteflagError {
    #[error("wf_field::Error")]
    WFFieldError(#[from] wf_field::Error),
    #[error("error")]
    Serde(serde_json::Error),
    #[error("issue decoding hex string")]
    HexDecode(#[from] hex::FromHexError),
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
