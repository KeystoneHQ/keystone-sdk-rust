//! Zcash Accounts Registry Type
//!
//! This module implements the CBOR encoding and decoding for Zcash accounts.
//! It represents a collection of Zcash unified full viewing keys with an associated
//! seed fingerprint for identification.
//!
//! The structure follows the UR Registry Type specification for Zcash accounts,
//! with a map containing:
//! - Seed fingerprint: A byte string that uniquely identifies the seed
//! - Accounts: An array of Zcash unified full viewing keys


use alloc::{string::{String, ToString}, vec::Vec};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::{cbor_array, cbor_map},
    registry_types::{RegistryType, ZCASH_ACCOUNTS, ZCASH_UNIFIED_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;

const SEED_FINGERPRINT: u8 = 1;
const ACCOUNTS: u8 = 2;
const DEVICE_VERSION: u8 = 3;

#[derive(Debug, Clone, Default)]
pub struct ZcashAccounts {
    pub seed_fingerprint: Bytes,
    pub accounts: Vec<ZcashUnifiedFullViewingKey>,
    pub device_version: Option<String>,
}

impl ZcashAccounts {
    pub fn new(
        seed_fingerprint: Bytes,
        accounts: Vec<ZcashUnifiedFullViewingKey>,
    ) -> Self {
        Self {
            seed_fingerprint,
            accounts,
            device_version: None,
        }
    }

    pub fn get_seed_fingerprint(&self) -> Bytes {
        self.seed_fingerprint.clone()
    }

    pub fn set_seed_fingerprint(&mut self, seed_fingerprint: Bytes) {
        self.seed_fingerprint = seed_fingerprint;
    }

    pub fn get_accounts(&self) -> Vec<ZcashUnifiedFullViewingKey> {
        self.accounts.clone()
    }

    pub fn set_accounts(&mut self, accounts: Vec<ZcashUnifiedFullViewingKey>) {
        self.accounts = accounts;
    }

    pub fn get_device_version(&self) -> Option<String> {
        self.device_version.clone()
    }

    pub fn set_device_version(&mut self, device_version: String) {
        self.device_version = Some(device_version);
    }
}

impl MapSize for ZcashAccounts {
    fn map_size(&self) -> u64 {
        let mut size = 2;
        if self.device_version.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for ZcashAccounts {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_ACCOUNTS
    }
}

impl<C> minicbor::Encode<C> for ZcashAccounts {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(SEED_FINGERPRINT))?
            .bytes(&self.seed_fingerprint)?;

        e.int(Int::from(ACCOUNTS))?
            .array(self.accounts.len() as u64)?;
        for account in &self.accounts {
            e.tag(Tag::Unassigned(ZCASH_UNIFIED_FULL_VIEWING_KEY.get_tag()))?;
            ZcashUnifiedFullViewingKey::encode(account, e, _ctx)?;
        }

        if let Some(device_version) = &self.device_version {
            e.int(Int::from(DEVICE_VERSION))?.str(device_version)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashAccounts {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashAccounts::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SEED_FINGERPRINT => {
                    obj.seed_fingerprint = d.bytes()?.to_vec();
                }
                ACCOUNTS => {
                    let mut keys: Vec<ZcashUnifiedFullViewingKey> = alloc::vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        keys.push(ZcashUnifiedFullViewingKey::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.accounts = keys;
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

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;
    use crate::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;

    #[test]
    fn test_zcash_accounts_encode_decode() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();

        let ufvk1 = ZcashUnifiedFullViewingKey::new(
            "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            0,
            Some("Keystone 1".to_string())
        );

        let ufvk2 = ZcashUnifiedFullViewingKey::new(
            "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            1,
            Some("Keystone 2".to_string())
        );

        let accounts = ZcashAccounts {
            seed_fingerprint,
            accounts: vec![ufvk1, ufvk2],
            device_version: None,
        };

        let cbor = minicbor::to_vec(&accounts).unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.seed_fingerprint, accounts.seed_fingerprint);
        assert_eq!(decoded.accounts.len(), 2);
        assert_eq!(decoded.accounts[0].get_ufvk(), accounts.accounts[0].get_ufvk());
        assert_eq!(decoded.accounts[0].get_index(), accounts.accounts[0].get_index());
        assert_eq!(decoded.accounts[0].get_name(), accounts.accounts[0].get_name());
        assert_eq!(decoded.accounts[1].get_ufvk(), accounts.accounts[1].get_ufvk());
        assert_eq!(decoded.accounts[1].get_index(), accounts.accounts[1].get_index());
        assert_eq!(decoded.accounts[1].get_name(), accounts.accounts[1].get_name());
        assert_eq!(decoded.device_version, None);
    }

    #[test]
    fn test_zcash_accounts_with_device_version() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();

        let ufvk = ZcashUnifiedFullViewingKey::new(
            "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            0,
            Some("Keystone 1".to_string())
        );

        let mut accounts = ZcashAccounts::new(seed_fingerprint, vec![ufvk]);
        accounts.set_device_version("1.2.3".to_string());

        let cbor = minicbor::to_vec(&accounts).unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.device_version, Some("1.2.3".to_string()));
        assert_eq!(decoded.accounts.len(), 1);
    }

    #[test]
    fn test_zcash_accounts_without_device_version_decodes_from_old_cbor() {
        // Encode without device_version, then decode — simulates old firmware
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let accounts = ZcashAccounts::new(seed_fingerprint, vec![]);

        let cbor = minicbor::to_vec(&accounts).unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.device_version, None);
    }

    #[test]
    fn test_zcash_accounts_empty() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();

        let accounts = ZcashAccounts::new(seed_fingerprint.clone(), vec![]);

        let cbor = minicbor::to_vec(&accounts).unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.seed_fingerprint, seed_fingerprint);
        assert_eq!(decoded.accounts.len(), 0);
    }

    #[test]
    fn test_map_size() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let accounts = ZcashAccounts::new(seed_fingerprint, vec![]);
        assert_eq!(accounts.map_size(), 2);

        let mut accounts_with_version = ZcashAccounts::new(vec![], vec![]);
        accounts_with_version.set_device_version("1.0.0".to_string());
        assert_eq!(accounts_with_version.map_size(), 3);
    }
}
