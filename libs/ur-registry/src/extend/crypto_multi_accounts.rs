use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_hd_key::CryptoHDKey;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_HDKEY, CRYPTO_MULTI_ACCOUNTS};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Fingerprint;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const MASTER_FINGERPRINT: u8 = 1;
const KEYS: u8 = 2;
const DEVICE: u8 = 3;
const DEVICE_ID: u8 = 4;
const DEVICE_VERSION: u8 = 5;

#[derive(Default, Clone, Debug)]
pub struct CryptoMultiAccounts {
    master_fingerprint: Fingerprint,
    keys: Vec<CryptoHDKey>,
    device: Option<String>,
    device_id: Option<String>,
    device_version: Option<String>,
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

    pub fn set_device_id(&mut self, device_id: String) {
        self.device_id = Some(device_id);
    }

    pub fn set_device_version(&mut self, device_version: String) { self.device_version = Some(device_version); }

    pub fn new(
        master_fingerprint: Fingerprint,
        keys: Vec<CryptoHDKey>,
        device: Option<String>,
        device_id: Option<String>,
        device_version: Option<String>,
    ) -> CryptoMultiAccounts {
        CryptoMultiAccounts {
            master_fingerprint,
            keys,
            device,
            device_id,
            device_version,
        }
    }

    pub fn get_master_fingerprint(&self) -> Fingerprint {
        self.master_fingerprint
    }
    pub fn get_keys(&self) -> Vec<CryptoHDKey> {
        self.keys.clone()
    }
    pub fn get_device(&self) -> Option<String> {
        self.device.clone()
    }
    pub fn get_device_id(&self) -> Option<String> {
        self.device_id.clone()
    }
    pub fn get_device_version(&self) -> Option<String> { self.device_version.clone() }
}

impl RegistryItem for CryptoMultiAccounts {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_MULTI_ACCOUNTS
    }
}

impl<C> minicbor::Encode<C> for CryptoMultiAccounts {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 2;
        if self.device.is_some() {
            size += 1;
        }
        if self.device_id.is_some() {
            size += 1;
        }
        if self.device_version.is_some() {
            size += 1;
        }
        e.map(size)?;

        e.int(Int::from(MASTER_FINGERPRINT))?
            .int(Int::from(u32::from_be_bytes(self.master_fingerprint)))?;

        e.int(Int::from(KEYS))?.array(self.keys.len() as u64)?;
        for key in &self.keys {
            e.tag(Tag::Unassigned(CRYPTO_HDKEY.get_tag()))?;
            CryptoHDKey::encode(key, e, ctx)?;
        }

        if let Some(device) = &self.device {
            e.int(Int::from(DEVICE))?.str(device)?;
        }
        if let Some(device_id) = &self.device_id {
            e.int(Int::from(DEVICE_ID))?.str(device_id)?;
        }
        if let Some(device_version) = &self.device_version {
            e.int(Int::from(DEVICE_VERSION))?.str(device_version)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CryptoMultiAccounts {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoMultiAccounts::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    obj.master_fingerprint = u32::to_be_bytes(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    );
                }
                KEYS => {
                    let mut keys: Vec<CryptoHDKey> = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        keys.push(CryptoHDKey::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.keys = keys;
                }
                DEVICE => {
                    obj.device = Some(d.str()?.to_string());
                }
                DEVICE_ID => {
                    obj.device_id = Some(d.str()?.to_string());
                }
                DEVICE_VERSION => {
                    obj.device_version = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CryptoMultiAccounts {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoMultiAccounts> for CryptoMultiAccounts {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CryptoMultiAccounts> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_hd_key::CryptoHDKey;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::extend::crypto_multi_accounts::CryptoMultiAccounts;
    use crate::traits::{From, To};
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
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
            Some("28475c8d80f6c06bafbe46a7d1750f3fcf2565f7".to_string()),
            Some("1.0.0".to_string()),
        );
        assert_eq!("a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30", hex::encode(crypto_multi_accounts.to_bytes().unwrap()));
        // let result = crypto_multi_accounts
        //     .to_ur_encoder(400)
        //     .next_part()
        //     .unwrap();
        // assert_eq!("ur:crypto-multi-accounts/1-1/lpadadcsgtcyeokkkgkthdgtotadcywlcscewfaolytaaddloeaxhdclaowdverokopdinhseeroisyalksaykctjshedprnuyjyfgrovawewftyghceglrpkgamtaaddyoyadlocsdwykcfadykykaeykaeykaxisjeihkkjkjyjljtihutltlrvo", result);
    }

    #[test]
    fn test_decode() {
        let crypto_multi_accounts = CryptoMultiAccounts::from_cbor(Vec::from_hex("a5011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65047828323834373563386438306636633036626166626534366137643137353066336663663235363566370565312e302e30").unwrap()).unwrap();
        assert_eq!(
            crypto_multi_accounts.master_fingerprint,
            [0xe9, 0x18, 0x1c, 0xf3]
        );
        assert_eq!(crypto_multi_accounts.device, Some("keystone".to_string()));
        assert_eq!(crypto_multi_accounts.keys.len(), 1);
        assert_eq!(crypto_multi_accounts.device_version, Some("1.0.0".to_string()));
    }

    #[test]
    fn test_decode_multi() {
        let part = "UR:CRYPTO-MULTI-ACCOUNTS/OTADCYCNTIFDWTAOLNTAADDLOXAOWKAXHDCXSPTPFWOEWNLBTSPKRPAYTODMONECOLWLHDURZSCXSGYNINQDFLRHBYSSCHCFIHGUAMTAADDYOTADLOCSDWYKCFADYKYKAEYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXBSMDKOCXPRDERDVORHGSLFUTTYRTMUMKFTIOENGOGORLEMWPKIUOBYCHVACEJPVTAMTAADDYOTADLOCSDWYKCFADYKYKADYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXWZDKVSECEOURRKKEVWWYRDFGAELYNNPYMDPRAATKAYJKTYRFHSTSBANYZMGLGHPMAMTAADDYOTADLOCSDWYKCFADYKYKAOYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXGLAAUECPATIEADBGPKJNUEYKNNTLADOXTIMURTGWCPAYGSZSYABTVLISECSOJYTKAMTAADDYOTADLOCSDWYKCFADYKYKAXYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXMUJLWLCKPYPMKBNEDPIOGRDINYRYIYWLECBAONHDPMSPBGFYTDEHASKEMTLDFZINAMTAADDYOTADLOCSDWYKCFADYKYKAAYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHTAADDLOXAOWKAXHDCXKEOLGWPEFSRSKEEMGAONWLMWVWKOISTPPEJZFRVEPKFWVDGAAMAHBTTIJSFSGSLDAMTAADDYOTADLOCSDWYKCFADYKYKAHYKAEYKAOCYCNTIFDWTAXAHASISGRIHKKJKJYJLJTIHAXISGRIHKKJKJYJLJTIHLDMEDATK";
        let decode_data = ur::decode(&part.to_lowercase());
        let crypto_multi_accounts = CryptoMultiAccounts::from_cbor(decode_data.unwrap().1).unwrap();
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
