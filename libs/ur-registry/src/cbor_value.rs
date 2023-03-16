use crate::types::Bytes;
use serde_cbor::Value;
use std::collections::BTreeMap;

pub struct CborValue {
    value: Value,
}

pub struct CborValueMap(BTreeMap<Value, CborValue>);

impl CborValueMap {
    pub fn get_by_integer(&self, key: i128) -> Option<&CborValue> {
        self.0.get(&Value::Integer(key))
    }
}

impl CborValue {
    pub fn new(value: Value) -> CborValue {
        CborValue { value }
    }

    pub fn get_value(&self) -> &Value {
        &self.value
    }

    pub fn get_text(&self) -> Result<String, String> {
        match self.value.clone() {
            Value::Text(value) => Ok(value),
            x => Err(format!(
                "unexpected data when decoding Value::Text: {:?}",
                x
            )),
        }
    }

    pub fn get_integer(&self) -> Result<i128, String> {
        match self.value.clone() {
            Value::Integer(value) => Ok(value),
            x => Err(format!(
                "unexpected data when decoding Value::Integer: {:?}",
                x
            )),
        }
    }

    pub fn get_bytes(&self) -> Result<Bytes, String> {
        match self.value.clone() {
            Value::Bytes(value) => Ok(value),
            x => Err(format!(
                "unexpected data when decoding Value::Bytes: {:?}",
                x
            )),
        }
    }

    pub fn get_bool(&self) -> Result<bool, String> {
        match self.value.clone() {
            Value::Bool(value) => Ok(value),
            x => Err(format!(
                "unexpected data when decoding Value::Bool: {:?}",
                x
            )),
        }
    }

    pub fn get_float(&self) -> Result<f64, String> {
        match self.value.clone() {
            Value::Float(value) => Ok(value),
            x => Err(format!(
                "unexpected data when decoding Value::Float: {:?}",
                x
            )),
        }
    }

    pub fn get_array(&self) -> Result<Vec<CborValue>, String> {
        match self.value.clone() {
            Value::Array(value) => Ok(value
                .iter()
                .map(|x| CborValue { value: x.clone() })
                .collect()),
            x => Err(format!(
                "unexpected data when decoding Value::Array: {:?}",
                x
            )),
        }
    }

    pub fn get_map(&self) -> Result<CborValueMap, String> {
        match self.value.clone() {
            Value::Map(value) => Ok(CborValueMap(
                value
                    .iter()
                    .map(|(x1, x2)| (x1.clone(), CborValue { value: x2.clone() }))
                    .collect(),
            )),
            x => Err(format!("unexpected data when decoding Value::Map: {:?}", x)),
        }
    }

    pub fn get_tag(&self, tag: u64) -> Result<CborValue, String> {
        match self.value.clone() {
            Value::Tag(x, y) => {
                if x != tag {
                    Err(format!(
                        "unexpected tag when decoding Value::Tag: received: {:?}, expected: {:?}",
                        x, tag
                    ))
                } else {
                    Ok(CborValue { value: *y.clone() })
                }
            }
            x => Err(format!("unexpected data when decoding Value::Tag: {:?}", x)),
        }
    }
}
