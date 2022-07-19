use std::fmt;

#[derive(Debug)]
pub enum WhiteflagCryptoError {
    InvalidMethod,
}

impl std::error::Error for WhiteflagCryptoError {}

impl fmt::Display for WhiteflagCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagCryptoError::InvalidMethod => write!(f, "invalid crypto method"),
        }
    }
}

pub type WhiteflagCryptoResult<T> = Result<T, WhiteflagCryptoError>;