use crate::cbor_value::CborValue;
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH};
use crate::traits::{From, RegistryItem, To};
use crate::types::Fingerprint;
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;

const COMPONENTS: i128 = 1;
const SOURCE_FINGERPRINT: i128 = 2;
const DEPTH: i128 = 3;

#[derive(Copy, Clone, Debug)]
pub struct PathComponent {
    index: Option<u32>,
    wildcard: bool,
    hardened: bool,
}

impl PathComponent {
    pub const HARDEN_BIT: u32 = 0x80000000;
    pub fn new(index: Option<u32>, hardened: bool) -> Result<PathComponent, String> {
        match index {
            Some(x) => {
                if x & PathComponent::HARDEN_BIT != 0 {
                    return Err(format!(
                        "Invalid index {} - most significant bit cannot be set",
                        x
                    ));
                }
                Ok(PathComponent {
                    index,
                    wildcard: false,
                    hardened,
                })
            }
            None => Ok(PathComponent {
                index,
                wildcard: true,
                hardened,
            }),
        }
    }

    pub fn get_index(&self) -> Option<u32> {
        self.index.clone()
    }

    pub fn get_canonical_index(&self) -> Option<u32> {
        self.get_index().map(|x| match self.is_hardened() {
            true => x + PathComponent::HARDEN_BIT,
            false => x,
        })
    }

    pub fn is_wildcard(&self) -> bool {
        self.wildcard.clone()
    }

    pub fn is_hardened(&self) -> bool {
        self.hardened.clone()
    }
}

#[derive(Clone, Debug, Default)]
pub struct CryptoKeyPath {
    components: Vec<PathComponent>,
    source_fingerprint: Option<Fingerprint>,
    depth: Option<u32>,
}

impl CryptoKeyPath {
    pub fn new(
        components: Vec<PathComponent>,
        source_fingerprint: Option<Fingerprint>,
        depth: Option<u32>,
    ) -> CryptoKeyPath {
        CryptoKeyPath {
            components,
            source_fingerprint,
            depth,
        }
    }
    pub fn get_components(&self) -> Vec<PathComponent> {
        self.components.clone()
    }
    pub fn get_source_fingerprint(&self) -> Option<Fingerprint> {
        self.source_fingerprint.clone()
    }
    pub fn get_depth(&self) -> Option<u32> {
        self.depth.clone()
    }
    pub fn get_path(&self) -> Option<String> {
        if self.components.len() == 0 {
            return None;
        }
        Some(
            self.components
                .iter()
                .map::<String, fn(&PathComponent) -> String>(|component| {
                    match (component.wildcard, component.hardened) {
                        (true, true) => "*'".to_string(),
                        (true, false) => "*".to_string(),
                        (false, true) => format!("{}'", component.index.unwrap()),
                        (false, false) => format!("{}", component.index.unwrap()),
                    }
                })
                .collect::<Vec<String>>()
                .join("/"),
        )
    }

    pub fn from_path(path: String, fingerprint: Option<Fingerprint>) -> Result<Self, String> {
        let remove_prefix = path.replace("M/", "").replace("m/", "");
        let chunks = remove_prefix.split('/').map(|split| {
            match split.chars().last() {
                Some('\'') => {
                    let mut remove_quote = split.to_string();
                    remove_quote.pop();
                    let index = remove_quote.parse().map_err(|_| format!("Invalid index: {}", remove_quote))?;
                    Ok(PathComponent { hardened: true, index: Some(index), wildcard: false })
                }
                Some(_) => {
                    let num = split.to_string();
                    let index = num.parse().map_err(|_| format!("Invalid index: {}", num))?;
                    Ok(PathComponent { hardened: false, index: Some(index), wildcard: false })
                }
                _ => {
                    Err(format!("Invalid Path"))
                }
            }
        }).collect::<Result<Vec<PathComponent>, String>>()?;
        Ok(CryptoKeyPath { components: chunks, source_fingerprint: fingerprint, depth: None })
    }
}

impl To for CryptoKeyPath {
    fn to_cbor(&self) -> Value {
        let mut map = BTreeMap::<Value, Value>::new();
        let mut components = Vec::<Value>::new();
        self.components.clone().iter().for_each(|component| {
            if component.is_wildcard() {
                components.push(Value::Array(vec![]))
            } else {
                components.push(Value::Integer(component.get_index().unwrap() as i128));
            }
            components.push(Value::Bool(component.is_hardened()))
        });
        map.insert(Value::Integer(COMPONENTS), Value::Array(components));

        match self.source_fingerprint {
            Some(x) => {
                map.insert(
                    Value::Integer(SOURCE_FINGERPRINT),
                    Value::Integer(u32::from_be_bytes(x) as i128),
                );
            }
            None => {}
        }
        match self.depth {
            Some(x) => {
                map.insert(Value::Integer(DEPTH), Value::Integer(x as i128));
            }
            None => {}
        }
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl RegistryItem for CryptoKeyPath {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_KEYPATH
    }
}

impl From<CryptoKeyPath> for CryptoKeyPath {
    fn from_cbor(cbor: Value) -> Result<CryptoKeyPath, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let components = map
            .get_by_integer(COMPONENTS)
            .map_or(Ok(vec![]), |v| v.get_array())?
            .iter()
            .map(|v| v.get_value().clone())
            .collect::<Vec<Value>>()
            .chunks(2)
            .map(|chunk| match chunk.clone() {
                [Value::Array(_), Value::Bool(hardened)] => Ok(PathComponent {
                    index: None,
                    wildcard: true,
                    hardened: hardened.clone(),
                }),
                [Value::Integer(x), Value::Bool(hardened)] => Ok(PathComponent {
                    index: Some(x.clone() as u32),
                    wildcard: false,
                    hardened: hardened.clone(),
                }),
                x => Err(format!("Unexpected value when parsing components: {:?}", x)),
            })
            .collect::<Result<Vec<PathComponent>, String>>()?;
        let source_fingerprint = map
            .get_by_integer(SOURCE_FINGERPRINT)
            .map(|v| v.get_integer())
            .transpose()?
            .map(|v| u32::to_be_bytes(v as u32));
        let depth = map
            .get_by_integer(DEPTH)
            .map(|v| v.get_integer())
            .transpose()?
            .map(|v| v as u32);
        Ok(CryptoKeyPath {
            components,
            source_fingerprint,
            depth,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoKeyPath, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoKeyPath::from_cbor(value)
    }
}
