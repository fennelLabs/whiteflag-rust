use std::fmt;

#[derive(Debug)]
pub enum WhiteflagAccountError {
    CantSetECDHPair,
    CantSetOwnECDHKey,
}

impl std::error::Error for WhiteflagAccountError {}

impl fmt::Display for WhiteflagAccountError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WhiteflagAccountError::CantSetECDHPair => {
                write!(f, "cannot set ECDH key pair on other's account")
            }
            WhiteflagAccountError::CantSetOwnECDHKey => {
                write!(f, "Cannot set ECDH key on own account")
            }
        }
    }
}

pub type WhiteflagAccountResult<T> = Result<T, WhiteflagAccountError>;
