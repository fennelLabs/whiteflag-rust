use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("issue serializing struct")]
    SerdeJsonError(#[from] serde_json::error::Error),
    #[error("message code does not exist")]
    InvalidMessageCode,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}
