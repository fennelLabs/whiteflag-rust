use std::fmt;

#[derive(Debug)]
pub enum WhiteflagError {
    InvalidPattern,
    InvalidLength,
}

impl std::error::Error for WhiteflagError {}

impl fmt::Display for WhiteflagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagError::InvalidPattern => write!(f, "pattern was invalid"),
            WhiteflagError::InvalidLength => write!(f, "length was too short"),
        }
    }
}

pub type WhiteflagResult<T> = Result<T, WhiteflagError>;
