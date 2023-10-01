use std::fmt;

#[derive(Debug)]
pub enum WhiteflagError {
    InvalidHex,
    InvalidPattern,
    InvalidLength,
    CannotRetrieveKey,
}

impl std::error::Error for WhiteflagError {}

impl fmt::Display for WhiteflagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagError::InvalidHex => write!(f, "hex string was invalid"),
            WhiteflagError::InvalidPattern => write!(f, "pattern was invalid"),
            WhiteflagError::InvalidLength => write!(f, "length was too short"),
            WhiteflagError::CannotRetrieveKey => {
                write!(f, "cannot retrieve encryption key for method")
            }
        }
    }
}

pub type WhiteflagResult<T> = Result<T, WhiteflagError>;
