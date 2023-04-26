pub mod solana;
pub mod ethereum;
pub mod extend;
pub mod bitcoin;
pub mod cosmos;
pub mod tron;
pub mod aptos;
pub mod keystone;
pub mod near;
mod export;
mod util;

ffi_support::define_string_destructor!(keystone_sdk_destroy_string);
