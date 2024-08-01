use std::str::Utf8Error;

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum CodecError {
    #[error("the buffer is not UTF8 formatted")]
    UTF8(#[from] Utf8Error),
    #[error("the buffer is not valid hexadecimal")]
    Hexadecimal(),
    #[error("field definition vector should not be empty")]
    EmptyFieldDefinition(),
    #[error("validation error while converting array of strings into fields: {error:?}")]
    Validation {
        error: String,
    },
}

#[derive(thiserror::Error, Debug, PartialEq, Eq)]
pub enum HexDecodeError {
    #[error("the buffer is not valid hexadecimal")]
    InvalidHexDigit(char),
}

impl From<HexDecodeError> for CodecError {
    fn from(_: HexDecodeError) -> Self {
        Self::Hexadecimal()
    }
}

pub type CodecResult<T> = Result<T, CodecError>;
