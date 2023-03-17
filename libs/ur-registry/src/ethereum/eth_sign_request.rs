use crate::cbor_value::CborValue;
use crate::crypto_key_path::CryptoKeyPath;
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, UUID, ETH_SIGN_REQUEST};
use crate::traits::{From, RegistryItem, To};
use crate::types::Bytes;
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;

const REQUEST_ID: i128 = 1;
const SIGN_DATA: i128 = 2;
const DATA_TYPE: i128 = 3;
const CHAIN_ID: i128 = 4;
const DERIVATION_PATH: i128 = 5;
const ADDRESS: i128 = 6;
const ORIGIN: i128 = 7;

#[derive(Clone, Debug)]
pub enum DataType {
    Transaction = 1,
    TypedData = 2,
    PersonalMessage = 3,
    TypedTransaction = 4,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::Transaction
    }
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Transaction),
            2 => Ok(DataType::TypedData),
            3 => Ok(DataType::PersonalMessage),
            4 => Ok(DataType::TypedTransaction),
            x => Err(format!(
                "invalid value for data_type in eth-sign-request, expected (1, 2, 3, 4), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct EthSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    data_type: DataType,
    chain_id: Option<i128>,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>,
}

impl EthSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = data_type
    }

    pub fn set_chain_id(&mut self, chain_id: i128) { self.chain_id = Some(chain_id) }

    pub fn set_derivation_path(&mut self, derivation_path: CryptoKeyPath) {
        self.derivation_path = derivation_path;
    }

    pub fn set_address(&mut self, address: Bytes) {
        self.address = Some(address)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        request_id: Option<Bytes>,
        sign_data: Bytes,
        data_type: DataType,
        chain_id: Option<i128>,
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
    ) -> EthSignRequest {
        EthSignRequest {
            request_id,
            sign_data,
            data_type,
            chain_id,
            derivation_path,
            address,
            origin,
        }
    }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
    pub fn get_chain_id(&self) -> Option<i128> { self.chain_id.clone() }
    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }
    pub fn get_address(&self) -> Option<Bytes> {
        self.address.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }
}

impl RegistryItem for EthSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_SIGN_REQUEST
    }
}

impl To for EthSignRequest {
    fn to_cbor(&self) -> Value {
        let mut map: BTreeMap<Value, Value> = BTreeMap::new();
        self.get_request_id().and_then(|id| {
            map.insert(
                Value::Integer(REQUEST_ID),
                Value::Tag(UUID.get_tag() as u64, Box::new(Value::Bytes(id))),
            )
        });
        map.insert(
            Value::Integer(SIGN_DATA),
            Value::Bytes(self.get_sign_data()),
        );
        map.insert(
            Value::Integer(DATA_TYPE),
            Value::Integer(self.get_data_type() as i128),
        );
        self.get_chain_id().and_then(|chain_id| map.insert(Value::Integer(CHAIN_ID), Value::Integer(chain_id)));
        map.insert(
            Value::Integer(DERIVATION_PATH),
            Value::Tag(
                CryptoKeyPath::get_registry_type().get_tag() as u64,
                Box::new(self.get_derivation_path().to_cbor()),
            ),
        );
        self.get_address()
            .and_then(|address| map.insert(Value::Integer(ADDRESS), Value::Bytes(address)));
        self.get_origin()
            .and_then(|origin| map.insert(Value::Integer(ORIGIN), Value::Text(origin)));
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<EthSignRequest> for EthSignRequest {
    fn from_cbor(cbor: Value) -> Result<EthSignRequest, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let request_id = map
            .get_by_integer(REQUEST_ID)
            .map(|v| v.get_tag(UUID.get_tag()).and_then(|v| v.get_bytes()))
            .transpose()?;
        let sign_data = map.get_by_integer(SIGN_DATA).map_or(
            Err("sign_data is required for sol-sign-request".to_string()),
            |v| v.get_bytes(),
        )?;
        let data_type = map.get_by_integer(DATA_TYPE)
            .map_or(Err("data_type is required for eth-sign-request".to_string()), |v| v.get_integer())
            .and_then(|v| match v {
                1 => Ok(DataType::Transaction),
                2 => Ok(DataType::TypedData),
                3 => Ok(DataType::PersonalMessage),
                4 => Ok(DataType::TypedTransaction),
                x => Err(format!(
                    "invalid value for data_type in eth-sign-request, expected (1, 2, 3, 4), received {:?}",
                    x
                )),
            })?;
        let chain_id = map.get_by_integer(CHAIN_ID).map(|v| v.get_integer()).transpose()?;
        let derivation_path = map.get_by_integer(DERIVATION_PATH).map_or(
            Err("derivation_path is required for sol-sign-request".to_string()),
            |v| {
                v.get_tag(CRYPTO_KEYPATH.get_tag())
                    .and_then(|v| CryptoKeyPath::from_cbor(v.get_value().clone()))
            },
        )?;
        let address = map
            .get_by_integer(ADDRESS)
            .map(|v| v.get_bytes())
            .transpose()?;
        let origin = map
            .get_by_integer(ORIGIN)
            .map(|v| v.get_text())
            .transpose()?;

        Ok(EthSignRequest {
            request_id,
            sign_data,
            data_type,
            chain_id,
            derivation_path,
            address,
            origin,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<EthSignRequest, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        EthSignRequest::from_cbor(value)
    }
}
