pub type CryptoResult<T> = Result<T, CryptoError>;

#[derive(thiserror::Error, Debug)]
pub enum CryptoError {
    #[error("issue decoding hex string")]
    HexDecode(#[from] hex::FromHexError),
    #[error("invalid psuedo random key length")]
    HkdfInput(hkdf::InvalidPrkLength),
    #[error("invalid output length")]
    HkdfOutput(hkdf::InvalidLength),
    #[error("invalid crypto method")]
    InvalidMethod,
    #[error(transparent)]
    Other(#[from] Box<dyn std::error::Error>),
}

/*
WhiteflagCryptoError::InvalidCipher => {
    write!(f, "Context and/or initialisation vector have not been set")
}
 */
