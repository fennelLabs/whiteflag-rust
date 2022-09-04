use std::str::Utf8Error;

#[derive(thiserror::Error, Debug, PartialEq)]
pub enum CodecError {
    #[error("the buffer is not UTF8 formatted")]
    UTF8(#[from] Utf8Error),
}

pub type CodecResult<T> = Result<T, CodecError>;
