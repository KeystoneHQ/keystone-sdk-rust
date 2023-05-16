use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, KEYSTONE_SIGN_REQUEST};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const SIGN_DATA: u8 = 1;
const ORIGIN: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct KeystoneSignRequest {
    sign_data: Bytes,
    origin: Option<String>,
}

impl KeystoneSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(sign_data: Bytes, origin: Option<String>) -> KeystoneSignRequest {
        KeystoneSignRequest { sign_data, origin }
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 1;
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for KeystoneSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        KEYSTONE_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for KeystoneSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for KeystoneSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = KeystoneSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                ORIGIN => {
                    obj.origin = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for KeystoneSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<KeystoneSignRequest> for KeystoneSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<KeystoneSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::keystone::keystone_sign_request::KeystoneSignRequest;
    use crate::traits::{From as FromCbor, To};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let sign_data = hex::decode("1f8b08000000000000ff554d3f4b23411c256bb36c93d52aa40a8b10092c9999dfecfc812beee21214d6603410926e7e33b345305993dcc5fb187e04bf805c7f1f4041b03bacafbd43eceddc56783c788ff7270c0e9ae3cd71e57ce77c537daf6c75d57e096a371c3218ea61ce92c720da2b26c707871aa9f3c85d0a285dcad1b854a3732943e22482b3285ce7d7dbdfdfefe428c4db207cda8ffff492bb46f4d5641a2d1a633ca5580b501cb867ae66c538f8cc8366594669262c37e095f2129d25c88c2a8580b8d13e8d0684694189408568b4954a122141d61525bc744a010a2d35ad170918414de93595256619f1e8508385d6c3fdff30692efb9c77fbacdb2735faa4d78d9262b6292a9c9f70ab2793ed6a9da3dbde4caf776b3d6363287ecc647c3bea7da983733d5aad0aff53bb3cf7ebc1d6dfecd8e0b254cb6ab1b8f866a610bf3eff0b5b8da479f6f9e603ce1eec266c010000").unwrap();

        let keystone_sign_request =
            KeystoneSignRequest::new(sign_data, Some("ltcWallet".to_string()));
        assert_eq!(
            "a2015901581f8b08000000000000ff554d3f4b23411c256bb36c93d52aa40a8b10092c9999dfecfc812beee21214d6603410926e7e33b345305993dcc5fb187e04bf805c7f1f4041b03bacafbd43eceddc56783c788ff7270c0e9ae3cd71e57ce77c537daf6c75d57e096a371c3218ea61ce92c720da2b26c707871aa9f3c85d0a285dcad1b854a3732943e22482b3285ce7d7dbdfdfefe428c4db207cda8ffff492bb46f4d5641a2d1a633ca5580b501cb867ae66c538f8cc8366594669262c37e095f2129d25c88c2a8580b8d13e8d0684694189408568b4954a122141d61525bc744a010a2d35ad170918414de93595256619f1e8508385d6c3fdff30692efb9c77fbacdb2735faa4d78d9262b6292a9c9f70ab2793ed6a9da3dbde4caf776b3d6363287ecc647c3bea7da983733d5aad0aff53bb3cf7ebc1d6dfecd8e0b254cb6ab1b8f866a610bf3eff0b5b8da479f6f9e603ce1eec266c01000002696c746357616c6c6574",
            hex::encode(keystone_sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a2015901581f8b08000000000000ff554d3f4b23411c256bb36c93d52aa40a8b10092c9999dfecfc812beee21214d6603410926e7e33b345305993dcc5fb187e04bf805c7f1f4041b03bacafbd43eceddc56783c788ff7270c0e9ae3cd71e57ce77c537daf6c75d57e096a371c3218ea61ce92c720da2b26c707871aa9f3c85d0a285dcad1b854a3732943e22482b3285ce7d7dbdfdfefe428c4db207cda8ffff492bb46f4d5641a2d1a633ca5580b501cb867ae66c538f8cc8366594669262c37e095f2129d25c88c2a8580b8d13e8d0684694189408568b4954a122141d61525bc744a010a2d35ad170918414de93595256619f1e8508385d6c3fdff30692efb9c77fbacdb2735faa4d78d9262b6292a9c9f70ab2793ed6a9da3dbde4caf776b3d6363287ecc647c3bea7da983733d5aad0aff53bb3cf7ebc1d6dfecd8e0b254cb6ab1b8f866a610bf3eff0b5b8da479f6f9e603ce1eec266c01000002696c746357616c6c6574",
        ).unwrap();
        let keystone_sign_request = KeystoneSignRequest::from_cbor(bytes).unwrap();

        assert_eq!("1f8b08000000000000ff554d3f4b23411c256bb36c93d52aa40a8b10092c9999dfecfc812beee21214d6603410926e7e33b345305993dcc5fb187e04bf805c7f1f4041b03bacafbd43eceddc56783c788ff7270c0e9ae3cd71e57ce77c537daf6c75d57e096a371c3218ea61ce92c720da2b26c707871aa9f3c85d0a285dcad1b854a3732943e22482b3285ce7d7dbdfdfefe428c4db207cda8ffff492bb46f4d5641a2d1a633ca5580b501cb867ae66c538f8cc8366594669262c37e095f2129d25c88c2a8580b8d13e8d0684694189408568b4954a122141d61525bc744a010a2d35ad170918414de93595256619f1e8508385d6c3fdff30692efb9c77fbacdb2735faa4d78d9262b6292a9c9f70ab2793ed6a9da3dbde4caf776b3d6363287ecc647c3bea7da983733d5aad0aff53bb3cf7ebc1d6dfecd8e0b254cb6ab1b8f866a610bf3eff0b5b8da479f6f9e603ce1eec266c010000", hex::encode(keystone_sign_request.sign_data));
        assert_eq!("ltcWallet", keystone_sign_request.origin.unwrap());
    }
}
