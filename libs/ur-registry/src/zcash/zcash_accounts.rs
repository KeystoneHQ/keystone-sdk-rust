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
//!
//! Decode also accepts a device version string at CBOR map key 3 if present.
//! The standard encoder does not emit that field, preserving the existing
//! two-key account export shape for older consumers.

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::{cbor_array, cbor_map},
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_ACCOUNTS, ZCASH_UNIFIED_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::cbor_helpers::{reject_duplicate_key, require_key};
use super::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;

const SEED_FINGERPRINT: u8 = 1;
const ACCOUNTS: u8 = 2;
const DEVICE_VERSION: u8 = 3;

#[derive(Debug, Clone, Default)]
pub struct ZcashAccounts {
    seed_fingerprint: Bytes,
    accounts: Vec<ZcashUnifiedFullViewingKey>,
    device_version: Option<String>,
}

impl ZcashAccounts {
    pub fn new(seed_fingerprint: Bytes, accounts: Vec<ZcashUnifiedFullViewingKey>) -> Self {
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

    /// Stores device version metadata without changing the canonical
    /// `zcash-accounts` encoding.
    pub fn set_device_version(&mut self, device_version: String) {
        self.device_version = Some(device_version);
    }
}

impl MapSize for ZcashAccounts {
    fn map_size(&self) -> u64 {
        2
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

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashAccounts {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashAccounts::default();
        let mut seen_keys = Vec::new();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-accounts map",
            )?;
            match key {
                SEED_FINGERPRINT => {
                    obj.seed_fingerprint = d.bytes()?.to_vec();
                }
                ACCOUNTS => {
                    let mut keys: Vec<ZcashUnifiedFullViewingKey> = alloc::vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        let tag = d.tag()?;
                        if tag != Tag::Unassigned(ZCASH_UNIFIED_FULL_VIEWING_KEY.get_tag()) {
                            return Err(minicbor::decode::Error::message(
                                "unexpected zcash account registry tag",
                            )
                            .at(d.position()));
                        }
                        keys.push(ZcashUnifiedFullViewingKey::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.accounts = keys;
                }
                DEVICE_VERSION => {
                    obj.device_version = Some(d.str()?.to_string());
                }
                _ => {
                    d.skip()?;
                }
            }
            Ok(())
        })?;
        require_key(
            &seen_keys,
            SEED_FINGERPRINT,
            d,
            "missing zcash-accounts seed fingerprint",
        )?;
        require_key(&seen_keys, ACCOUNTS, d, "missing zcash-accounts accounts")?;
        Ok(result)
    }
}

impl TryFrom<Vec<u8>> for ZcashAccounts {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = minicbor::Decoder::new(&value);
        let accounts = <ZcashAccounts as minicbor::Decode<'_, ()>>::decode(&mut decoder, &mut ())
            .map_err(|e| URError::CborDecodeError(e.to_string()))?;
        if decoder.position() != value.len() {
            return Err(URError::CborDecodeError(
                "trailing data after zcash-accounts".to_string(),
            ));
        }
        Ok(accounts)
    }
}

impl TryInto<Vec<u8>> for ZcashAccounts {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;
    use alloc::vec;

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
        assert_eq!(
            decoded.accounts[0].get_ufvk(),
            accounts.accounts[0].get_ufvk()
        );
        assert_eq!(
            decoded.accounts[0].get_index(),
            accounts.accounts[0].get_index()
        );
        assert_eq!(
            decoded.accounts[0].get_name(),
            accounts.accounts[0].get_name()
        );
        assert_eq!(
            decoded.accounts[1].get_ufvk(),
            accounts.accounts[1].get_ufvk()
        );
        assert_eq!(
            decoded.accounts[1].get_index(),
            accounts.accounts[1].get_index()
        );
        assert_eq!(
            decoded.accounts[1].get_name(),
            accounts.accounts[1].get_name()
        );
        assert_eq!(decoded.device_version, None);
    }

    #[test]
    fn test_zcash_accounts_decodes_device_version_extension() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let cbor = hex::decode("a30150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d102800365312e322e33").unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.seed_fingerprint, seed_fingerprint);
        assert_eq!(decoded.device_version, Some("1.2.3".to_string()));
        assert!(decoded.accounts.is_empty());
    }

    #[test]
    fn test_zcash_accounts_encoder_omits_device_version_for_compatibility() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let mut accounts = ZcashAccounts::new(seed_fingerprint, vec![]);
        accounts.set_device_version("1.2.3".to_string());

        let cbor = minicbor::to_vec(&accounts).unwrap();

        assert_eq!(
            hex::encode(&cbor),
            "a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d10280"
        );
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();
        assert_eq!(decoded.device_version, None);
    }

    #[test]
    fn test_zcash_accounts_without_device_version_decodes_from_old_cbor() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let cbor = hex::decode("a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d10280").unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.seed_fingerprint, seed_fingerprint);
        assert!(decoded.accounts.is_empty());
        assert_eq!(decoded.device_version, None);
    }

    #[test]
    fn test_zcash_accounts_rejects_missing_seed_fingerprint() {
        let cbor = hex::decode("a10280").unwrap();

        let result: Result<ZcashAccounts, _> = minicbor::decode(&cbor);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing zcash-accounts seed fingerprint"));
    }

    #[test]
    fn test_zcash_accounts_rejects_missing_accounts() {
        let cbor = hex::decode("a10150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();

        let result: Result<ZcashAccounts, _> = minicbor::decode(&cbor);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("missing zcash-accounts accounts"));
    }

    #[test]
    fn test_zcash_accounts_rejects_duplicate_keys() {
        for cbor_hex in ["a3014001400280", "a3014002800280", "a401400280036131036132"] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let result: Result<ZcashAccounts, _> = minicbor::decode(&cbor);

            assert!(result
                .unwrap_err()
                .to_string()
                .contains("duplicate key in zcash-accounts map"));
        }
    }

    #[test]
    fn test_zcash_accounts_rejects_unexpected_account_tag() {
        let cbor =
            hex::decode("a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d10281d9c034a20161750200").unwrap();

        let result: Result<ZcashAccounts, _> = minicbor::decode(&cbor);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("unexpected zcash account registry tag"));
    }

    #[test]
    fn test_zcash_accounts_try_from_rejects_trailing_data() {
        let mut cbor = hex::decode("a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d10280").unwrap();
        cbor.push(0x00);

        let err = ZcashAccounts::try_from(cbor).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn test_zcash_accounts_skips_unknown_key_and_preserves_device_version() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let cbor =
            hex::decode("a40150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1098201a16178f502800365312e322e33")
                .unwrap();

        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.seed_fingerprint, seed_fingerprint);
        assert!(decoded.accounts.is_empty());
        assert_eq!(decoded.device_version, Some("1.2.3".to_string()));
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
        assert_eq!(accounts_with_version.map_size(), 2);
    }
}
