use std::fmt;

#[derive(Debug)]
pub enum WhiteflagCreatorError {
    InvalidHeaderField,
    InvalidBodyField,
}

impl std::error::Error for WhiteflagCreatorError {}

impl fmt::Display for WhiteflagCreatorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagCreatorError::InvalidHeaderField => write!(f, "header fieldname-to-value mapping contains invalid field names and/or values"),
            WhiteflagCreatorError::InvalidBodyField => write!(f, "body fieldname-to-value mapping contains invalid field names and/or values"),
        }
    }
}

pub type WhiteflagResult<T> = Result<T, WhiteflagCreatorError>;
