use std::path::Path;
use crate::export;
use bip32::{DerivationPath, XPub};
use secp256k1::{Parity, XOnlyPublicKey};
use hex;
use anyhow::{format_err, Error};
use serde_json::json;
use std::str::FromStr;
use ur_registry::crypto_key_path;
use ur_registry::crypto_key_path::{CryptoKeyPath, PathComponent};
use serde::{Deserialize, Serialize};


#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct PathItem {
    index: u32,
    hardened: bool,
}

impl PathItem {
    fn new(index: u32, hardened: bool) -> Self {
        PathItem {
            index,
            hardened
        }
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub struct HDPath {
    purpose: Option<PathItem>,
    coin_type: Option<PathItem>,
    account: Option<PathItem>,
    change: Option<PathItem>,
    address_index: Option<PathItem>,
}

impl HDPath {
    pub fn empty() -> Self {
        HDPath {
            purpose: None,
            coin_type: None,
            account: None,
            change: None,
            address_index: None,
        }
    }
}

impl TryFrom<&str> for HDPath {
    type Error = String;

    fn try_from(hd_path: &str) -> Result<Self, Self::Error> {
        let key_path = CryptoKeyPath::from_path(hd_path.to_owned(), None)?;
        let path_items = key_path.get_components()
            .iter()
            .map(|component| PathItem::new(component.get_index().unwrap_or_default(), component.is_hardened()))
            .collect::<Vec<PathItem>>();
        let hd_path = HDPath {
            purpose: path_items.get(0).cloned(),
            coin_type: path_items.get(1).cloned(),
            account: path_items.get(2).cloned(),
            change: path_items.get(3).cloned(),
            address_index: path_items.get(4).cloned(),
        };
        Ok(hd_path)
    }
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseHDPath
    fn parse_hd_path(
        hd_path: &str
    ) -> String {
        let hd_path = HDPath::try_from(hd_path);
        let result = match hd_path {
            Ok(path) => path,
            Err(_) => HDPath::empty()
        };
        json!({"result": result}).to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hd_path() {
        let hd_path = "m/44'/60'/0'/0/0";
        let expect_result = r#"{"result":{"account":{"hardened":true,"index":0},"address_index":{"hardened":false,"index":0},"change":{"hardened":false,"index":0},"coin_type":{"hardened":true,"index":60},"purpose":{"hardened":true,"index":44}}}"#;

        assert_eq!(expect_result, parse_hd_path(hd_path))
    }

    #[test]
    fn test_parse_hd_path_given_eth_xpub_path() {
        let hd_path = "M/44\'/60\'/0\'";
        let expect_result = r#"{"result":{"account":{"hardened":true,"index":0},"address_index":null,"change":null,"coin_type":{"hardened":true,"index":60},"purpose":{"hardened":true,"index":44}}}"#;

        assert_eq!(expect_result, parse_hd_path(hd_path))
    }

    #[test]
    fn test_parse_hd_path_given_empty_path() {
        let hd_path = "";
        let expect_result = r#"{"result":{"account":null,"address_index":null,"change":null,"coin_type":null,"purpose":null}}"#;

        assert_eq!(expect_result, parse_hd_path(hd_path))
    }
}
