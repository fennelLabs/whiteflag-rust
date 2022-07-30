use std::fmt;

#[derive(Debug)]
pub enum WhiteflagCryptoError {
    InvalidMethod,
    KeypairDestroyed,
    InvalidCipher,
}

impl std::error::Error for WhiteflagCryptoError {}

impl fmt::Display for WhiteflagCryptoError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagCryptoError::InvalidMethod => write!(f, "invalid crypto method"),
            WhiteflagCryptoError::KeypairDestroyed => write!(f, "keypair destroyed"),
            WhiteflagCryptoError::InvalidCipher => {
                write!(f, "Context and/or initialisation vector have not been set")
            }
        }
    }
}

pub type WhiteflagCryptoResult<T> = Result<T, WhiteflagCryptoError>;
