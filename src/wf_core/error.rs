use std::fmt;

#[derive(Debug)]
pub enum WhiteflagError {
    InvalidPattern,
    InvalidLength,
    CannotRetrieveKey,
}

impl std::error::Error for WhiteflagError {}

impl fmt::Display for WhiteflagError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InvalidPattern => write!(f, "pattern was invalid"),
            Self::InvalidLength => write!(f, "length was too short"),
            Self::CannotRetrieveKey => {
                write!(f, "cannot retrieve encryption key for method")
            }
        }
    }
}

pub type WhiteflagResult<T> = Result<T, WhiteflagError>;
