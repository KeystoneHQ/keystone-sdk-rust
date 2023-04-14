use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::extend::crypto_multi_accounts::CryptoMultiAccounts;
use ur_registry::traits::From;
use crate::export;
use serde::{Serialize, Deserialize};
use ur_registry::registry_types::CRYPTO_MULTI_ACCOUNTS;
use crate::util::chain::map_coin_type;

pub type Bytes = Vec<u8>;
pub type Fingerprint = [u8; 4];

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiAccounts {
    master_fingerprint: String,
    keys: Vec<Account>,
    device: Option<String>,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    chain: String,
    path: String,
    public_key: String,
    name: String,
    chain_code: String,
    extended_public_key: String,
}

impl core::convert::From<&CryptoHDKey> for Account {
    fn from(value: &CryptoHDKey) -> Account {
        let hd_path = value.get_origin().unwrap().get_components().iter()
            .map(|path|
                format!("{}{}",
                        if path.is_wildcard() { "*".to_string() } else { path.get_index().unwrap_or_default().to_string() },
                        if path.is_hardened() { "'" } else { "" }))
            .collect::<Vec<String>>()
            .join("/");
        let coin_type = value.get_origin().unwrap_or_default().get_components().to_vec()[1].get_index().unwrap_or_default();
        let chain_code = hex::encode(value.get_chain_code().unwrap_or_default());
        let mut xpub = "".to_string();
        if !chain_code.is_empty() && value.get_parent_fingerprint().is_some() && value.get_origin().is_some() {
            xpub = value.get_bip32_key();
        }

        Account {
            chain: map_coin_type(coin_type),
            path: format!("m/{}", hd_path),
            public_key: hex::encode(value.get_key()),
            name: value.get_name().unwrap_or_default(),
            chain_code,
            extended_public_key: xpub,
        }
    }
}

impl Into<MultiAccounts> for CryptoMultiAccounts {
    fn into(self) -> MultiAccounts {
        MultiAccounts {
            master_fingerprint: hex::encode(self.get_master_fingerprint()),
            keys: self.get_keys().iter().map(|key| Account::from(key)).collect(),
            device: self.get_device(),
        }
    }
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseCryptoMultiAccounts
	fn parse_crypto_multi_accounts(ur_type: &str, cbor_hex: &str) -> String {
        if CRYPTO_MULTI_ACCOUNTS.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<MultiAccounts, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let crypto_multi_accounts = CryptoMultiAccounts::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let multi_accounts = crypto_multi_accounts.into();
            Ok(multi_accounts)
        };
        match parse_signature() {
            Ok(multi) => json!(multi).to_string(),
            Err(_) => json!({"error": "crypto multi accounts is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_crypto_multi_accounts() {
        let multi_accounts_cbor = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65";
        let expect_result = "{\"device\":\"keystone\",\"keys\":[{\"chain\":\"SOL\",\"chain_code\":\"\",\"extended_public_key\":\"\",\"name\":\"\",\"path\":\"m/44'/501'/0'/0'\",\"public_key\":\"02eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b\"}],\"master_fingerprint\":\"e9181cf3\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts("crypto-multi-accounts", multi_accounts_cbor));
    }

    #[test]
    fn test_parse_crypto_multi_accounts_with_xpub() {
        // feed illegal large weekend demand typical brick bid dilemma between gasp art

        let multi_accounts_cbor = "a3011aa424853c0281d9012fa4035821034af544244d31619d773521a1a366373c485ff89de50bea543c2b14cccfbb6a500458208dc2427d8ab23caab07729f88f089a3cfa2cfffcd7d1e507f983c0d44a5dbd3506d90130a10186182cf500f500f5081a149439dc03686b657973746f6e65";
        let expect_result = "{\"device\":\"keystone\",\"keys\":[{\"chain\":\"BTC\",\"chain_code\":\"8dc2427d8ab23caab07729f88f089a3cfa2cfffcd7d1e507f983c0d44a5dbd35\",\"extended_public_key\":\"xpub6BoYPFH1MivLdh2BWZuRu6LfuaVSkVak5wsDxjjkAWcUM2QPKyeCHXMgDfRJFvKZhqA4vM5vsgcD6C5ot9eThnFHstgPntNzBLUdLeKS7Zt\",\"name\":\"\",\"path\":\"m/44'/0'/0'\",\"public_key\":\"034af544244d31619d773521a1a366373c485ff89de50bea543c2b14cccfbb6a50\"}],\"master_fingerprint\":\"a424853c\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts("crypto-multi-accounts", multi_accounts_cbor));
    }

    #[test]
    fn test_parse_crypto_multi_accounts_type_error() {
        let hd_key_cbor = "A301F503582100E8F32E723DECF4051AEFAC8E2C93C9C5B214313817CDB01A1494B917C8436B35045820873DFF81C02F525623FD1FE5167EAC3A55A049DE3D314BB42EE227FFED37D508";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts("crypto-hdkey", hd_key_cbor));
    }

    #[test]
    fn test_parse_crypto_multi_accounts_error() {
        let multi_accounts_cbor = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f";
        let expect_result = "{\"error\":\"crypto multi accounts is invalid\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts("crypto-multi-accounts", multi_accounts_cbor));
    }
}
