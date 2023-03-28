use thiserror_no_std::Error;
use alloc::string::String;

#[derive(Error, Debug, PartialEq)]
pub enum URError {
    #[error("cbor decode failed, reason: `{0}`")]
    CborDecodeError(String),

    #[error("cbor encode failed, reason: `{0}`")]
    CborEncodeError(String),
}

pub type UrResult<T> = Result<T, URError>;