use crate::error::URResult;
use crate::registry_types::RegistryType;
use alloc::vec::Vec;

pub trait From<T> {
    #[deprecated(since = "0.2.0", note = "please use `try_from` instead")]
    fn from_cbor(bytes: Vec<u8>) -> URResult<T>;
}

pub trait To {
    #[deprecated(since = "0.2.0", note = "please use `try_into` instead")]
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
    fn to_ur_encoder(&self, max_fragment_length: usize) -> ur::Encoder {
        let message = self.to_bytes().unwrap();
        ur::Encoder::new(
            message.as_slice(),
            max_fragment_length,
            N::get_registry_type().get_type(),
        )
        .unwrap()
    }
}

pub trait MapSize {
    fn map_size(&self) -> u64;
}
