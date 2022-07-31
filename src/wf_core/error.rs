use std::fmt;

#[derive(Debug)]
pub enum WhiteflagError {
    InvalidPattern,
    InvalidLength,
    InvalidHeaderField,
    InvalidBodyField,
}

impl std::error::Error for WhiteflagError {}

impl fmt::Display for WhiteflagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagError::InvalidPattern => write!(f, "pattern was invalid"),
            WhiteflagError::InvalidLength => write!(f, "length was too short"),
            WhiteflagError::InvalidHeaderField => write!(f, "header fieldname-to-value mapping contains invalid field names and/or values"),
            WhiteflagError::InvalidBodyField => write!(f, "body fieldname-to-value mapping contains invalid field names and/or values"),
        }
    }
}

pub type WhiteflagResult<T> = Result<T, WhiteflagError>;
