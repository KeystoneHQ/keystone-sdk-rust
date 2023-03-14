use crate::cbor_value::CborValue;
use crate::crypto_coin_info::CryptoCoinInfo;
use crate::crypto_key_path::CryptoKeyPath;
use crate::registry_types::{RegistryType, CRYPTO_COIN_INFO, CRYPTO_HDKEY, CRYPTO_KEYPATH};
use crate::traits::{From, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;

const IS_MASTER: i128 = 1;
const IS_PRIVATE: i128 = 2;
const KEY_DATA: i128 = 3;
const CHAIN_CODE: i128 = 4;
const USE_INFO: i128 = 5;
const ORIGIN: i128 = 6;
const CHILDREN: i128 = 7;
const PARENT_FINGERPRINT: i128 = 8;
const NAME: i128 = 9;
const NOTE: i128 = 10;

#[derive(Clone, Debug, Default)]
pub struct CryptoHDKey {
    is_master: Option<bool>,
    is_private_key: Option<bool>,
    key: Bytes,
    chain_code: Option<Bytes>,
    use_info: Option<CryptoCoinInfo>,
    origin: Option<CryptoKeyPath>,
    children: Option<CryptoKeyPath>,
    parent_fingerprint: Option<Fingerprint>,
    name: Option<String>,
    note: Option<String>,
}

impl CryptoHDKey {
    pub fn new_master_key(key: Bytes, chain_code: Bytes) -> CryptoHDKey {
        CryptoHDKey {
            is_master: Some(true),
            key,
            chain_code: Some(chain_code),
            ..Default::default()
        }
    }

    pub fn new_extended_key(
        is_private_key: Option<bool>,
        key: Bytes,
        chain_code: Option<Bytes>,
        use_info: Option<CryptoCoinInfo>,
        origin: Option<CryptoKeyPath>,
        children: Option<CryptoKeyPath>,
        parent_fingerprint: Option<Fingerprint>,
        name: Option<String>,
        note: Option<String>,
    ) -> CryptoHDKey {
        CryptoHDKey {
            is_master: Some(false),
            is_private_key,
            key,
            chain_code,
            use_info,
            origin,
            children,
            parent_fingerprint,
            name,
            note,
        }
    }

    pub fn is_master(&self) -> bool {
        self.is_master.clone().unwrap_or(false)
    }
    pub fn is_private_key(&self) -> bool {
        self.is_private_key.clone().unwrap_or(false)
    }
    pub fn get_key(&self) -> Bytes {
        self.key.clone()
    }
    pub fn get_chain_code(&self) -> Option<Vec<u8>> {
        self.chain_code.clone()
    }
    pub fn get_use_info(&self) -> Option<CryptoCoinInfo> {
        self.use_info.clone()
    }
    pub fn get_origin(&self) -> Option<CryptoKeyPath> {
        self.origin.clone()
    }
    pub fn get_children(&self) -> Option<CryptoKeyPath> {
        self.children.clone()
    }
    pub fn get_parent_fingerprint(&self) -> Option<Fingerprint> {
        self.parent_fingerprint.clone()
    }
    pub fn get_name(&self) -> Option<String> {
        self.name.clone()
    }
    pub fn get_note(&self) -> Option<String> {
        self.note.clone()
    }

    pub fn get_bip32_key(&self) -> String {
        let mut version: Bytes;
        let mut depth: u8 = 0;
        let mut index: u32 = 0;
        let parent_fingerprint: Fingerprint = self.parent_fingerprint.unwrap_or([0, 0, 0, 0]);
        let mut chain_code = self.get_chain_code().unwrap_or(vec![0; 32]);
        let mut key = self.get_key();
        if self.is_master() {
            version = vec![0x04, 0x88, 0xAD, 0xE4];
            depth = 0;
            index = 0;
        } else {
            match self.get_origin() {
                Some(x) => {
                    depth = x.get_components().len() as u8;
                    index = x
                        .get_components()
                        .last()
                        .unwrap()
                        .get_canonical_index()
                        .unwrap_or(0);
                }
                None => {}
            };
            version = match self.is_private_key() {
                true => vec![0x04, 0x88, 0xAD, 0xE4],
                false => vec![0x04, 0x88, 0xB2, 0x1E],
            }
        }
        let mut output = vec![];
        output.append(version.as_mut()); // 4
        output.append(depth.to_be_bytes().to_vec().as_mut()); // 1
        output.append(parent_fingerprint.to_vec().as_mut()); // 4
        output.append(index.to_be_bytes().to_vec().as_mut()); // 4
        output.append(chain_code.as_mut()); //32
        output.append(key.as_mut()); //33
        bs58::encode(output).with_check().into_string()
    }

    pub fn get_account_index(&self, level: u32) -> Option<u32> {
        self.origin
            .clone()
            .map_or(None, |o| match o.get_components().len() {
                0 => None,
                _ => o
                    .get_components()
                    .get(level as usize)
                    .and_then(|v| v.get_index()),
            })
    }

    pub fn get_depth(&self) -> Option<u32> {
        self.origin.clone().map_or(None, |v| v.get_depth())
    }
}

impl RegistryItem for CryptoHDKey {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_HDKEY
    }
}

impl To for CryptoHDKey {
    fn to_cbor(&self) -> Value {
        let mut map: BTreeMap<Value, Value> = BTreeMap::new();
        if self.is_master() {
            map.insert(Value::Integer(IS_MASTER), Value::Bool(self.is_master()));
            map.insert(Value::Integer(KEY_DATA), Value::Bytes(self.key.clone()));
            map.insert(
                Value::Integer(CHAIN_CODE),
                Value::Bytes(self.chain_code.clone().unwrap()),
            );
        } else {
            match self.is_private_key {
                Some(x) => {
                    map.insert(Value::Integer(IS_PRIVATE), Value::Bool(x));
                }
                None => {}
            }
            map.insert(Value::Integer(KEY_DATA), Value::Bytes(self.key.clone()));
            match &self.chain_code {
                Some(x) => {
                    map.insert(Value::Integer(CHAIN_CODE), Value::Bytes(x.clone()));
                }
                None => {}
            }
            match &self.use_info {
                Some(x) => {
                    map.insert(
                        Value::Integer(USE_INFO),
                        Value::Tag(
                            CryptoCoinInfo::get_registry_type().get_tag() as u64,
                            Box::new(x.to_cbor()),
                        ),
                    );
                }
                None => {}
            }
            match &self.origin {
                Some(x) => {
                    map.insert(
                        Value::Integer(ORIGIN),
                        Value::Tag(
                            CryptoKeyPath::get_registry_type().get_tag() as u64,
                            Box::new(x.to_cbor()),
                        ),
                    );
                }
                None => {}
            }
            match &self.children {
                Some(x) => {
                    map.insert(
                        Value::Integer(ORIGIN),
                        Value::Tag(
                            CryptoKeyPath::get_registry_type().get_tag() as u64,
                            Box::new(x.to_cbor()),
                        ),
                    );
                }
                None => {}
            }
            match self.parent_fingerprint {
                Some(x) => {
                    map.insert(
                        Value::Integer(PARENT_FINGERPRINT),
                        Value::Integer(u32::from_be_bytes(x) as i128),
                    );
                }
                None => {}
            }
            match &self.name {
                Some(x) => {
                    map.insert(Value::Integer(NAME), Value::Text(x.clone()));
                }
                None => {}
            }
            match &self.note {
                Some(x) => {
                    map.insert(Value::Integer(NOTE), Value::Text(x.clone()));
                }
                None => {}
            }
        }
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<CryptoHDKey> for CryptoHDKey {
    fn from_cbor(cbor: Value) -> Result<CryptoHDKey, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let is_master = map
            .get_by_integer(IS_MASTER)
            .map(|v| v.get_bool())
            .transpose()?;
        match is_master {
            Some(true) => {
                let key = map
                    .get_by_integer(KEY_DATA)
                    .map(|v| v.get_bytes())
                    .transpose()?
                    .ok_or("key data is required for crypto-hdkey".to_string())?;
                let chain_code = map
                    .get_by_integer(CHAIN_CODE)
                    .map(|v| v.get_bytes())
                    .transpose()?
                    .ok_or(
                        "chain code is required for crypto-hdkey when it is a master key"
                            .to_string(),
                    )?;
                Ok(CryptoHDKey::new_master_key(key, chain_code))
            }
            _ => {
                let is_private_key = map
                    .get_by_integer(IS_PRIVATE)
                    .map(|v| v.get_bool())
                    .transpose()?;
                let key = map
                    .get_by_integer(KEY_DATA)
                    .map(|v| v.get_bytes())
                    .transpose()?
                    .ok_or("key data is required for crypto-hdkey".to_string())?;
                let chain_code = map
                    .get_by_integer(CHAIN_CODE)
                    .map(|v| v.get_bytes())
                    .transpose()?;
                let use_info = map
                    .get_by_integer(USE_INFO)
                    .map(|v| v.get_tag(CRYPTO_COIN_INFO.get_tag()))
                    .transpose()?
                    .map(|v| CryptoCoinInfo::from_cbor(v.get_value().clone()))
                    .transpose()?;
                let origin = map
                    .get_by_integer(ORIGIN)
                    .map(|v| v.get_tag(CRYPTO_KEYPATH.get_tag()))
                    .transpose()?
                    .map(|v| CryptoKeyPath::from_cbor(v.get_value().clone()))
                    .transpose()?;
                let children = map
                    .get_by_integer(CHILDREN)
                    .map(|v| v.get_tag(CRYPTO_KEYPATH.get_tag()))
                    .transpose()?
                    .map(|v| CryptoKeyPath::from_cbor(v.get_value().clone()))
                    .transpose()?;
                let parent_fingerprint = map
                    .get_by_integer(PARENT_FINGERPRINT)
                    .map(|v| v.get_integer())
                    .transpose()?
                    .map(|v| u32::to_be_bytes(v as u32));
                let name = map.get_by_integer(NAME).map(|v| v.get_text()).transpose()?;
                let note = map.get_by_integer(NOTE).map(|v| v.get_text()).transpose()?;
                Ok(CryptoHDKey::new_extended_key(
                    is_private_key,
                    key,
                    chain_code,
                    use_info,
                    origin,
                    children,
                    parent_fingerprint,
                    name,
                    note,
                ))
            }
        }
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoHDKey, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoHDKey::from_cbor(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_coin_info::{CoinType, CryptoCoinInfo, Network};
    use crate::crypto_hd_key::CryptoHDKey;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::traits::{From, To, UR};
    use hex;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let master_key = CryptoHDKey::new_master_key(
            Vec::from_hex("00e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35")
                .unwrap(),
            Vec::from_hex("873dff81c02f525623fd1fe5167eac3a55a049de3d314bb42ee227ffed37d508")
                .unwrap(),
        );
        assert_eq!(
            "A301F503582100E8F32E723DECF4051AEFAC8E2C93C9C5B214313817CDB01A1494B917C8436B35045820873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508",
            hex::encode(master_key.to_bytes()).to_uppercase()
        );

        let hd_key = CryptoHDKey::new_extended_key(
            None,
            Vec::from_hex("026fe2355745bb2db3630bbc80ef5d58951c963c841f54170ba6e5c12be7fc12a6")
                .unwrap(),
            Some(
                Vec::from_hex("ced155c72456255881793514edc5bd9447e7f74abb88c6d6b6480fd016ee8c85")
                    .unwrap(),
            ),
            Some(CryptoCoinInfo::new(None, Some(Network::TestNet))),
            Some(CryptoKeyPath::new(
                vec![
                    PathComponent::new(Some(44), true).unwrap(),
                    PathComponent::new(Some(1), true).unwrap(),
                    PathComponent::new(Some(1), true).unwrap(),
                    PathComponent::new(Some(0), false).unwrap(),
                    PathComponent::new(Some(1), false).unwrap(),
                ],
                None,
                None,
            )),
            None,
            Some([0xe9, 0x18, 0x1c, 0xf3]),
            None,
            None,
        );

        assert_eq!(
            "A5035821026FE2355745BB2DB3630BBC80EF5D58951C963C841F54170BA6E5C12BE7FC12A6045820CED155C72456255881793514EDC5BD9447E7F74ABB88C6D6B6480FD016EE8C8505D90131A1020106D90130A1018A182CF501F501F500F401F4081AE9181CF3",
            hex::encode(hd_key.to_bytes()).to_uppercase()
        );
        assert_eq!(
            "ur:crypto-hdkey/1-1/lpadadcsiocyihbdaehnhdioonaxhdclaojlvoechgferkdpqdiabdrflawshlhdmdcemtfnlrctghchbdolvwsednvdztbgolaahdcxtottgostdkhfdahdlykkecbbweskrymwflvdylgerkloswtbrpfdbsticmwylklpahtaadehoyaoadamtaaddyoyadlecsdwykadykadykaewkadwkaycywlcscewfcpghbziy",
            hd_key.to_ur_encoder(400).next_part().unwrap());
    }

    #[test]
    fn test_decode() {
        let master_key = CryptoHDKey::from_bytes(Vec::from_hex("A301F503582100E8F32E723DECF4051AEFAC8E2C93C9C5B214313817CDB01A1494B917C8436B35045820873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508").unwrap()).unwrap();
        assert_eq!(
            "00e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35",
            hex::encode(master_key.key)
        );
        assert_eq!(
            "873dff81c02f525623fd1fe5167eac3a55a049de3d314bb42ee227ffed37d508",
            hex::encode(master_key.chain_code.unwrap())
        );

        let hd_key = CryptoHDKey::from_bytes(Vec::from_hex("A5035821026FE2355745BB2DB3630BBC80EF5D58951C963C841F54170BA6E5C12BE7FC12A6045820CED155C72456255881793514EDC5BD9447E7F74ABB88C6D6B6480FD016EE8C8505D90131A1020106D90130A1018A182CF501F501F500F401F4081AE9181CF3").unwrap()).unwrap();
        assert_eq!(
            "026fe2355745bb2db3630bbc80ef5d58951c963c841f54170ba6e5c12be7fc12a6",
            hex::encode(hd_key.key.clone())
        );
        assert_eq!(
            "ced155c72456255881793514edc5bd9447e7f74abb88c6d6b6480fd016ee8c85",
            hex::encode(hd_key.chain_code.clone().unwrap())
        );
        assert_eq!(false, hd_key.is_master());
        assert_eq!(false, hd_key.is_private_key());
        assert_eq!(
            CoinType::Bitcoin,
            hd_key.get_use_info().unwrap().get_coin_type()
        );
        assert_eq!(
            Network::TestNet,
            hd_key.get_use_info().unwrap().get_network()
        );
        assert_eq!(
            "44'/1'/1'/0/1",
            hd_key.get_origin().unwrap().get_path().unwrap()
        );
        assert_eq!(
            [0xe9, 0x18, 0x1c, 0xf3],
            hd_key.get_parent_fingerprint().unwrap()
        );
        assert_eq!("xpub6H8Qkexp9BdSgEwPAnhiEjp7NMXVEZWoAFWwon5mSwbuPZMfSUTpPwAP1Q2q2kYMRgRQ8udBpEj89wburY1vW7AWDuYpByteGogpB6pPprX", hd_key.get_bip32_key());
    }
}
