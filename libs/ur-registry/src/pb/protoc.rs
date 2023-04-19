#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Account {
    #[prost(string, tag = "1")]
    pub hd_path: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub x_pub: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub address_length: i32,
    #[prost(bool, tag = "4")]
    pub is_multi_sign: bool,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coin {
    #[prost(string, tag = "1")]
    pub coin_code: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub active: bool,
    #[prost(message, repeated, tag = "3")]
    pub accounts: ::prost::alloc::vec::Vec<Account>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sync {
    #[prost(message, repeated, tag = "1")]
    pub coins: ::prost::alloc::vec::Vec<Coin>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcTx {
    /// fee = outputs.size > 1 ? fee = sum(input.value) - sum(output.value) : fee
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(int32, tag = "2")]
    pub dust_threshold: i32,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    /// for normal btc transaction
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
    /// for omni
    #[prost(message, optional, tag = "6")]
    pub omni: ::core::option::Option<Omni>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Omni {
    #[prost(string, tag = "5")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub change_address: ::prost::alloc::string::String,
    /// sat unit
    #[prost(int64, tag = "7")]
    pub omni_amount: i64,
    /// optional default 31 for usdt
    #[prost(int32, tag = "8")]
    pub property_id: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Input {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub index: i32,
    #[prost(message, optional, tag = "3")]
    pub utxo: ::core::option::Option<Utxo>,
    #[prost(string, tag = "4")]
    pub owner_key_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Utxo {
    #[prost(string, tag = "1")]
    pub public_key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub script: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub value: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub value: i64,
    #[prost(bool, tag = "3")]
    pub is_change: bool,
    #[prost(string, tag = "4")]
    pub change_address_path: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EthTx {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub gas_limit: ::prost::alloc::string::String,
    /// optional
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub nonce: i32,
    /// optional, required by erc20 token
    #[prost(message, optional, tag = "7")]
    pub r#override: ::core::option::Option<eth_tx::Override>,
}
/// Nested message and enum types in `EthTx`.
pub mod eth_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Override {
        #[prost(int32, tag = "1")]
        pub decimals: i32,
        #[prost(string, tag = "2")]
        pub token_short_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_full_name: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub contract_address: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EtcTx {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub gas_limit: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    #[prost(int32, tag = "6")]
    pub nonce: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LatestBlock {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub number: i32,
    #[prost(int64, tag = "3")]
    pub timestamp: i64,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Override {
    #[prost(string, tag = "1")]
    pub token_short_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub token_full_name: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub decimals: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TronTx {
    /// required for TRC10 token, for example '1001090' for TRONONE
    #[prost(string, tag = "1")]
    pub token: ::prost::alloc::string::String,
    /// required for TRC20 token
    #[prost(string, tag = "2")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub memo: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub value: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub latest_block: ::core::option::Option<LatestBlock>,
    /// for display token info
    #[prost(message, optional, tag = "8")]
    pub r#override: ::core::option::Option<Override>,
    #[prost(int32, tag = "9")]
    pub fee: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BchTx {
    /// fee = outputs.size > 1 ? fee = sum(input.value) - sum(output.value) : fee
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(int32, tag = "2")]
    pub dust_threshold: i32,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<bch_tx::Input>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
/// Nested message and enum types in `BchTx`.
pub mod bch_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag = "1")]
        pub hash: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub index: i32,
        #[prost(int64, tag = "3")]
        pub value: i64,
        #[prost(string, tag = "4")]
        pub pubkey: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub owner_key_path: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DashTx {
    /// fee = outputs.size > 1 ? fee = sum(input.value) - sum(output.value) : fee
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(int32, tag = "2")]
    pub dust_threshold: i32,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<dash_tx::Input>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
/// Nested message and enum types in `DashTx`.
pub mod dash_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag = "1")]
        pub hash: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub index: i32,
        #[prost(int64, tag = "3")]
        pub value: i64,
        #[prost(string, tag = "4")]
        pub pubkey: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub owner_key_path: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LtcTx {
    /// fee = outputs.size > 1 ? fee = sum(input.value) - sum(output.value) : fee
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(int32, tag = "2")]
    pub dust_threshold: i32,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<Input>,
    #[prost(message, repeated, tag = "5")]
    pub outputs: ::prost::alloc::vec::Vec<Output>,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DcrTx {
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub amount: i64,
    #[prost(message, repeated, tag = "5")]
    pub inputs: ::prost::alloc::vec::Vec<dcr_tx::Input>,
    #[prost(string, tag = "6")]
    pub change_address: ::prost::alloc::string::String,
}
/// Nested message and enum types in `DcrTx`.
pub mod dcr_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag = "1")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub tx_id: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub output_index: i32,
        #[prost(int64, tag = "4")]
        pub atoms: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XzcTx {
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(string, tag = "2")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub amount: i64,
    #[prost(message, repeated, tag = "5")]
    pub inputs: ::prost::alloc::vec::Vec<xzc_tx::Input>,
    #[prost(string, tag = "6")]
    pub change_address: ::prost::alloc::string::String,
}
/// Nested message and enum types in `XzcTx`.
pub mod xzc_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag = "1")]
        pub address: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub tx_id: ::prost::alloc::string::String,
        #[prost(int32, tag = "3")]
        pub output_index: i32,
        #[prost(int64, tag = "4")]
        pub satoshis: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct XrpTx {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub amount: i64,
    #[prost(string, tag = "3")]
    pub change_address: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub fee: i64,
    #[prost(int64, tag = "5")]
    pub sequence: i64,
    #[prost(int64, tag = "6")]
    pub tag: i64,
    #[prost(string, tag = "7")]
    pub memo: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IostTx {
    /// required for token
    #[prost(string, tag = "1")]
    pub token_name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub memo: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub timestamp: i64,
    /// optional default 300s
    #[prost(int32, tag = "7")]
    pub expiration: i32,
    /// optional
    #[prost(message, optional, tag = "8")]
    pub config: ::core::option::Option<iost_tx::Config>,
}
/// Nested message and enum types in `IostTx`.
pub mod iost_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Config {
        #[prost(int64, tag = "1")]
        pub gas_ratio: i64,
        #[prost(int64, tag = "2")]
        pub gas_limit: i64,
        #[prost(int32, tag = "3")]
        pub delay: i32,
        #[prost(string, tag = "4")]
        pub default_limit: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OmniTx {
    /// fee = outputs.size > 1 ? fee = sum(input.value) - sum(output.value) : fee
    #[prost(int64, tag = "1")]
    pub fee: i64,
    #[prost(int32, tag = "2")]
    pub dust_threshold: i32,
    #[prost(string, tag = "3")]
    pub memo: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub inputs: ::prost::alloc::vec::Vec<omni_tx::Input>,
    #[prost(string, tag = "5")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub change_address: ::prost::alloc::string::String,
    /// sat unit
    #[prost(int64, tag = "7")]
    pub omni_amount: i64,
    /// optional default 31 for usdt
    #[prost(int32, tag = "8")]
    pub property_id: i32,
}
/// Nested message and enum types in `OmniTx`.
pub mod omni_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Input {
        #[prost(string, tag = "1")]
        pub hash: ::prost::alloc::string::String,
        #[prost(int32, tag = "2")]
        pub index: i32,
        #[prost(message, optional, tag = "3")]
        pub utxo: ::core::option::Option<Utxo>,
        #[prost(string, tag = "4")]
        pub owner_key_path: ::prost::alloc::string::String,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Utxo {
        #[prost(string, tag = "1")]
        pub public_key: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub script: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub value: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EosTx {
    /// optional default 'transfer'
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    /// optional default, required for token
    #[prost(string, tag = "2")]
    pub token_account: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<eos_tx::Data>,
    #[prost(message, optional, tag = "4")]
    pub header: ::core::option::Option<eos_tx::Header>,
}
/// Nested message and enum types in `EosTx`.
pub mod eos_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Data {
        #[prost(string, tag = "1")]
        pub from: ::prost::alloc::string::String,
        #[prost(string, tag = "2")]
        pub to: ::prost::alloc::string::String,
        #[prost(int64, tag = "3")]
        pub amount: i64,
        /// optional, default "EOS"
        #[prost(string, tag = "4")]
        pub symbol: ::prost::alloc::string::String,
        #[prost(string, tag = "5")]
        pub memo: ::prost::alloc::string::String,
        #[prost(int64, tag = "6")]
        pub fee: i64,
        #[prost(int32, tag = "7")]
        pub decimal: i32,
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Header {
        #[prost(int64, tag = "1")]
        pub time: i64,
        #[prost(int32, tag = "2")]
        pub expire_in_seconds: i32,
        #[prost(int64, tag = "3")]
        pub ref_block_num: i64,
        #[prost(int64, tag = "4")]
        pub ref_block_prefix: i64,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DotTx {
    #[prost(int64, tag = "1")]
    pub value: i64,
    #[prost(string, tag = "2")]
    pub dest: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub nonce: i64,
    /// optional
    #[prost(int64, tag = "5")]
    pub tip: i64,
    #[prost(int64, tag = "6")]
    pub transaction_version: i64,
    #[prost(int64, tag = "7")]
    pub spec_version: i64,
    /// optional
    #[prost(int64, tag = "8")]
    pub validity_period: i64,
    #[prost(int64, tag = "9")]
    pub impl_version: i64,
    #[prost(int64, tag = "10")]
    pub authoring_version: i64,
    #[prost(int32, tag = "11")]
    pub block_number: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KsmTx {
    #[prost(int64, tag = "1")]
    pub value: i64,
    #[prost(string, tag = "2")]
    pub dest: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub nonce: i64,
    /// optional
    #[prost(int64, tag = "5")]
    pub tip: i64,
    #[prost(int64, tag = "6")]
    pub transaction_version: i64,
    #[prost(int64, tag = "7")]
    pub spec_version: i64,
    /// optional
    #[prost(int64, tag = "8")]
    pub validity_period: i64,
    #[prost(int64, tag = "9")]
    pub impl_version: i64,
    #[prost(int64, tag = "10")]
    pub authoring_version: i64,
    #[prost(int32, tag = "11")]
    pub block_number: i32,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CfxTx {
    #[prost(string, tag = "1")]
    pub to: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub gas_price: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub gas: ::prost::alloc::string::String,
    #[prost(int32, tag = "5")]
    pub nonce: i32,
    #[prost(string, tag = "6")]
    pub storage_limit: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub epoch_height: ::prost::alloc::string::String,
    /// optional
    #[prost(string, tag = "8")]
    pub chain_id: ::prost::alloc::string::String,
    /// optional
    #[prost(string, tag = "9")]
    pub contract_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "10")]
    pub r#override: ::core::option::Option<cfx_tx::Override>,
}
/// Nested message and enum types in `CfxTx`.
pub mod cfx_tx {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Override {
        #[prost(int32, tag = "1")]
        pub decimals: i32,
        #[prost(string, tag = "2")]
        pub token_short_name: ::prost::alloc::string::String,
        #[prost(string, tag = "3")]
        pub token_full_name: ::prost::alloc::string::String,
        #[prost(string, tag = "4")]
        pub contract_address: ::prost::alloc::string::String,
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransaction {
    #[prost(string, tag = "1")]
    pub coin_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub sign_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub hd_path: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub timestamp: i64,
    #[prost(int32, tag = "5")]
    pub decimal: i32,
    #[prost(
        oneof = "sign_transaction::Transaction",
        tags = "6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21"
    )]
    pub transaction: ::core::option::Option<sign_transaction::Transaction>,
}
/// Nested message and enum types in `SignTransaction`.
pub mod sign_transaction {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Transaction {
        #[prost(message, tag = "6")]
        BtcTx(super::BtcTx),
        #[prost(message, tag = "7")]
        EthTx(super::EthTx),
        #[prost(message, tag = "8")]
        TronTx(super::TronTx),
        #[prost(message, tag = "9")]
        EtcTx(super::EtcTx),
        #[prost(message, tag = "10")]
        BchTx(super::BchTx),
        #[prost(message, tag = "11")]
        DashTx(super::DashTx),
        #[prost(message, tag = "12")]
        LtcTx(super::LtcTx),
        #[prost(message, tag = "13")]
        DcrTx(super::DcrTx),
        #[prost(message, tag = "14")]
        XzcTx(super::XzcTx),
        #[prost(message, tag = "15")]
        XrpTx(super::XrpTx),
        #[prost(message, tag = "16")]
        IostTx(super::IostTx),
        #[prost(message, tag = "17")]
        OmniTx(super::OmniTx),
        #[prost(message, tag = "18")]
        EosTx(super::EosTx),
        #[prost(message, tag = "19")]
        DotTx(super::DotTx),
        #[prost(message, tag = "20")]
        KsmTx(super::KsmTx),
        #[prost(message, tag = "21")]
        CfxTx(super::CfxTx),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignMessage {
    #[prost(string, tag = "1")]
    pub coin_code: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub hd_path: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub message: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VerifyAddress {
    #[prost(int32, tag = "1")]
    pub coin_type: i32,
    #[prost(int32, tag = "2")]
    pub address_index: i32,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignTransactionResult {
    #[prost(string, tag = "1")]
    pub sign_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub raw_tx: ::prost::alloc::string::String,
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Payload {
    #[prost(enumeration = "payload::Type", tag = "1")]
    pub r#type: i32,
    #[prost(string, tag = "2")]
    pub xfp: ::prost::alloc::string::String,
    #[prost(oneof = "payload::Content", tags = "3, 4, 5, 6, 7")]
    pub content: ::core::option::Option<payload::Content>,
}
/// Nested message and enum types in `Payload`.
pub mod payload {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[derive(
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration
    )]
    #[repr(i32)]
    pub enum Type {
        Reserve = 0,
        Sync = 1,
        SignTx = 2,
        SignMsg = 3,
        SignMultiSig = 4,
        SyncMultiSigMsg = 5,
        SignEthMultiSigMsg = 6,
        VerifyAddress = 7,
        Staking = 8,
        SignTxResult = 9,
    }
    impl Type {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Type::Reserve => "TYPE_RESERVE",
                Type::Sync => "TYPE_SYNC",
                Type::SignTx => "TYPE_SIGN_TX",
                Type::SignMsg => "TYPE_SIGN_MSG",
                Type::SignMultiSig => "TYPE_SIGN_MULTI_SIG",
                Type::SyncMultiSigMsg => "TYPE_SYNC_MULTI_SIG_MSG",
                Type::SignEthMultiSigMsg => "TYPE_SIGN_ETH_MULTI_SIG_MSG",
                Type::VerifyAddress => "TYPE_VERIFY_ADDRESS",
                Type::Staking => "TYPE_STAKING",
                Type::SignTxResult => "TYPE_SIGN_TX_RESULT",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "TYPE_RESERVE" => Some(Self::Reserve),
                "TYPE_SYNC" => Some(Self::Sync),
                "TYPE_SIGN_TX" => Some(Self::SignTx),
                "TYPE_SIGN_MSG" => Some(Self::SignMsg),
                "TYPE_SIGN_MULTI_SIG" => Some(Self::SignMultiSig),
                "TYPE_SYNC_MULTI_SIG_MSG" => Some(Self::SyncMultiSigMsg),
                "TYPE_SIGN_ETH_MULTI_SIG_MSG" => Some(Self::SignEthMultiSigMsg),
                "TYPE_VERIFY_ADDRESS" => Some(Self::VerifyAddress),
                "TYPE_STAKING" => Some(Self::Staking),
                "TYPE_SIGN_TX_RESULT" => Some(Self::SignTxResult),
                _ => None,
            }
        }
    }
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(message, tag = "3")]
        Sync(super::Sync),
        #[prost(message, tag = "4")]
        SignTx(super::SignTransaction),
        #[prost(message, tag = "5")]
        SignMsg(super::SignMessage),
        #[prost(message, tag = "6")]
        VerifyAddr(super::VerifyAddress),
        #[prost(message, tag = "7")]
        SignTxResult(super::SignTransactionResult),
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Base {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub data: ::core::option::Option<Payload>,
    #[prost(string, tag = "6")]
    pub device_type: ::prost::alloc::string::String,
    #[prost(oneof = "base::Content", tags = "4, 5")]
    pub content: ::core::option::Option<base::Content>,
}
/// Nested message and enum types in `Base`.
pub mod base {
    #[derive(serde::Serialize, serde::Deserialize)]
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Content {
        #[prost(int32, tag = "4")]
        HotVersion(i32),
        #[prost(int32, tag = "5")]
        ColdVersion(i32),
    }
}
