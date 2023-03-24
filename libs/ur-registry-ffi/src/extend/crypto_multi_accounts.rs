use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::extend::crypto_multi_accounts::CryptoMultiAccounts;
use ur_registry::traits::From;
use crate::export;
use serde::{Serialize, Deserialize};
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

        Account {
            chain: map_coin_type(coin_type),
            path: format!("m/{}", hd_path),
            public_key: hex::encode(value.get_key()),
            name: value.get_name().unwrap_or_default(),
            chain_code: hex::encode(value.get_chain_code().unwrap_or_default())
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
	fn parse_crypto_multi_accounts(
		cbor_hex: &str
	) -> String {
        let parse_signature = || -> Result<MultiAccounts, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let crypto_multi_accounts = CryptoMultiAccounts::from_cbor(res).map_err(|_| format_err!(""))?;
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
        let expect_result = "{\"device\":\"keystone\",\"keys\":[{\"chain\":\"SOL\",\"chain_code\":\"\",\"name\":\"\",\"path\":\"m/44'/501'/0'/0'\",\"public_key\":\"02eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b\"}],\"master_fingerprint\":\"e9181cf3\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts(multi_accounts_cbor));
    }

    #[test]
    fn test_parse_crypto_multi_accounts_error() {
        let multi_accounts_cbor = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f";
        let expect_result = "{\"error\":\"crypto multi accounts is invalid\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts(multi_accounts_cbor));
    }
}
