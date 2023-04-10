use alloc::vec::Vec;
use crate::registry_types::RegistryType;
use crate::error::URResult;

pub trait From<T> {
    fn from_cbor(bytes: Vec<u8>) -> URResult<T>;
}

pub trait To {
    fn to_bytes(&self) -> URResult<Vec<u8>>;
}

pub trait UR {
    fn to_ur_encoder(&self, max_fragment_length: usize) -> ur::Encoder;
}

pub trait RegistryItem {
    fn get_registry_type() -> RegistryType<'static>;
}
