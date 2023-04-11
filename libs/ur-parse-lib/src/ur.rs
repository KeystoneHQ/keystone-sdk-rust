use alloc::vec::Vec;
use ur_registry::error::URResult;
use ur_registry::registry_types::URType;
use ur_registry::traits::From;

pub struct UR {
    ur_type: URType,
    data: Vec<u8>,
}

impl UR {
    pub fn new(ur_type: URType, data: Vec<u8>) -> Self {
        UR { ur_type, data }
    }

    pub fn parse<T: From<T>> (&self) -> URResult<(URType, T)> {
        let result = T::from_cbor(self.data.clone())?;
        Ok((self.ur_type.clone(), result))
    }

}