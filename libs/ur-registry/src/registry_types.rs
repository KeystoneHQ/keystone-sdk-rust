use crate::error::{URError, URResult};
use alloc::string::{String, ToString};

#[derive(Clone, Debug)]
pub enum URType {
    CryptoPsbt(String),
    CryptoMultiAccounts(String),
    CryptoAccount(String),
    EthSignRequest(String),
    SolSignRequest(String),
    StellarSignRequest(String),
    NearSignRequest(String),
    ArweaveSignRequest(String),
    AptosSignRequest(String),
    CardanoSignRequest(String),
    CardanoSignDataRequest(String),
    CardanoSignCip8DataRequest(String),
    CardanoCatalystVotingRegistrationRequest(String),
    CosmosSignRequest(String),
    EvmSignRequest(String),
    SuiSignRequest(String),
    SuiSignHashRequest(String),
    TonSignRequest(String),
    QRHardwareCall(String),
    Bytes(String),
    BtcSignRequest(String),
    KeystoneSignRequest(String),
    ZcashPczt(String),
    XmrOutput(String),
    XmrTxUnsigned(String),
}

impl URType {
    pub fn from(type_str: &str) -> URResult<URType> {
        match type_str {
            "crypto-psbt" => Ok(URType::CryptoPsbt(type_str.to_string())),
            "crypto-multi-accounts" => Ok(URType::CryptoMultiAccounts(type_str.to_string())),
            "crypto-account" => Ok(URType::CryptoAccount(type_str.to_string())),
            "bytes" => Ok(URType::Bytes(type_str.to_string())),
            "btc-sign-request" => Ok(URType::BtcSignRequest(type_str.to_string())),
            "keystone-sign-request" => Ok(URType::KeystoneSignRequest(type_str.to_string())),
            "eth-sign-request" => Ok(URType::EthSignRequest(type_str.to_string())),
            "sol-sign-request" => Ok(URType::SolSignRequest(type_str.to_string())),
            "stellar-sign-request" => Ok(URType::StellarSignRequest(type_str.to_string())),
            "arweave-sign-request" => Ok(URType::ArweaveSignRequest(type_str.to_string())),
            "cosmos-sign-request" => Ok(URType::CosmosSignRequest(type_str.to_string())),
            "evm-sign-request" => Ok(URType::EvmSignRequest(type_str.to_string())),
            "near-sign-request" => Ok(URType::NearSignRequest(type_str.to_string())),
            "aptos-sign-request" => Ok(URType::AptosSignRequest(type_str.to_string())),
            "sui-sign-request" => Ok(URType::SuiSignRequest(type_str.to_string())),
            "sui-sign-hash-request" => Ok(URType::SuiSignHashRequest(type_str.to_string())),
            "cardano-sign-request" => Ok(URType::CardanoSignRequest(type_str.to_string())),
            "cardano-sign-data-request" => Ok(URType::CardanoSignDataRequest(type_str.to_string())),
            "cardano-sign-cip8-data-request" => {
                Ok(URType::CardanoSignCip8DataRequest(type_str.to_string()))
            }
            "cardano-catalyst-voting-registration" => Ok(
                URType::CardanoCatalystVotingRegistrationRequest(type_str.to_string()),
            ),
            "qr-hardware-call" => Ok(URType::QRHardwareCall(type_str.to_string())),
            "ton-sign-request" => Ok(URType::TonSignRequest(type_str.to_string())),
            "zcash-pczt" => Ok(URType::ZcashPczt(type_str.to_string())),
            "xmr-output" => Ok(URType::XmrOutput(type_str.to_string())),
            "xmr-txunsigned" => Ok(URType::XmrTxUnsigned(type_str.to_string())),
            _ => Err(URError::NotSupportURTypeError(type_str.to_string())),
        }
    }

    pub fn get_type_str(&self) -> String {
        match self {
            URType::CryptoPsbt(type_str) => type_str.to_string(),
            URType::CryptoMultiAccounts(type_str) => type_str.to_string(),
            URType::CryptoAccount(type_str) => type_str.to_string(),
            URType::Bytes(type_str) => type_str.to_string(),
            URType::BtcSignRequest(type_str) => type_str.to_string(),
            URType::KeystoneSignRequest(type_str) => type_str.to_string(),
            URType::EthSignRequest(type_str) => type_str.to_string(),
            URType::SolSignRequest(type_str) => type_str.to_string(),
            URType::StellarSignRequest(type_str) => type_str.to_string(),
            URType::NearSignRequest(type_str) => type_str.to_string(),
            URType::ArweaveSignRequest(type_str) => type_str.to_string(),
            URType::AptosSignRequest(type_str) => type_str.to_string(),
            URType::CardanoSignRequest(type_str) => type_str.to_string(),
            URType::CardanoSignDataRequest(type_str) => type_str.to_string(),
            URType::CardanoSignCip8DataRequest(type_str) => type_str.to_string(),
            URType::CardanoCatalystVotingRegistrationRequest(type_str) => type_str.to_string(),
            URType::SuiSignRequest(type_str) => type_str.to_string(),
            URType::SuiSignHashRequest(type_str) => type_str.to_string(),
            URType::CosmosSignRequest(type_str) => type_str.to_string(),
            URType::EvmSignRequest(type_str) => type_str.to_string(),
            URType::QRHardwareCall(type_str) => type_str.to_string(),
            URType::TonSignRequest(type_str) => type_str.to_string(),
            URType::ZcashPczt(type_str) => type_str.to_string(),
            URType::XmrOutput(type_str) => type_str.to_string(),
            URType::XmrTxUnsigned(type_str) => type_str.to_string(),
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

// QR hardware call
pub const QR_HARDWARE_CALL: RegistryType = RegistryType("qr-hardware-call", Some(1201));
pub const KEY_DERIVATION_CALL: RegistryType = RegistryType("key-derivation-call", Some(1301));
pub const KEY_DERIVATION_SCHEMA: RegistryType = RegistryType("key-derivation-schema", Some(1302));

// Near
pub const NEAR_SIGN_REQUEST: RegistryType = RegistryType("near-sign-request", Some(2101));
pub const NEAR_SIGNATURE: RegistryType = RegistryType("near-signature", Some(2102));
// Arweave
pub const ARWEAVE_CRYPTO_ACCOUNT: RegistryType = RegistryType("arweave-crypto-account", Some(5101));
pub const ARWEAVE_SIGN_REQUEST: RegistryType = RegistryType("arweave-sign-request", Some(5102));
pub const ARWEAVE_SIGNATURE: RegistryType = RegistryType("arweave-signature", Some(5103));
// Cosmos
pub const COSMOS_SIGN_REQUEST: RegistryType = RegistryType("cosmos-sign-request", Some(4101));
pub const COSMOS_SIGNATURE: RegistryType = RegistryType("cosmos-signature", Some(4102));
// EVM
pub const EVM_SIGN_REQUEST: RegistryType = RegistryType("evm-sign-request", Some(4101));
pub const EVM_SIGNATURE: RegistryType = RegistryType("evm-signature", Some(4102));
// Tron
pub const TRON_SIGN_REQUEST: RegistryType = RegistryType("tron-sign-request", Some(5201));
pub const TRON_SIGNATURE: RegistryType = RegistryType("tron-signature", Some(5202));
// Aptos
pub const APTOS_SIGN_REQUEST: RegistryType = RegistryType("aptos-sign-request", Some(3101));
pub const APTOS_SIGNATURE: RegistryType = RegistryType("aptos-signature", Some(3102));
// UTXO
pub const KEYSTONE_SIGN_REQUEST: RegistryType = RegistryType("keystone-sign-request", Some(6101));
pub const KEYSTONE_SIGN_RESULT: RegistryType = RegistryType("keystone-sign-result", Some(6102));
// CARDANO
pub const CARDANO_UTXO: RegistryType = RegistryType("cardano-utxo", Some(2201));
pub const CARDANO_SIGN_REQUEST: RegistryType = RegistryType("cardano-sign-request", Some(2202));
pub const CARDANO_SIGNATURE: RegistryType = RegistryType("cardano-signature", Some(2203));
pub const CARDANO_CERT_KEY: RegistryType = RegistryType("cardano-cert-key", Some(2204));
pub const CARDANO_SIGN_DATA_REQUEST: RegistryType =
    RegistryType("cardano-sign-data-request", Some(2205));
pub const CARDANO_SIGN_DATA_SIGNATURE: RegistryType =
    RegistryType("cardano-sign-data-signature", Some(2206));
pub const CARDANO_CATALYST_VOTING_REGISTRATION: RegistryType =
    RegistryType("cardano-catalyst-voting-registration", Some(2207));
pub const CARDANO_CATALYST_VOTING_REGISTRATION_SIGNATURE: RegistryType =
    RegistryType("cardano-catalyst-voting-registration-signature", Some(2208));
pub const CARDANO_DELEGSTION: RegistryType = RegistryType("cardano-delegation", Some(2209));
pub const CARDANO_SIGN_CIP8_DATA_REQUEST: RegistryType =
    RegistryType("cardano-sign-cip8-data-request", Some(2210));
pub const CARDANO_SIGN_CIP8_DATA_SIGNATURE: RegistryType =
    RegistryType("cardano-sign-cip8-data-signature", Some(2211));
// Sui
pub const SUI_SIGN_REQUEST: RegistryType = RegistryType("sui-sign-request", Some(7101));
pub const SUI_SIGNATURE: RegistryType = RegistryType("sui-signature", Some(7102));
pub const SUI_SIGN_HASH_REQUEST: RegistryType = RegistryType("sui-sign-hash-request", Some(7103));
// Ton
pub const TON_SIGN_REQUEST: RegistryType = RegistryType("ton-sign-request", Some(7201));
pub const TON_SIGNATURE: RegistryType = RegistryType("ton-signature", Some(7202));
// BTC
pub const BTC_SIGN_REQUEST: RegistryType = RegistryType("btc-sign-request", Some(8101));
pub const BTC_SIGNATURE: RegistryType = RegistryType("btc-signature", Some(8102));
// Stellar
pub const STELLAR_SIGN_REQUEST: RegistryType = RegistryType("stellar-sign-request", Some(8201));
pub const STELLAR_SIGNATURE: RegistryType = RegistryType("stellar-signature", Some(8202));
// Monero
pub const XMR_OUTPUT: RegistryType = RegistryType("xmr-output", Some(8301));
pub const XMR_KEYIMAGE: RegistryType = RegistryType("xmr-keyimage", Some(8302));
pub const XMR_TXUNSIGNED: RegistryType = RegistryType("xmr-txunsigned", Some(8303));
pub const XMR_TXSIGNED: RegistryType = RegistryType("xmr-txsigned", Some(8304));

// Zcash
pub const ZCASH_ACCOUNTS: RegistryType = RegistryType("zcash-accounts", Some(49201));
pub const ZCASH_FULL_VIEWING_KEY: RegistryType = RegistryType("zcash-full-viewing-key", Some(49202));
pub const ZCASH_UNIFIED_FULL_VIEWING_KEY: RegistryType = RegistryType("zcash-unified-full-viewing-key", Some(49203));
pub const ZCASH_PCZT: RegistryType = RegistryType("zcash-pczt", Some(49204));
