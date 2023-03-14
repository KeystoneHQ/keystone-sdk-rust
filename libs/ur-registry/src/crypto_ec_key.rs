use crate::cbor_value::CborValue;
use crate::registry_types::{RegistryType, CRYPTO_ECKEY};
use crate::traits::{From, RegistryItem, To};
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;
use crate::types::Bytes;

const CURVE: i128 = 1;
const PRIVATE: i128 = 2;
const DATA: i128 = 3;

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
        match self.curve {
            Some(x) => x,
            None => 0,
        }
    }

    pub fn get_is_private_key(&self) -> bool {
        match self.is_private_key {
            Some(x) => x,
            None => false,
        }
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

impl To for CryptoECKey {
    fn to_cbor(&self) -> Value {
        let mut map = BTreeMap::<Value, Value>::new();
        match self.curve {
            Some(x) => {
                map.insert(Value::Integer(CURVE), Value::Integer(x));
            }
            None => {}
        }
        match self.is_private_key {
            Some(x) => {
                map.insert(Value::Integer(PRIVATE), Value::Bool(x));
            }
            None => {}
        }
        map.insert(Value::Integer(DATA), Value::Bytes(self.data.clone()));
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<CryptoECKey> for CryptoECKey {
    fn from_cbor(cbor: Value) -> Result<CryptoECKey, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let curve = map
            .get_by_integer(CURVE)
            .map(|v| v.get_integer())
            .transpose()?;
        let is_private_key = map
            .get_by_integer(PRIVATE)
            .map(|v| v.get_bool())
            .transpose()?;
        let data = map
            .get_by_integer(DATA)
            .map_or(Ok(vec![]), |v| v.get_bytes())?;
        Ok(CryptoECKey {
            curve,
            is_private_key,
            data,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoECKey, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoECKey::from_cbor(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_ec_key::CryptoECKey;
    use crate::traits::{From, To, UR};
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
            hex::encode(crypto_ec_key.to_bytes()).to_uppercase()
        );

        let mut encoder = crypto_ec_key.to_ur_encoder(1000);
        let ur = encoder.next_part().unwrap();
        assert_eq!(ur, "ur:crypto-eckey/oeaoykaxhdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkrphsmyid");
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "A202F50358208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA",
        )
            .unwrap();
        let crypto_ec_key = CryptoECKey::from_bytes(bytes).unwrap();
        assert_eq!(crypto_ec_key.get_curve(), 0);
        assert_eq!(crypto_ec_key.get_is_private_key(), true);
        assert_eq!(
            crypto_ec_key.get_data(),
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap()
        );
    }
}
