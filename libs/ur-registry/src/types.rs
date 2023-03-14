use serde_cbor::Value;
use std::collections::BTreeMap;

pub type Bytes = Vec<u8>;
pub type Fingerprint = [u8; 4];
pub type CborMap = BTreeMap<Value, Value>;
