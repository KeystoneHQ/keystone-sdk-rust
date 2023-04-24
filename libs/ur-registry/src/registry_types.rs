use alloc::string::{String, ToString};
use crate::error::{URError, URResult};

#[derive(Clone, Debug)]
pub enum URType {
    CryptoPsbt(String),
    CryptoAccount(String),
    EthSignRequest(String),
    Bytes(String)
}

impl URType {
    pub fn from(type_str: &str) -> URResult<URType> {
        match type_str {
            "crypto-psbt" => {
                Ok(URType::CryptoPsbt(type_str.to_string()))
            }
            "crypto-account" => {
                Ok(URType::CryptoAccount(type_str.to_string()))
            }
            "bytes" => {
                Ok(URType::Bytes(type_str.to_string()))
            }
            "eth-sign-request" => {
                Ok(URType::EthSignRequest(type_str.to_string()))
            }
            _ => {
                Err(URError::NotSupportURTypeError(type_str.to_string()))
            }
        }
    }

    pub fn get_type_str(&self) -> String {
        match self {
            URType::CryptoPsbt(type_str) => {
                type_str.to_string()
            }
            URType::CryptoAccount(type_str) => {
                type_str.to_string()
            }
            URType::Bytes(type_str) => {
                type_str.to_string()
            }
            URType::EthSignRequest(type_str) => {
                type_str.to_string()
            }
	}
    }
}


pub struct RegistryType<'a>(&'a str, Option<u64>);

impl<'a> RegistryType<'_> {
    pub fn get_type(&self) -> String {
        self.0.to_string()
    }
    pub fn get_tag(&self) -> u64 {
        self.1.unwrap_or(u64::MAX)
    }
}

pub const BYTES: RegistryType = RegistryType("bytes", None);
pub const UUID: RegistryType = RegistryType("uuid", Some(37));
pub const CRYPTO_HDKEY: RegistryType = RegistryType("crypto-hdkey", Some(303));
pub const CRYPTO_KEYPATH: RegistryType = RegistryType("crypto-keypath", Some(304));
pub const CRYPTO_COIN_INFO: RegistryType = RegistryType("crypto-coin-info", Some(305));
pub const CRYPTO_ECKEY: RegistryType = RegistryType("crypto-eckey", Some(306));
pub const CRYPTO_OUTPUT: RegistryType = RegistryType("crypto-output", Some(308));
pub const CRYPTO_PSBT: RegistryType = RegistryType("crypto-psbt", Some(310));
pub const CRYPTO_ACCOUNT: RegistryType = RegistryType("crypto-account", Some(311));

// Multiple Accounts
pub const CRYPTO_MULTI_ACCOUNTS: RegistryType = RegistryType("crypto-multi-accounts", Some(1103));

// ETH
pub const ETH_SIGN_REQUEST: RegistryType = RegistryType("eth-sign-request", Some(401));
pub const ETH_SIGNATURE: RegistryType = RegistryType("eth-signature", Some(402));
// SOL
pub const SOL_SIGN_REQUEST: RegistryType = RegistryType("sol-sign-request", Some(1101));
pub const SOL_SIGNATURE: RegistryType = RegistryType("sol-signature", Some(1102));
// Cosmos
pub const COSMOS_SIGN_REQUEST: RegistryType = RegistryType("cosmos-sign-request", Some(4101));
pub const COSMOS_SIGNATURE: RegistryType = RegistryType("cosmos-signature", Some(4102));
// Tron
pub const TRON_SIGN_REQUEST: RegistryType = RegistryType("tron-sign-request", Some(5101));
pub const TRON_SIGNATURE: RegistryType = RegistryType("tron-signature", Some(5102));
// Aptos
pub const APTOS_SIGN_REQUEST: RegistryType = RegistryType("aptos-sign-request", Some(3101));
pub const APTOS_SIGNATURE: RegistryType = RegistryType("aptos-signature", Some(3102));
// UTXO
pub const KEYSTONE_SIGN_REQUEST: RegistryType = RegistryType("keystone-sign-request", Some(6101));
pub const KEYSTONE_SIGN_RESULT: RegistryType = RegistryType("keystone-sign-result", Some(6102));
