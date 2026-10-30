#![cfg_attr(not(feature = "std-base"), no_std)]

//! Existing `std` and `core` configurations enable all registry formats.
//! For Zcash CBOR codecs only, disable default features and select `std-base`
//! and `zcash`. Add `ur-encoder` for the UR encoding convenience trait.
//! Features are additive: selecting `std`, `core`, or `all-chains` restores
//! the complete registry API.

extern crate alloc;
extern crate core;

#[cfg(feature = "all-chains")]
pub mod aptos;
#[cfg(feature = "all-chains")]
pub mod arweave;
#[cfg(feature = "all-chains")]
pub mod avalanche;
#[cfg(feature = "all-chains")]
pub mod bitcoin;
#[cfg(feature = "all-chains")]
pub mod bytes;
#[cfg(feature = "all-chains")]
pub mod cardano;
mod cbor;
#[cfg(feature = "all-chains")]
pub mod cosmos;
#[cfg(feature = "all-chains")]
pub mod crypto_account;
#[cfg(feature = "all-chains")]
pub mod crypto_coin_info;
#[cfg(feature = "all-chains")]
pub mod crypto_ec_key;
#[cfg(feature = "all-chains")]
pub mod crypto_hd_key;
#[cfg(feature = "all-chains")]
pub mod crypto_key_path;
#[cfg(feature = "all-chains")]
pub mod crypto_output;
#[cfg(feature = "all-chains")]
pub mod crypto_psbt;
#[cfg(feature = "all-chains")]
pub mod crypto_psbt_extend;
#[cfg(feature = "all-chains")]
pub mod ergo;
pub mod error;
#[cfg(feature = "all-chains")]
pub mod ethereum;
#[cfg(feature = "all-chains")]
pub mod extend;
#[cfg(feature = "all-chains")]
pub mod iota;
#[cfg(feature = "all-chains")]
pub mod kaspa;
mod macros;
#[cfg(feature = "all-chains")]
mod macros_impl;
#[cfg(feature = "all-chains")]
pub mod monero;
#[cfg(feature = "all-chains")]
pub mod multi_key;
#[cfg(feature = "all-chains")]
pub mod near;
#[cfg(feature = "all-chains")]
pub mod pb;
pub mod registry_types;
#[cfg(feature = "all-chains")]
pub mod script_expression;
#[cfg(feature = "all-chains")]
pub mod solana;
#[cfg(feature = "all-chains")]
pub mod stellar;
#[cfg(feature = "all-chains")]
pub mod sui;
#[cfg(feature = "all-chains")]
pub mod ton;
pub mod traits;
#[cfg(feature = "all-chains")]
pub mod tron;
mod types;
#[cfg(feature = "zcash")]
pub mod zcash;

#[cfg(all(feature = "zcash", not(feature = "all-chains")))]
mod zcash_conversions {
    use crate::{
        error::{URError, URResult},
        impl_ur_try_from_cbor_bytes, impl_ur_try_into_cbor_bytes,
        zcash::zcash_pczt::ZcashPczt,
    };
    use alloc::{string::ToString, vec::Vec};
    crate::impl_cbor_bytes!(ZcashPczt,);
}
