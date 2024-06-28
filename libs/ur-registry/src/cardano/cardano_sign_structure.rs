use crate::cbor::cbor_array;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{CARDANO_CERT_KEY, CRYPTO_KEYPATH, RegistryType};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use prost::Message;
use core::convert::From;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const KEY_CONTEXT: u8 = 0;
const KEY_PROTECTED_HEADER: u8 = 1;
const KEY_EXTERNAL_AAD: u8 = 2;
const KEY_PAYLOAD: u8 = 3;

use crate::impl_template_struct;

impl_template_struct!(CardanoSignStructure {
    context: Bytes,
    protected_header: Bytes,
    external_aad: Bytes,
    payload: String
});

impl MapSize for CardanoSignStructure {
    fn map_size(&self) -> u64 {
        4
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoSignStructure {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_sign_structure = CardanoSignStructure::default();
        cbor_array(d, &mut cardano_sign_structure, |_index, obj, d| {
            let _index =
                u8::try_from(_index).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;

            match _index {
                KEY_CONTEXT => {
                    obj.context = d.str()?.to_string().encode_to_vec()
                }
                KEY_PROTECTED_HEADER => {
                    obj.protected_header = d.bytes()?.to_vec()
                }
                KEY_EXTERNAL_AAD => {
                    obj.external_aad = d.bytes()?.to_vec()
                }
                KEY_PAYLOAD => {
                    obj.payload = hex::encode(d.bytes()?);
                }
                _ => {
                    d.skip()?;
                }
            }
            Ok(())
        })?;
        Ok(cardano_sign_structure)
    }
}

impl FromCbor<CardanoSignStructure> for CardanoSignStructure {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoSignStructure> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_sign_structure() {
        let sign_structure_str = "846a5369676e617475726531582aa201276761646472657373581de133a4f1f468454744b2ff3644b2ab79d48e76a3187f902fe8a1bcfaad4058857b22707572706f7365223a224b6f696f73204163636f756e7420566572696669636174696f6e222c226163636f756e74223a2265313333613466316634363834353437343462326666333634346232616237396434386537366133313837663930326665386131626366616164222c226e6f6e6365223a313731393533383936383333347d";
        let sign_structure_bytes = hex::decode(sign_structure_str).unwrap();

        let sign_structure = CardanoSignStructure::from_cbor(sign_structure_bytes).unwrap();
        let payload = hex::decode(sign_structure.payload).unwrap();
        let payload = String::from_utf8(payload).unwrap();
        assert_eq!(payload, "{\"purpose\":\"Koios Account Verification\",\"account\":\"e133a4f1f468454744b2ff3644b2ab79d48e76a3187f902fe8a1bcfaad\",\"nonce\":1719538968334}");
    }
}