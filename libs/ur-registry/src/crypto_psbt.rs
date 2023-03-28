use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use crate::error::{URError, UrResult};
use crate::registry_types::{CRYPTO_PSBT, RegistryType};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

#[derive(Debug, Clone, Default)]
pub struct CryptoPSBT {
    psbt: Bytes,
}

impl CryptoPSBT {
    pub fn new(psbt: Bytes) -> Self {
        CryptoPSBT { psbt }
    }

    pub fn get_psbt(&self) -> Bytes {
        self.psbt.clone()
    }

    pub fn set_psbt(&mut self, psbt: Bytes) {
        self.psbt = psbt;
    }
}

impl RegistryItem for CryptoPSBT {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_PSBT
    }
}

impl<C> minicbor::Encode<C> for CryptoPSBT {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.psbt)?;
        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for CryptoPSBT {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        Ok(Self {
            psbt: d.bytes()?.to_vec()
        })
    }
}

impl To for CryptoPSBT {
    fn to_cbor(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoPSBT> for CryptoPSBT {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<CryptoPSBT> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use hex::FromHex;
    use crate::crypto_psbt::CryptoPSBT;

    #[test]
    fn test_encode() {
        let crypto = CryptoPSBT {
            psbt: Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap(),
        };
        assert_eq!(
            "58208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
            hex::encode(crypto.to_cbor().unwrap()).to_uppercase()
        );

        let ur  = ur::encode(&*(crypto.to_cbor().unwrap()), CryptoPSBT::get_registry_type().get_type());
        assert_eq!(ur, "ur:crypto-psbt/hdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkvoonhknt");
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "58208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
        )
            .unwrap();
        let crypto = CryptoPSBT::from_cbor(bytes).unwrap();
        assert_eq!(crypto.get_psbt(), Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa").unwrap());
    }
}