use crate::cbor_value::CborValue;
use crate::crypto_key_path::CryptoKeyPath;
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, SOL_SIGN_REQUEST, UUID};
use crate::traits::{From, RegistryItem, To};
use crate::types::Bytes;
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;

const REQUEST_ID: i128 = 1;
const SIGN_DATA: i128 = 2;
const DERIVATION_PATH: i128 = 3;
const ADDRESS: i128 = 4;
const ORIGIN: i128 = 5;
const SIGN_TYPE: i128 = 6;

#[derive(Clone, Debug)]
pub enum SignType {
    Transaction = 1,
    Message,
}

impl Default for SignType {
    fn default() -> Self {
        SignType::Transaction
    }
}

impl SignType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(SignType::Transaction),
            2 => Ok(SignType::Message),
            x => Err(format!(
                "invalid value for sign_type in sol-sign-request, expected 1 or 2, received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SolSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>,
    sign_type: SignType,
}

impl SolSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_derivation_path(&mut self, derivation_path: CryptoKeyPath) {
        self.derivation_path = derivation_path;
    }

    pub fn set_address(&mut self, address: Bytes) {
        self.address = Some(address)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn set_sign_type(&mut self, sign_type: SignType) {
        self.sign_type = sign_type
    }

    pub fn new(
        request_id: Option<Bytes>,
        sign_data: Bytes,
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
        sign_type: SignType,
    ) -> SolSignRequest {
        SolSignRequest {
            request_id,
            sign_data,
            derivation_path,
            address,
            origin,
            sign_type,
        }
    }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }
    pub fn get_address(&self) -> Option<Bytes> {
        self.address.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }
    pub fn get_sign_type(&self) -> SignType {
        self.sign_type.clone()
    }
}

impl RegistryItem for SolSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        SOL_SIGN_REQUEST
    }
}

impl To for SolSignRequest {
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
        map.insert(
            Value::Integer(SIGN_TYPE),
            Value::Integer(self.get_sign_type() as i128),
        );
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<SolSignRequest> for SolSignRequest {
    fn from_cbor(cbor: Value) -> Result<SolSignRequest, String> {
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
        let sign_type = map.get_by_integer(SIGN_TYPE)
            .map_or(Err("sign_type is required for sol-sign-request".to_string()), |v| v.get_integer())
            .and_then(|v| match v {
                1 => Ok(SignType::Transaction),
                2 => Ok(SignType::Message),
                x => Err(format!("invalid value for sign_type in sol-sign-request, expected 1 or 2, received {:?}", x)),
            })?;
        Ok(SolSignRequest {
            request_id,
            sign_data,
            derivation_path,
            address,
            origin,
            sign_type,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<SolSignRequest, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        SolSignRequest::from_cbor(value)
    }
}
