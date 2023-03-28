use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use crate::cbor::cbor_map;
use crate::crypto_coin_info::CryptoCoinInfo;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, UrResult};
use crate::registry_types::{CRYPTO_HDKEY, RegistryType};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};

const IS_MASTER: u8 = 1;
const IS_PRIVATE: u8 = 2;
const KEY_DATA: u8 = 3;
const CHAIN_CODE: u8 = 4;
const USE_INFO: u8 = 5;
const ORIGIN: u8 = 6;
const CHILDREN: u8 = 7;
const PARENT_FINGERPRINT: u8 = 8;
const NAME: u8 = 9;
const NOTE: u8 = 10;

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

    fn get_map_size(&self) -> u64 {
        let mut size = 1;
        if let Some(_) = self.is_private_key {
            size = size + 1;
        }
        if let Some(_) = self.chain_code {
            size = size + 1;
        }
        if let Some(_) = self.use_info {
            size = size + 1;
        }

        if let Some(_) = self.origin {
            size = size + 1;
        }

        if let Some(_) = self.children {
            size = size + 1;
        }

        if let Some(_) = self.parent_fingerprint {
            size = size + 1;
        }

        if let Some(_) = self.name {
            size = size + 1;
        }

        if let Some(_) = self.note {
            size = size + 1;
        }
        size
    }
}

impl RegistryItem for CryptoHDKey {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_HDKEY
    }
}

impl<C> minicbor::Encode<C> for CryptoHDKey {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        if self.is_master() {
            e.map(3)?;
            e.int(
                Int::try_from(IS_MASTER)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?.bool(self.is_master())?;
            e.int(
                Int::try_from(KEY_DATA)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?.bytes(&*self.get_key())?;
            e.int(
                Int::try_from(CHAIN_CODE)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?.bytes(&*self.get_chain_code().ok_or(minicbor::encode::Error::message("is_master is true, but have no chain code"))?)?;
        } else {
            let size = self.get_map_size();
            e.map(size)?;

            match self.is_private_key {
                Some(x) => {
                    e.int(
                        Int::try_from(IS_PRIVATE)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                    )?.bool(x)?;
                }
                None => {}
            }

            e.int(
                Int::try_from(KEY_DATA)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?.bytes(&*self.get_key())?;

            match &self.chain_code {
                Some(x) => {
                    e.int(
                        Int::try_from(CHAIN_CODE)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                    )?.bytes(x)?;
                }
                None => {}
            }

            match &self.use_info {
                Some(x) => {
                    e.int(
                        Int::try_from(USE_INFO)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                    )?;
                    e.tag(Tag::Unassigned(CryptoCoinInfo::get_registry_type().get_tag()))?;
                    CryptoCoinInfo::encode(x, e, ctx)?;
                }
                None => {}
            }

            match &self.origin {
                Some(x) => {
                    e.int(
                        Int::try_from(ORIGIN)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                    )?;
                    e.tag(Tag::Unassigned(CryptoKeyPath::get_registry_type().get_tag()))?;
                    CryptoKeyPath::encode(x, e, ctx)?;
                }
                None => {}
            }

            match &self.children {
                Some(x) => {
                    e.int(
                        Int::try_from(CHILDREN)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                    )?;
                    e.tag(Tag::Unassigned(CryptoKeyPath::get_registry_type().get_tag()))?;
                    CryptoKeyPath::encode(x, e, ctx)?;
                }
                None => {}
            }

            match self.parent_fingerprint {
                Some(x) => {
                    e.int(
                        Int::try_from(PARENT_FINGERPRINT)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
                        .int(
                            Int::try_from(u32::from_be_bytes(x))
                                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
                        )?;
                }
                None => {}
            }

            match &self.name {
                Some(x) => {
                    e.int(
                        Int::try_from(NAME)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
                        .str(x)?;
                }
                None => {}
            }

            match &self.note {
                Some(x) => {
                    e.int(
                        Int::try_from(NOTE)
                            .map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
                        .str(x)?;
                }
                None => {}
            }
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CryptoHDKey {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoHDKey::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                IS_MASTER => {
                    obj.is_master = Some(d.bool()?);
                }
                IS_PRIVATE => {
                    obj.is_private_key = Some(d.bool()?);
                }
                KEY_DATA => {
                    obj.key = d.bytes()?.to_vec();
                }
                CHAIN_CODE => {
                    obj.chain_code = Some(d.bytes()?.to_vec());
                }
                USE_INFO => {
                    d.tag()?;
                    obj.use_info = Some(CryptoCoinInfo::decode(d, ctx)?);
                }
                ORIGIN => {
                    d.tag()?;
                    obj.origin = Some(CryptoKeyPath::decode(d, ctx)?)
                }
                CHILDREN => {
                    d.tag()?;
                    obj.children = Some(CryptoKeyPath::decode(d, ctx)?)
                }
                PARENT_FINGERPRINT => {
                    obj.parent_fingerprint = Some(u32::to_be_bytes(u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?));
                }
                NAME => {
                    obj.name = Some(d.str()?.to_string())
                }
                NOTE => {
                    obj.note = Some(d.str()?.to_string())
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(result)
    }
}


impl To for CryptoHDKey {
    fn to_cbor(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoHDKey> for CryptoHDKey {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<CryptoHDKey> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::crypto_coin_info::{CoinType, CryptoCoinInfo, Network};
    use crate::crypto_hd_key::CryptoHDKey;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::traits::{From as FromCbor, RegistryItem, To};
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
            hex::encode(master_key.to_cbor().unwrap()).to_uppercase()
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
            hex::encode(hd_key.to_cbor().unwrap()).to_uppercase()
        );
        assert_eq!(
            "ur:crypto-hdkey/onaxhdclaojlvoechgferkdpqdiabdrflawshlhdmdcemtfnlrctghchbdolvwsednvdztbgolaahdcxtottgostdkhfdahdlykkecbbweskrymwflvdylgerkloswtbrpfdbsticmwylklpahtaadehoyaoadamtaaddyoyadlecsdwykadykadykaewkadwkaycywlcscewfihbdaehn",
            ur::encode(&*(hd_key.to_cbor().unwrap()), CryptoHDKey::get_registry_type().get_type()));
    }

    #[test]
    fn test_decode() {
        let master_key = CryptoHDKey::from_cbor(Vec::from_hex("A301F503582100E8F32E723DECF4051AEFAC8E2C93C9C5B214313817CDB01A1494B917C8436B35045820873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508").unwrap()).unwrap();
        assert_eq!(
            "00e8f32e723decf4051aefac8e2c93c9c5b214313817cdb01a1494b917c8436b35",
            hex::encode(master_key.key)
        );
        assert_eq!(
            "873dff81c02f525623fd1fe5167eac3a55a049de3d314bb42ee227ffed37d508",
            hex::encode(master_key.chain_code.unwrap())
        );

        let hd_key = CryptoHDKey::from_cbor(Vec::from_hex("A5035821026FE2355745BB2DB3630BBC80EF5D58951C963C841F54170BA6E5C12BE7FC12A6045820CED155C72456255881793514EDC5BD9447E7F74ABB88C6D6B6480FD016EE8C8505D90131A1020106D90130A1018A182CF501F501F500F401F4081AE9181CF3").unwrap()).unwrap();
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
