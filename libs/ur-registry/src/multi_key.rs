use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_ec_key::CryptoECKey;
use crate::crypto_hd_key::CryptoHDKey;
use crate::registry_types::{CRYPTO_ECKEY, CRYPTO_HDKEY};
use crate::traits::RegistryItem;

const THRESHOLD_KEY: u8 = 1;
const KEYS_KEY: u8 = 2;


#[derive(Clone, Debug, Default)]
pub struct MultiKey {
    threshold: u32,
    ec_keys: Option<Vec<CryptoECKey>>,
    hd_keys: Option<Vec<CryptoHDKey>>,
}

impl MultiKey {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn new(threshold: u32, ec_keys: Option<Vec<CryptoECKey>>, hd_keys: Option<Vec<CryptoHDKey>>) -> Self {
        MultiKey { threshold, ec_keys, hd_keys }
    }

    pub fn get_threshold(&self) -> u32 {
        self.threshold
    }

    pub fn get_ec_keys(&self) -> Option<Vec<CryptoECKey>> {
        self.ec_keys.clone()
    }

    pub fn get_hd_keys(&self) -> Option<Vec<CryptoHDKey>> {
        self.hd_keys.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 1;
        if let Some(_) = self.ec_keys {
            size = size + 1;
        }
        if let Some(_) = self.hd_keys {
            size = size + 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for MultiKey {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        let size = self.get_map_size();
        e.map(size)?;
        e.int(Int::from(THRESHOLD_KEY))?.int(
            Int::from(self.threshold)
        )?;
        if let Some(ec_keys) = &self.ec_keys {
            e.int(Int::from(KEYS_KEY))?;
            e.array(ec_keys.len() as u64)?;
            for ec_key in ec_keys {
                e.tag(Tag::Unassigned(CryptoECKey::get_registry_type().get_tag()))?;
                CryptoECKey::encode(ec_key, e, ctx)?;
            }
        }

        if let Some(hd_keys) = &self.hd_keys {
            e.int(Int::from(KEYS_KEY))?;
            e.array(hd_keys.len() as u64)?;
            for hd_key in hd_keys {
                e.tag(Tag::Unassigned(CryptoHDKey::get_registry_type().get_tag()))?;
                CryptoHDKey::encode(hd_key, e, ctx)?;
            }
        }
        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for MultiKey {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = MultiKey::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                THRESHOLD_KEY => {
                    obj.threshold = u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?
                }
                KEYS_KEY => {
                    cbor_array(d, obj, |_index, obj, d| {
                        let tag = d.tag()?;
                        if let Tag::Unassigned(n) = tag {
                            if n == CRYPTO_ECKEY.get_tag() {
                                match &mut obj.ec_keys {
                                    Some(ec_keys) => ec_keys.push(CryptoECKey::decode(d, ctx)?),
                                    None => {
                                        obj.ec_keys = Some(vec![CryptoECKey::decode(d, ctx)?]);
                                    }
                                }
                            } else if n == CRYPTO_HDKEY.get_tag() {
                                match &mut obj.hd_keys {
                                    Some(hd_keys) => hd_keys.push(CryptoHDKey::decode(d, ctx)?),
                                    None => {
                                        obj.hd_keys = Some(vec![CryptoHDKey::decode(d, ctx)?]);
                                    }
                                }
                            }
                        }
                        Ok(())
                    })?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}