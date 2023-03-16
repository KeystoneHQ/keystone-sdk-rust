pub mod solana;
pub mod ethereum;
pub mod extend;
mod export;

ffi_support::define_string_destructor!(signer_destroy_string);
