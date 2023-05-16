use crate::error::{URError, URResult};
use crate::registry_types::RegistryType;
use crate::registry_types::BYTES as BYTES_TYPE;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes as bytes;

#[derive(Debug, Clone, Default)]
pub struct Bytes(bytes);

impl Bytes {
    pub fn new(bytes: bytes) -> Self {
        Bytes(bytes)
    }

    pub fn get_bytes(&self) -> bytes {
        self.0.clone()
    }

    pub fn set_bytes(&mut self, bytes: bytes) {
        self.0 = bytes;
    }
}

impl RegistryItem for Bytes {
    fn get_registry_type() -> RegistryType<'static> {
        BYTES_TYPE
    }
}

impl<C> minicbor::Encode<C> for Bytes {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.0)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for Bytes {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        Ok(Self(d.bytes()?.to_vec()))
    }
}

impl To for Bytes {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<Bytes> for Bytes {
    fn from_cbor(bytes: Vec<u8>) -> URResult<Bytes> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::bytes::Bytes;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let crypto = Bytes(
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap(),
        );
        assert_eq!(
            "58208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
            hex::encode(crypto.to_bytes().unwrap()).to_uppercase()
        );
        let ur = ur::encode(
            &(crypto.to_bytes().unwrap()),
            Bytes::get_registry_type().get_type(),
        );
        assert_eq!(
            ur,
            "ur:bytes/hdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkvoonhknt"
        );
    }

    #[test]
    fn test_decode() {
        let part =
            "ur:bytes/hdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkvoonhknt";
        let decode_data = ur::decode(part);
        let crypto = Bytes::from_cbor(decode_data.unwrap().1).unwrap();
        assert_eq!(
            crypto.get_bytes(),
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap()
        );
    }
}
