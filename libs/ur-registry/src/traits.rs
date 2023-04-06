use alloc::vec::Vec;
use crate::registry_types::RegistryType;
use ur::Encoder;
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

impl<N> UR for N
where
    N: To + RegistryItem,
{
    fn to_ur_encoder(&self, max_fragment_length: usize) -> Encoder {
        let message = self.to_bytes().unwrap();
        ur::Encoder::new(
            message.as_slice(),
            max_fragment_length,
            N::get_registry_type().get_type(),
        )
        .unwrap()
    }
}
