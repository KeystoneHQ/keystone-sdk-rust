use crate::registry_types::RegistryType;
use serde_cbor::Value;
use ur::Encoder;

pub trait From<T> {
    fn from_cbor(cbor: Value) -> Result<T, String>;
    fn from_bytes(bytes: Vec<u8>) -> Result<T, String>;
}

pub trait To {
    fn to_cbor(&self) -> Value;
    fn to_bytes(&self) -> Vec<u8>;
}

pub trait UR {
    fn to_ur_encoder(&self, max_fragment_length: usize) -> ur::Encoder;
}

pub trait RegistryItem {
    fn get_registry_type() -> RegistryType<'static>;
}

impl<N> UR for N
where
    N: To + RegistryItem,
{
    fn to_ur_encoder(&self, max_fragment_length: usize) -> Encoder {
        let message = self.to_bytes();
        ur::Encoder::new(
            message.as_slice(),
            max_fragment_length,
            N::get_registry_type().get_type(),
        )
        .unwrap()
    }
}
