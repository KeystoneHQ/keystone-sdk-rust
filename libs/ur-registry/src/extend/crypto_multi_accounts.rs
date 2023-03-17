use crate::cbor_value::CborValue;
use crate::crypto_hd_key::CryptoHDKey;
use crate::registry_types::{RegistryType, CRYPTO_HDKEY, CRYPTO_MULTI_ACCOUNTS};
use crate::traits::{From, RegistryItem, To};
use crate::types::Fingerprint;
use serde_cbor::{from_slice, to_vec, Value};
use std::collections::BTreeMap;
use serde::{Serialize, Serializer};
use serde::ser::SerializeStruct;

const MASTER_FINGERPRINT: i128 = 1;
const KEYS: i128 = 2;
const DEVICE: i128 = 3;

#[derive(Default, Clone, Debug)]
pub struct CryptoMultiAccounts {
    master_fingerprint: Fingerprint,
    keys: Vec<CryptoHDKey>,
    device: Option<String>,
}

impl Serialize for CryptoMultiAccounts {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut accounts = serializer.serialize_struct("CryptoMultiAccounts", 3)?;
        accounts.serialize_field("master_fingerprint", &hex::encode(self.master_fingerprint))?;
        accounts.serialize_field("keys", &self.keys)?;
        accounts.serialize_field("device", &self.device)?;
        accounts.end()
    }
}

impl CryptoMultiAccounts {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_master_fingerprint(&mut self, master_fingerprint: Fingerprint) {
        self.master_fingerprint = master_fingerprint;
    }

    pub fn set_keys(&mut self, keys: Vec<CryptoHDKey>) {
        self.keys = keys;
    }

    pub fn add_key(&mut self, key: CryptoHDKey) {
        self.keys.push(key)
    }

    pub fn set_device(&mut self, device: String) {
        self.device = Some(device);
    }

    pub fn new(
        master_fingerprint: Fingerprint,
        keys: Vec<CryptoHDKey>,
        device: Option<String>,
    ) -> CryptoMultiAccounts {
        CryptoMultiAccounts {
            master_fingerprint,
            keys,
            device,
        }
    }

    pub fn get_master_fingerprint(&self) -> Fingerprint {
        self.master_fingerprint.clone()
    }
    pub fn get_keys(&self) -> Vec<CryptoHDKey> {
        self.keys.clone()
    }
    pub fn get_device(&self) -> Option<String> {
        self.device.clone()
    }
}

impl RegistryItem for CryptoMultiAccounts {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_MULTI_ACCOUNTS
    }
}

impl To for CryptoMultiAccounts {
    fn to_cbor(&self) -> Value {
        let mut map: BTreeMap<Value, Value> = BTreeMap::new();
        map.insert(
            Value::Integer(MASTER_FINGERPRINT),
            Value::Integer(u32::from_be_bytes(self.get_master_fingerprint()) as i128),
        );
        map.insert(
            Value::Integer(KEYS),
            Value::Array(
                self.get_keys()
                    .iter()
                    .map(|key| {
                        Value::Tag(
                            CryptoHDKey::get_registry_type().get_tag() as u64,
                            Box::new(key.to_cbor()),
                        )
                    })
                    .collect(),
            ),
        );
        self.get_device()
            .and_then(|device| map.insert(Value::Integer(DEVICE), Value::Text(device)));
        Value::Map(map)
    }

    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<CryptoMultiAccounts> for CryptoMultiAccounts {
    fn from_cbor(cbor: Value) -> Result<CryptoMultiAccounts, String> {
        let value = CborValue::new(cbor);
        let map = value.get_map()?;
        let master_fingerprint = map
            .get_by_integer(MASTER_FINGERPRINT)
            .map_or(
                Err("master_fingerprint is required for crypto-multi-accounts".to_string()),
                |v| v.get_integer(),
            )
            .map(|v| (v as u32).to_be_bytes())?;
        let keys = map
            .get_by_integer(KEYS)
            .map_or(
                Err("keys is required for crypto-multi-accounts".to_string()),
                |v| v.get_array(),
            )?
            .iter()
            .map(|v| {
                v.get_tag(CRYPTO_HDKEY.get_tag())
                    .and_then(|v| CryptoHDKey::from_cbor(v.get_value().clone()))
            })
            .collect::<Result<Vec<CryptoHDKey>, String>>()?;
        let device = map
            .get_by_integer(DEVICE)
            .map(|v| v.get_text())
            .transpose()?;
        Ok(CryptoMultiAccounts {
            master_fingerprint,
            keys,
            device,
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoMultiAccounts, String> {
        let value: Value = match from_slice(bytes.as_slice()) {
            Ok(x) => x,
            Err(e) => return Err(e.to_string()),
        };
        CryptoMultiAccounts::from_cbor(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_hd_key::CryptoHDKey;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::extend::crypto_multi_accounts::CryptoMultiAccounts;
    use crate::traits::{From, To, UR};
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let crypto_hdkey = CryptoHDKey::new_extended_key(
            None,
            Vec::from_hex("02eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b")
                .unwrap(),
            None,
            None,
            Some(CryptoKeyPath::new(
                vec![
                    PathComponent::new(Some(44), true).unwrap(),
                    PathComponent::new(Some(501), true).unwrap(),
                    PathComponent::new(Some(0), true).unwrap(),
                    PathComponent::new(Some(0), true).unwrap(),
                ],
                None,
                None,
            )),
            None,
            None,
            None,
            None,
        );
        let crypto_multi_accounts = CryptoMultiAccounts::new(
            [0xe9, 0x18, 0x1c, 0xf3],
            vec![crypto_hdkey],
            Some("keystone".to_string()),
        );
        assert_eq!("a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65", hex::encode(crypto_multi_accounts.to_bytes()));
        let result = crypto_multi_accounts
            .to_ur_encoder(400)
            .next_part()
            .unwrap();
        assert_eq!("ur:crypto-multi-accounts/1-1/lpadadcsgtcyeokkkgkthdgtotadcywlcscewfaolytaaddloeaxhdclaowdverokopdinhseeroisyalksaykctjshedprnuyjyfgrovawewftyghceglrpkgamtaaddyoyadlocsdwykcfadykykaeykaeykaxisjeihkkjkjyjljtihutltlrvo", result);
    }

    #[test]
    fn test_decode() {
        let crypto_multi_accounts = CryptoMultiAccounts::from_bytes(Vec::from_hex("a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65").unwrap()).unwrap();
        assert_eq!(
            crypto_multi_accounts.master_fingerprint,
            [0xe9, 0x18, 0x1c, 0xf3]
        );
        assert_eq!(crypto_multi_accounts.device, Some("keystone".to_string()));
        assert_eq!(crypto_multi_accounts.keys.len(), 1);
    }

    #[test]
    fn test_decode_multi() {
        let mut decoder = ur::Decoder::default();
        decoder.receive("UR:CRYPTO-MULTI-ACCOUNTS/OTADCYCNTIFDWTAOLNTAADDLOXAOWKAXHDCXSPTPFWOEWNLBTSPKRPAYTODMONECOLWLHDURZSCXSGYNINQDFLRHBYSSCHCFIHGUAMTAADDYOTADLOCSDWYKCFADYKYKAEYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXBSMDKOCXPRDERDVORHGSLFUTTYRTMUMKFTIOENGOGORLEMWPKIUOBYCHVACEJPVTAMTAADDYOTADLOCSDWYKCFADYKYKADYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXWZDKVSECEOURRKKEVWWYRDFGAELYNNPYMDPRAATKAYJKTYRFHSTSBANYZMGLGHPMAMTAADDYOTADLOCSDWYKCFADYKYKAOYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXGLAAUECPATIEADBGPKJNUEYKNNTLADOXTIMURTGWCPAYGSZSYABTVLISECSOJYTKAMTAADDYOTADLOCSDWYKCFADYKYKAXYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXMUJLWLCKPYPMKBNEDPIOGRDINYRYIYWLECBAONHDPMSPBGFYTDEHASKEMTLDFZINAMTAADDYOTADLOCSDWYKCFADYKYKAAYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXKEOLGWPEFSRSKEEMGAONWLMWVWKOISTPPEJZFRVEPKFWVDGAAMAHBTTIJSFSGSLDAMTAADDYOTADLOCSDWYKCFADYKYKAHYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHAXISGRIHKKJKJYJLJTIHLDMEDATK").unwrap();
        let value = decoder.message().unwrap().unwrap();
        let crypto_multi_accounts = CryptoMultiAccounts::from_bytes(value).unwrap();
        assert_eq!(
            0,
            crypto_multi_accounts
                .keys
                .get(0)
                .unwrap()
                .get_account_index(3)
                .unwrap()
        );
        assert_eq!(
            "44'/501'/0'/0'".to_string(),
            crypto_multi_accounts
                .keys
                .get(0)
                .unwrap()
                .get_origin()
                .unwrap()
                .get_path()
                .unwrap()
        );
        assert_eq!(
            5,
            crypto_multi_accounts
                .keys
                .get(0)
                .unwrap()
                .get_depth()
                .unwrap()
        );
    }
}
