use alloc::string::String;
use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum URError {
    #[error("cbor decode failed, reason: `{0}`")]
    CborDecodeError(String),

    #[error("cbor encode failed, reason: `{0}`")]
    CborEncodeError(String),
}

pub type URResult<T> = Result<T, URError>;