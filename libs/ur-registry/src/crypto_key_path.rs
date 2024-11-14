use crate::cbor::{cbor_array, cbor_map, cbor_type};
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Fingerprint;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::{format, vec};
use minicbor::data::{Int, Type};
use minicbor::encode::Write;
use minicbor::Encoder;

const COMPONENTS: u8 = 1;
const SOURCE_FINGERPRINT: u8 = 2;
const DEPTH: u8 = 3;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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
        self.index
    }

    pub fn get_canonical_index(&self) -> Option<u32> {
        self.get_index().map(|x| match self.is_hardened() {
            true => x + PathComponent::HARDEN_BIT,
            false => x,
        })
    }

    pub fn is_wildcard(&self) -> bool {
        self.wildcard
    }

    pub fn is_hardened(&self) -> bool {
        self.hardened
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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
        self.source_fingerprint
    }
    pub fn get_depth(&self) -> Option<u32> {
        self.depth
    }
    pub fn get_path(&self) -> Option<String> {
        if self.components.is_empty() {
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
        let chunks = remove_prefix
            .split('/')
            .map(|split| match split.chars().last() {
                Some('\'') => {
                    let mut remove_quote = split.to_string();
                    remove_quote.pop();
                    let index = remove_quote
                        .parse()
                        .map_err(|_| format!("Invalid index: {}", remove_quote))?;
                    Ok(PathComponent {
                        hardened: true,
                        index: Some(index),
                        wildcard: false,
                    })
                }
                Some(_) => {
                    let num = split.to_string();
                    let index = num.parse().map_err(|_| format!("Invalid index: {}", num))?;
                    Ok(PathComponent {
                        hardened: false,
                        index: Some(index),
                        wildcard: false,
                    })
                }
                _ => Err("Invalid Path".to_string()),
            })
            .collect::<Result<Vec<PathComponent>, String>>()?;
        Ok(CryptoKeyPath {
            components: chunks,
            source_fingerprint: fingerprint,
            depth: None,
        })
    }
}

impl RegistryItem for CryptoKeyPath {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_KEYPATH
    }
}

impl<C> minicbor::Encode<C> for CryptoKeyPath {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 1;
        if let Some(_data) = self.source_fingerprint {
            size += 1;
        }
        if let Some(_data) = self.depth {
            size += 1;
        }
        e.map(size)?;
        e.int(
            Int::try_from(COMPONENTS)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .array(2 * self.components.len() as u64)?;
        for component in self.components.iter() {
            if component.is_wildcard() {
                e.array(0)?;
            } else {
                match component.index {
                    Some(index) => {
                        e.int(Int::from(index))?;
                    }
                    None => {
                        e.int(Int::from(0))?;
                    }
                }
            }
            e.bool(component.is_hardened())?;
        }

        if let Some(source_fingerprint) = self.source_fingerprint {
            e.int(
                Int::try_from(SOURCE_FINGERPRINT)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?
            .int(
                Int::try_from(u32::from_be_bytes(source_fingerprint))
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;
        }

        if let Some(depth) = self.depth {
            e.int(
                Int::try_from(DEPTH)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?
            .int(
                Int::try_from(depth)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CryptoKeyPath {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoKeyPath::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                COMPONENTS => {
                    let mut path_component: Vec<PathComponent> = vec![];
                    let mut hardened = false;
                    let mut previous_type: Type = Type::Null;
                    let mut path_index: Option<u32> = None;
                    cbor_array(d, obj, |_index, _obj, d| {
                        let data_type = cbor_type(d.datatype()?);
                        match data_type {
                            Type::Array => {
                                d.array()?;
                                previous_type = Type::Array;
                            }
                            Type::Int => {
                                path_index = Some(u32::try_from(d.int()?).map_err(|e| {
                                    minicbor::decode::Error::message(e.to_string())
                                })?);
                                previous_type = Type::Int;
                            }
                            Type::Bool => {
                                hardened = d.bool()?;
                                match previous_type {
                                    Type::Array => {
                                        path_component
                                            .push(PathComponent::new(None, hardened).map_err(
                                                |e| minicbor::decode::Error::message(e),
                                            )?);
                                    }
                                    Type::Int => {
                                        path_component.push(
                                            PathComponent::new(path_index, hardened)
                                                .map_err(minicbor::decode::Error::message)?,
                                        );
                                    }
                                    _ => {}
                                }
                            }
                            _ => {}
                        }
                        Ok(())
                    })?;
                    obj.components = path_component;
                }
                SOURCE_FINGERPRINT => {
                    obj.source_fingerprint =
                        Some(u32::to_be_bytes(u32::try_from(d.int()?).map_err(|e| {
                            minicbor::decode::Error::message(e.to_string())
                        })?));
                }
                DEPTH => {
                    obj.depth = Some(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    );
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CryptoKeyPath {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoKeyPath> for CryptoKeyPath {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CryptoKeyPath> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(118), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(None, false).unwrap();

        let source_fingerprint: [u8; 4] = [120, 35, 8, 4];
        let crypto_key_path = CryptoKeyPath {
            components: vec![path1, path2, path3, path4, path5],
            source_fingerprint: Some(source_fingerprint),
            depth: Some(5),
        };

        assert_eq!(
            "A3018A182CF51876F500F500F480F4021A782308040305",
            hex::encode(crypto_key_path.to_bytes().unwrap()).to_uppercase()
        );

        let ur = ur::encode(
            &(crypto_key_path.to_bytes().unwrap()),
            CryptoKeyPath::get_registry_type().get_type(),
        );
        assert_eq!(
            ur,
            "ur:crypto-keypath/otadlecsdwykcskoykaeykaewklawkaocykscnayaaaxahrybsckoe"
        );

        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(118), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(Some(0), false).unwrap();

        let source_fingerprint: [u8; 4] = [120, 35, 8, 4];
        let crypto_key_path = CryptoKeyPath {
            components: vec![path1, path2, path3, path4, path5],
            source_fingerprint: Some(source_fingerprint),
            depth: Some(5),
        };

        assert_eq!(
            "A3018A182CF51876F500F500F400F4021A782308040305",
            hex::encode(crypto_key_path.to_bytes().unwrap()).to_uppercase()
        );

        let ur = ur::encode(
            &(crypto_key_path.to_bytes().unwrap()),
            CryptoKeyPath::get_registry_type().get_type(),
        );
        assert_eq!(
            ur,
            "ur:crypto-keypath/otadlecsdwykcskoykaeykaewkaewkaocykscnayaaaxahhpbkchot"
        );
    }

    #[test]
    fn test_decode() {
        let hex = "a3018a182cf51876f500f500f480f4021a782308040305"; //a3018a182cf51876f500f500f480f4021a782308040305
        let bytes = Vec::from_hex(hex).unwrap();

        let crypto = CryptoKeyPath::from_cbor(bytes).unwrap();
        assert_eq!(crypto.get_depth().unwrap(), 5);
        assert_eq!(crypto.get_source_fingerprint().unwrap(), [120, 35, 8, 4]);
        assert_eq!(crypto.get_path().unwrap(), "44'/118'/0'/0/*");

        let hex = "a3018a182cf51876f500f500f400f4021a782308040305";
        let bytes = Vec::from_hex(hex).unwrap();
        let crypto = CryptoKeyPath::from_cbor(bytes).unwrap();
        assert_eq!(crypto.get_path().unwrap(), "44'/118'/0'/0/0");
    }
}
