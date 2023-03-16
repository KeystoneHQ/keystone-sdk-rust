pub mod solana;
pub mod ethereum;
mod export;

ffi_support::define_string_destructor!(signer_destroy_string);
