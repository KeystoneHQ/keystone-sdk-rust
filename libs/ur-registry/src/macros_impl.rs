use crate::aptos::{aptos_sign_request::AptosSignRequest, aptos_signature::AptosSignature};
use crate::arweave::{
    arweave_crypto_account::ArweaveCryptoAccount, arweave_sign_request::ArweaveSignRequest,
    arweave_signature::ArweaveSignature,
};
use crate::bytes::Bytes;
use crate::cardano::{
    cardano_cert_key::CardanoCertKey, cardano_sign_request::CardanoSignRequest,
    cardano_signature::CardanoSignature, cardano_utxo::CardanoUTXO,
};
use crate::cosmos::{cosmos_sign_request::CosmosSignRequest, cosmos_signature::CosmosSignature};
use crate::crypto_account::CryptoAccount;
use crate::crypto_coin_info::CryptoCoinInfo;
use crate::crypto_ec_key::CryptoECKey;
use crate::crypto_hd_key::CryptoHDKey;
use crate::crypto_key_path::CryptoKeyPath;
use crate::crypto_output::CryptoOutput;
use crate::crypto_psbt::CryptoPSBT;
use crate::error::{URError, URResult};
use crate::ethereum::{eth_sign_request::EthSignRequest, eth_signature::EthSignature};
use crate::extend::crypto_multi_accounts::CryptoMultiAccounts;
use crate::keystone::{
    keystone_sign_request::KeystoneSignRequest, keystone_sign_result::KeystoneSignResult,
};
use crate::near::{near_sign_request::NearSignRequest, near_signature::NearSignature};
use crate::solana::{sol_sign_request::SolSignRequest, sol_signature::SolSignature};
use crate::{impl_cbor_bytes, impl_ur_try_from_cbor_bytes, impl_ur_try_into_cbor_bytes};
use alloc::string::ToString;
use alloc::vec::Vec;

impl_cbor_bytes!(
    Bytes,
    CryptoAccount,
    CryptoCoinInfo,
    CryptoECKey,
    CryptoHDKey,
    CryptoKeyPath,
    CryptoOutput,
    CryptoPSBT,
    CardanoSignature,
    CardanoUTXO,
    CardanoSignRequest,
    CardanoCertKey,
    AptosSignRequest,
    AptosSignature,
    ArweaveCryptoAccount,
    ArweaveSignRequest,
    ArweaveSignature,
    CosmosSignRequest,
    CosmosSignature,
    EthSignRequest,
    EthSignature,
    CryptoMultiAccounts,
    KeystoneSignRequest,
    KeystoneSignResult,
    NearSignRequest,
    NearSignature,
    SolSignRequest,
    SolSignature,
);
