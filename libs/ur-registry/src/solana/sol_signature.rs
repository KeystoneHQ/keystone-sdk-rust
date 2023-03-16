use crate::cbor_value::CborValue;
use crate::registry_types::{RegistryType, SOL_SIGNATURE, UUID};
use crate::traits::{From, RegistryItem, To};
use crate::types::{Bytes, CborMap};
use serde_cbor::{from_slice, to_vec, Value};

const REQUEST_ID: i128 = 1;
const SIGNATURE: i128 = 2;

#[derive(Clone, Debug, Default)]
pub struct SolSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
}

impl SolSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Bytes) {
        self.signature = signature;
    }

    pub fn new(request_id: Option<Bytes>, signature: Bytes) -> Self {
        SolSignature { request_id, signature }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
}

impl RegistryItem for SolSignature {
    fn get_registry_type() -> RegistryType<'static> {
        SOL_SIGNATURE
    }
}

impl To for SolSignature {
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
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<SolSignature> for SolSignature {
    fn from_cbor(cbor: Value) -> Result<SolSignature, String> {
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
        Ok(SolSignature {
            request_id,
            signature,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<SolSignature, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        SolSignature::from_cbor(value)
    }
}
