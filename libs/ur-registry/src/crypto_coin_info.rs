use crate::cbor_value::{CborValue, CborValueMap};
use crate::registry_types::{RegistryType, CRYPTO_COIN_INFO};
use crate::traits::{From, RegistryItem, To};
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;

const COIN_TYPE: i128 = 1;
const NETWORK: i128 = 2;

#[derive(Clone, Debug, PartialEq)]
pub enum CoinType {
    Bitcoin = 0,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Network {
    MainNet = 0,
    TestNet = 1,
}

#[derive(Clone, Debug, Default)]
pub struct CryptoCoinInfo {
    coin_type: Option<CoinType>,
    network: Option<Network>,
}

impl CryptoCoinInfo {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_coin_type(&mut self, coin_type: CoinType) {
        self.coin_type = Some(coin_type)
    }

    pub fn set_network(&mut self, network: Network) {
        self.network = Some(network)
    }

    pub fn new(coin_type: Option<CoinType>, network: Option<Network>) -> CryptoCoinInfo {
        CryptoCoinInfo { coin_type, network }
    }
    pub fn get_coin_type(&self) -> CoinType {
        self.coin_type.clone().unwrap_or(CoinType::Bitcoin)
    }
    pub fn get_network(&self) -> Network {
        self.network.clone().unwrap_or(Network::MainNet)
    }
}

impl To for CryptoCoinInfo {
    fn to_cbor(&self) -> Value {
        let mut map = BTreeMap::<Value, Value>::new();
        self.coin_type
            .clone()
            .and_then(|x| map.insert(Value::Integer(COIN_TYPE), Value::Integer(x as i128)));
        self.network
            .clone()
            .and_then(|x| map.insert(Value::Integer(NETWORK), Value::Integer(x as i128)));
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl RegistryItem for CryptoCoinInfo {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_COIN_INFO
    }
}

impl From<CryptoCoinInfo> for CryptoCoinInfo {
    fn from_cbor(cbor: Value) -> Result<CryptoCoinInfo, String> {
        let value = CborValue::new(cbor);
        let map: CborValueMap = value.get_map()?;
        let coin_type = map
            .get_by_integer(COIN_TYPE)
            .map(|v| v.get_integer())
            .transpose()?
            .map(|v| match v {
                0 => CoinType::Bitcoin,
                _ => CoinType::Bitcoin,
            });
        let network = map
            .get_by_integer(NETWORK)
            .map(|v| v.get_integer())
            .transpose()?
            .map(|v| match v {
                0 => Network::MainNet,
                1 => Network::TestNet,
                _ => Network::MainNet,
            });
        Ok(CryptoCoinInfo { coin_type, network })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoCoinInfo, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoCoinInfo::from_cbor(value)
    }
}
