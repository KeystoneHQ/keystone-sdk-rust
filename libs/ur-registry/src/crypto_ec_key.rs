use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_ECKEY};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::convert::From;
use minicbor::data::Int;

const CURVE: u8 = 1;
const PRIVATE: u8 = 2;
const DATA: u8 = 3;

#[derive(Default, Clone, Debug)]
pub struct CryptoECKey {
    curve: Option<i128>,
    is_private_key: Option<bool>,
    data: Bytes,
}

impl CryptoECKey {
    pub fn default() {
        Default::default()
    }

    pub fn new(curve: Option<i128>, is_private_key: Option<bool>, data: Bytes) -> Self {
        CryptoECKey {
            curve,
            is_private_key,
            data,
        }
    }

    pub fn set_curve(&mut self, curve: i128) {
        self.curve = Some(curve)
    }

    pub fn set_is_private_key(&mut self, flag: bool) {
        self.is_private_key = Some(flag)
    }

    pub fn set_data(&mut self, data: Bytes) {
        self.data = data;
    }

    pub fn get_curve(&self) -> i128 {
        self.curve.unwrap_or(0)
    }

    pub fn get_is_private_key(&self) -> bool {
        self.is_private_key.unwrap_or(false)
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.data.clone()
    }
}

impl RegistryItem for CryptoECKey {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_ECKEY
    }
}

impl<C> minicbor::Encode<C> for CryptoECKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 1;

        if let Some(_data) = self.curve {
            size += 1;
        }
        if let Some(_data) = self.is_private_key {
            size += 1;
        }
        e.map(size)?;
        if let Some(data) = self.curve {
            e.int(Int::from(CURVE))?.int(
                Int::try_from(data).map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;
        }

        if let Some(data) = self.is_private_key {
            e.int(Int::from(PRIVATE))?.bool(data)?;
        }
        e.int(Int::from(DATA))?.bytes(&self.data)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CryptoECKey {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoECKey {
            curve: None,
            is_private_key: None,
            data: vec![],
        };
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                CURVE => {
                    obj.curve = Some(core::convert::From::from(d.int()?));
                }
                PRIVATE => {
                    obj.is_private_key = Some(d.bool()?);
                }
                DATA => {
                    obj.data = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CryptoECKey {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoECKey> for CryptoECKey {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CryptoECKey> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_ec_key::CryptoECKey;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let crypto_ec_key = CryptoECKey {
            is_private_key: Some(true),
            data: Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap(),
            ..Default::default()
        };
        assert_eq!(
            "A202F50358208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
            hex::encode(crypto_ec_key.to_bytes().unwrap()).to_uppercase()
        );

        let ur = ur::encode(
            &(crypto_ec_key.to_bytes().unwrap()),
            CryptoECKey::get_registry_type().get_type(),
        );
        assert_eq!(ur, "ur:crypto-eckey/oeaoykaxhdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkrphsmyid");
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "A202F50358208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
        )
        .unwrap();
        let crypto_ec_key = CryptoECKey::from_cbor(bytes).unwrap();
        assert_eq!(crypto_ec_key.get_curve(), 0);
        assert!(crypto_ec_key.get_is_private_key());
        assert_eq!(
            crypto_ec_key.get_data(),
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap()
        );
    }
}
