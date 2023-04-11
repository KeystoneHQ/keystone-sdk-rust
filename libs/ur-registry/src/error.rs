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

    #[error("protobuf decode failed, reason: `{0}`")]
    ProtobufDecodeError(String),

    #[error("protobuf encode failed, reason: `{0}`")]
    ProtobufEncodeError(String),

    #[error("gzip decode failed, reason: `{0}`")]
    GzipDecodeError(String),

    #[error("gzip encode failed, reason: `{0}`")]
    GzipEncodeError(String),
}

pub type URResult<T> = Result<T, URError>;