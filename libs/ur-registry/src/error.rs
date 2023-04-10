use alloc::string::String;
use thiserror::Error;


#[derive(Error, Debug, PartialEq)]
pub enum URError {
    #[error("cbor decode failed, reason: `{0}`")]
    CborDecodeError(String),

    #[error("cbor encode failed, reason: `{0}`")]
    CborEncodeError(String),

    #[error("ur decode failed, reason: `{0}`")]
    UrDecodeError(String),

    #[error("ur encode failed, reason: `{0}`")]
    UrEncodeError(String),

    #[error("not support this type: `{0}`")]
    NotSupportURTypeError(String),

    #[error("not a ur")]
    NotAUr,

    #[error("not specified type")]
    TypeUnspecified,
}

pub type URResult<T> = Result<T, URError>;