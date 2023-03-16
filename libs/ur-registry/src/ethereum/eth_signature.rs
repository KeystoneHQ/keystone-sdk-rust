use crate::cbor_value::CborValue;
use crate::registry_types::{ETH_SIGNATURE, RegistryType, UUID};
use crate::traits::{From, RegistryItem, To};
use crate::types::{Bytes, CborMap};
use serde_cbor::{from_slice, to_vec, Value};

const REQUEST_ID: i128 = 1;
const SIGNATURE: i128 = 2;
const ORIGIN: i128 = 3;

#[derive(Clone, Debug, Default)]
pub struct EthSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
    origin: Option<String>,
}

impl EthSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Bytes) {
        self.signature = signature;
    }

    pub fn set_origin(&mut self, origin: String) { self.origin = Some(origin) }

    pub fn new(request_id: Option<Bytes>, signature: Bytes, origin: Option<String>) -> Self {
        EthSignature { request_id, signature, origin }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
    pub fn get_origin(&self) -> Option<String> { self.origin.clone() }
}

impl RegistryItem for EthSignature {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_SIGNATURE
    }
}

impl To for EthSignature {
    fn to_cbor(&self) -> Value {
        let mut map: CborMap = CborMap::new();
        self.get_request_id().and_then(|request_id| {
            map.insert(
                Value::Integer(REQUEST_ID),
                Value::Tag(37, Box::new(Value::Bytes(request_id))),
            )
        });
        map.insert(
            Value::Integer(SIGNATURE),
            Value::Bytes(self.get_signature()),
        );
        self.get_origin().and_then(|origin| {
            map.insert(Value::Integer(ORIGIN), Value::Text(origin))
        });
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<EthSignature> for EthSignature {
    fn from_cbor(cbor: Value) -> Result<EthSignature, String> {
        let cbor_value = CborValue::new(cbor);
        let map = cbor_value.get_map()?;
        let request_id = map
            .get_by_integer(REQUEST_ID)
            .map(|v| v.get_tag(UUID.get_tag()).and_then(|v| v.get_bytes()))
            .transpose()?;
        let signature = map.get_by_integer(SIGNATURE).map_or(
            Err("signature is required for sol-signature".to_string()),
            |r| r.get_bytes(),
        )?;
        let origin = map
            .get_by_integer(ORIGIN)
            .map(|v| v.get_text())
            .transpose()?;
        Ok(EthSignature {
            request_id,
            signature,
            origin
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<EthSignature, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        EthSignature::from_cbor(value)
    }
}
