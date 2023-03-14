use serde_cbor::{from_slice, to_vec, Value};
use crate::cbor_value::CborValue;
use crate::crypto_hd_key::CryptoHDKey;
use crate::registry_types::{CRYPTO_OUTPUT, RegistryType};
use crate::traits::{RegistryItem, To, From};

#[derive(Clone, Debug, Default)]
pub struct CryptoOutput {
    //TODO: add script_expressions
    crypto_key: CryptoHDKey, // crypto_key: CryptoHDKey, CryptoECKey, MultiKey
}

impl CryptoOutput {
    pub fn new(crypto_key: CryptoHDKey) -> Self {
        CryptoOutput {
            crypto_key
        }
    }

    pub fn get_crypto_key(&self) -> CryptoHDKey {
        self.crypto_key.clone()
    }

    pub fn set_crypto_key(&mut self, crypto_key: CryptoHDKey) {
        self.crypto_key = crypto_key;
    }
}

impl RegistryItem for CryptoOutput {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_OUTPUT
    }
}

impl To for CryptoOutput {
    fn to_cbor(&self) -> Value {
        let key = self.get_crypto_key().to_cbor();
        Value::Tag(CryptoHDKey::get_registry_type().get_tag(), Box::new(key))
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        //TODO: remove unwrap
        to_vec(&value).unwrap()
    }
}

impl From<CryptoOutput> for CryptoOutput {
    fn from_cbor(cbor: Value) -> Result<CryptoOutput, String> {
        let value = CborValue::new(cbor);
        let tag = value.get_tag(CryptoHDKey::get_registry_type().get_tag())?;
        let key = CryptoHDKey::from_cbor(tag.get_value().clone())?;
        Ok(CryptoOutput {
            crypto_key: key
        })
    }
    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoOutput, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoOutput::from_cbor(value)
    }
}