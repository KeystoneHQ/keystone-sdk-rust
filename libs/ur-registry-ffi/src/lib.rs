pub mod solana;
pub mod ethereum;
pub mod extend;
pub mod bitcoin;
mod export;

ffi_support::define_string_destructor!(signer_destroy_string);
