use std::collections::BTreeMap;
use serde_cbor::{from_slice, to_vec, Value};
use crate::cbor_value::CborValue;
use crate::crypto_output::CryptoOutput;
use crate::registry_types::{CRYPTO_ACCOUNT, RegistryType};
use crate::traits::{RegistryItem, To, From};
use crate::types::{Fingerprint};

const MASTER_FINGERPRINT: i128 = 1;
const OUTPUT_DESCRIPTORS: i128 = 2;

#[derive(Clone, Debug, Default)]
pub struct CryptoAccount {
    master_fingerprint: Fingerprint,
    output_descriptors: Vec<CryptoOutput>,
}

impl RegistryItem for CryptoAccount {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_ACCOUNT
    }
}

impl CryptoAccount {
    pub fn new(master_fingerprint: Fingerprint, output_descriptors: Vec<CryptoOutput>) -> Self {
        CryptoAccount {
            master_fingerprint,
            output_descriptors,
        }
    }

    pub fn get_master_fingerprint(&self) -> Fingerprint {
        self.master_fingerprint.clone()
    }

    pub fn get_output_descriptors(&self) -> Vec<CryptoOutput> {
        self.output_descriptors.clone()
    }

    pub fn set_master_fingerprint(&mut self, fingerprint: Fingerprint) {
        self.master_fingerprint = fingerprint;
    }

    pub fn set_output_descriptors(&mut self, outputs: Vec<CryptoOutput>) {
        self.output_descriptors = outputs;
    }
}

impl To for CryptoAccount {
    fn to_cbor(&self) -> Value {
        let mut map: BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(Value::Integer(MASTER_FINGERPRINT), Value::Integer(u32::from_be_bytes(self.master_fingerprint) as i128));
        map.insert(Value::Integer(OUTPUT_DESCRIPTORS), Value::Array(self.output_descriptors.iter().map(|v| v.to_cbor()).collect()));
        Value::Map(map)
    }
    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<CryptoAccount> for CryptoAccount {
    fn from_cbor(cbor: Value) -> Result<CryptoAccount, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let master_fingerprint =
            map.get_by_integer(MASTER_FINGERPRINT).map(|v| v.get_integer())
                .transpose()?
                .map(|v| u32::to_be_bytes(v as u32))
                .ok_or("master fingerprint is required for crypto-account".to_string())?;
        let outputs = map.get_by_integer(OUTPUT_DESCRIPTORS).map(|v| v.get_array())
            .transpose()?
            .ok_or("output descriptors are required for crypto-account".to_string())?
            .iter().map(|v| CryptoOutput::from_cbor(v.get_value().clone())).collect::<Result<Vec<CryptoOutput>, String>>()?;

        Ok(CryptoAccount {
            master_fingerprint,
            output_descriptors: outputs,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoAccount, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoAccount::from_cbor(value)
    }
}
