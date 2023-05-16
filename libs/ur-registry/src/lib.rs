#![no_std]
#![feature(error_in_core)]

extern crate core;
extern crate alloc;


pub mod registry_types;
pub mod traits;
mod types;
pub mod error;
pub mod crypto_psbt;
pub mod crypto_key_path;
mod cbor;
pub mod crypto_coin_info;
pub mod crypto_hd_key;
pub mod crypto_ec_key;
pub mod multi_key;
pub mod script_expression;
pub mod crypto_output;
pub mod crypto_account;
pub mod solana;
pub mod ethereum;
pub mod extend;
pub mod cosmos;
pub mod aptos;
pub mod keystone;
pub mod pb;
pub mod bytes;
pub mod near;
pub mod arweave;
pub mod cardano;
mod macros;

#[cfg(test)]
mod test {
    use super::*;
    extern crate std;

    use std::println;
    use std::string::String;
    use std::vec::Vec;
    use crate::{impl_template_struct};
    #[test]
    fn test() {
        impl_template_struct!(What {a: String, b: u32, c: Vec::<u8>});
        let x = What::default();
        x.get_a();
    }
}