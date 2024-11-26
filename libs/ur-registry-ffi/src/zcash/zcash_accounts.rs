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
struct ZcashTransparentAccount {
    path: String,
    xpub: String,
}

impl From<CryptoHDKey> for ZcashTransparentAccount {
    fn from(value: CryptoHDKey) -> Self {
        Self {
            path: value.get_origin().unwrap().get_path().unwrap(),
            xpub: value.get_bip32_key(),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct ZcashShieldedAccount {
    path: String,
    fvk: String,
}

impl From<ur_registry::zcash::zcash_full_viewing_key::ZcashFullViewingKey>
    for ZcashShieldedAccount
{
    fn from(value: ur_registry::zcash::zcash_full_viewing_key::ZcashFullViewingKey) -> Self {
        Self {
            path: value.get_key_path().get_path().unwrap(),
            fvk: hex::encode(value.get_key_data()),
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
struct ZcashUnifiedAccount {
    transparent: Option<ZcashTransparentAccount>,
    orchard: ZcashShieldedAccount,
    name: Option<String>,
}

impl From<ur_registry::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey>
    for ZcashUnifiedAccount
{
    fn from(
        value: ur_registry::zcash::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey,
    ) -> Self {
        Self {
            transparent: value.get_transparent().map(|account| account.into()),
            orchard: value.get_orchard().into(),
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
