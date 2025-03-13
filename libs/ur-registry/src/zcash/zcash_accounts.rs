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


use alloc::{string::ToString, vec::Vec};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::{cbor_array, cbor_map},
    impl_template_struct,
    registry_types::{RegistryType, ZCASH_ACCOUNTS, ZCASH_UNIFIED_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;

const SEED_FINGERPRINT: u8 = 1;
const ACCOUNTS: u8 = 2;

impl_template_struct!(ZcashAccounts {
    seed_fingerprint: Bytes,
    accounts: Vec<ZcashUnifiedFullViewingKey>
});

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
    }
    
    #[test]
    fn test_zcash_accounts_empty() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        
        let accounts = ZcashAccounts {
            seed_fingerprint,
            accounts: vec![],
        };
        
        let cbor = minicbor::to_vec(&accounts).unwrap();
        let decoded: ZcashAccounts = minicbor::decode(&cbor).unwrap();
        
        assert_eq!(decoded.seed_fingerprint, accounts.seed_fingerprint);
        assert_eq!(decoded.accounts.len(), 0);
    }
    
    #[test]
    fn test_map_size() {
        let seed_fingerprint = hex::decode("d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1").unwrap();
        let accounts = ZcashAccounts {
            seed_fingerprint,
            accounts: vec![],
        };
        
        assert_eq!(accounts.map_size(), 2);
    }
}
