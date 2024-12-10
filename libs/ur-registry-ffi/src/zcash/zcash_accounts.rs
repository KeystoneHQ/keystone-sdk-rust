use crate::export;
use anyhow::{format_err, Error};
use serde::{Deserialize, Serialize};
use serde_json::json;
use ur_registry::{crypto_hd_key::CryptoHDKey, registry_types::ZCASH_ACCOUNTS};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct ZcashAccounts {
    seed_fingerprint: String,
    accounts: Vec<ZcashUnifiedAccount>,
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
