// This module provides FFI (Foreign Function Interface) functions for handling Zcash accounts
// data structures. It allows for conversion between CBOR-encoded Uniform Resources (URs) and
// Zcash account information.
//
// The module exports functions for parsing Zcash accounts from UR format, which includes:
// - Seed fingerprint: A unique identifier for the seed that generated the accounts
// - Accounts: A collection of Zcash Unified Full Viewing Keys (UFVKs) with their metadata
// - Device version: An optional firmware version included in parsed JSON when present
//
// Each Zcash Unified Account contains:
// - UFVK: The Unified Full Viewing Key string
// - Index: The account index
// - Name: An optional account name
//
// The registry decoder rejects malformed account CBOR, including missing
// required fields, duplicate map keys, unexpected nested UFVK tags, and trailing
// data. This FFI layer maps those decode failures to the stable JSON error
// string used by existing callers.

use crate::export;
use anyhow::{format_err, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ur_registry::registry_types::ZCASH_ACCOUNTS;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct ZcashAccounts {
    seed_fingerprint: String,
    accounts: Vec<ZcashUnifiedAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    device_version: Option<String>,
}

impl From<ur_registry::zcash::zcash_accounts::ZcashAccounts> for ZcashAccounts {
    fn from(value: ur_registry::zcash::zcash_accounts::ZcashAccounts) -> Self {
        Self {
            seed_fingerprint: hex::encode(value.get_seed_fingerprint()),
            accounts: value
                .get_accounts()
                .iter()
                .map(|account| account.clone().into())
                .collect(),
            device_version: value.get_device_version(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct ZcashUnifiedAccount {
    ufvk: String,
    index: u32,
    name: Option<String>,
}

impl From<ur_registry::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey>
    for ZcashUnifiedAccount
{
    fn from(
        value: ur_registry::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey,
    ) -> Self {
        Self {
            ufvk: value.get_ufvk(),
            index: value.get_index(),
            name: value.get_name(),
        }
    }
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseZcashAccounts
    fn parse_zcash_accounts(
        ur_type: &str,
        cbor_hex: &str
    ) -> String {
        if ZCASH_ACCOUNTS.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }
        let parse_accounts = || -> Result<ZcashAccounts, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let zcash_accounts =
                ur_registry::zcash::zcash_accounts::ZcashAccounts::try_from(cbor).map_err(|_| format_err!(""))?;
            let accounts = zcash_accounts.into();
            Ok(accounts)
        };
        match parse_accounts() {
            Ok(accounts) => json!(accounts).to_string(),
            Err(_) => json!({"error": "zcash accounts is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minicbor;
    use ur_registry::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;
    #[test]
    fn test_zcash_accounts_conversion() {
        let seed_fingerprint = vec![0xd1; 16];

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

        let mut ur_accounts = ur_registry::zcash::zcash_accounts::ZcashAccounts::new(
            seed_fingerprint.clone(),
            vec![ufvk1, ufvk2],
        );
        ur_accounts.set_device_version("1.2.3".to_string());

        let ffi_accounts: ZcashAccounts = ur_accounts.into();

        assert_eq!(ffi_accounts.seed_fingerprint, hex::encode(seed_fingerprint));
        assert_eq!(ffi_accounts.device_version, Some("1.2.3".to_string()));
        assert_eq!(ffi_accounts.accounts.len(), 2);
        assert_eq!(ffi_accounts.accounts[0].ufvk, "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl");
        assert_eq!(ffi_accounts.accounts[0].index, 0);
        assert_eq!(
            ffi_accounts.accounts[0].name,
            Some("Keystone 1".to_string())
        );
        assert_eq!(ffi_accounts.accounts[1].ufvk, "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl");
        assert_eq!(ffi_accounts.accounts[1].index, 1);
        assert_eq!(
            ffi_accounts.accounts[1].name,
            Some("Keystone 2".to_string())
        );
    }

    #[test]
    fn test_zcash_unified_account_conversion() {
        let ur_ufvk = ZcashUnifiedFullViewingKey::new(
            "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            42,
            Some("Keystone".to_string())
        );

        let ffi_account: ZcashUnifiedAccount = ur_ufvk.into();

        assert_eq!(ffi_account.ufvk, "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl");
        assert_eq!(ffi_account.index, 42);
        assert_eq!(ffi_account.name, Some("Keystone".to_string()));
    }

    #[test]
    fn test_parse_zcash_accounts() {
        let seed_fingerprint = vec![0xd1; 16];
        let expected_seed_fingerprint = hex::encode(&seed_fingerprint);
        let expected_ufvk = "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl";

        let ufvk = ZcashUnifiedFullViewingKey::new(
            expected_ufvk.to_string(),
            0,
            Some("Keystone".to_string()),
        );

        let ur_accounts =
            ur_registry::zcash::zcash_accounts::ZcashAccounts::new(seed_fingerprint, vec![ufvk]);

        let cbor = minicbor::to_vec(&ur_accounts).unwrap();
        let cbor_hex = hex::encode(&cbor);

        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), &cbor_hex);

        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json_result.get("error").is_none());

        assert_eq!(json_result["seed_fingerprint"], expected_seed_fingerprint);
        assert!(json_result.get("device_version").is_none());
        assert_eq!(json_result["accounts"].as_array().unwrap().len(), 1);
        assert_eq!(json_result["accounts"][0]["ufvk"], expected_ufvk);
        assert_eq!(json_result["accounts"][0]["index"], 0);
        assert_eq!(json_result["accounts"][0]["name"], "Keystone");
    }

    #[test]
    fn test_parse_zcash_accounts_includes_device_version_when_cbor_has_extension() {
        let cbor_hex = "a30150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d102800365312e322e33";

        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), cbor_hex);

        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json_result.get("error").is_none());
        assert_eq!(
            json_result["seed_fingerprint"],
            "d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1"
        );
        assert_eq!(json_result["device_version"], "1.2.3");
        assert_eq!(json_result["accounts"].as_array().unwrap().len(), 0);
    }

    #[test]
    fn test_parse_zcash_accounts_omits_device_version_when_absent() {
        let cbor_hex = "a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d10280";

        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), cbor_hex);

        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert!(json_result.get("error").is_none());
        assert_eq!(
            json_result["seed_fingerprint"],
            "d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1"
        );
        assert_eq!(json_result["accounts"].as_array().unwrap().len(), 0);
        assert!(json_result.get("device_version").is_none());
    }

    #[test]
    fn test_parse_zcash_accounts_type_mismatch() {
        let result = parse_zcash_accounts("wrong-type", "deadbeef");
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "type not match");
    }

    #[test]
    fn test_parse_zcash_accounts_invalid_cbor() {
        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), "invalid");
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "zcash accounts is invalid");
    }

    #[test]
    fn test_parse_zcash_accounts_rejects_trailing_data() {
        let cbor_hex = "a20150d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1028000";

        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), cbor_hex);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "zcash accounts is invalid");
    }

    #[test]
    fn test_parse_zcash_accounts_rejects_duplicate_keys() {
        let cbor_hex = "a3014001400280";

        let result = parse_zcash_accounts(&ZCASH_ACCOUNTS.get_type(), cbor_hex);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();

        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "zcash accounts is invalid");
    }
}
