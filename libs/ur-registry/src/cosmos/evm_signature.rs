use alloc::string::ToString;
use minicbor::data::{Int, Tag};

use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, EVM_SIGNATURE, UUID};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;

impl_template_struct!(EvmSignature {
    request_id: Bytes,
    signature: Bytes
});

impl RegistryItem for EvmSignature {
    fn get_registry_type() -> RegistryType<'static> {
        EVM_SIGNATURE
    }
}

impl MapSize for EvmSignature {
    fn map_size(&self) -> u64 {
        2
    }
}

impl<C> minicbor::Encode<C> for EvmSignature {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(
            Int::try_from(REQUEST_ID)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .tag(Tag::Unassigned(UUID.get_tag()))?
        .bytes(&self.get_request_id())?;
        e.int(
            Int::try_from(SIGNATURE)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .bytes(&self.get_signature())?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for EvmSignature {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = EvmSignature::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Err(minicbor::decode::Error::message("UUID tag is invalid"));
                    }
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGNATURE => {
                    obj.signature = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_encode_evm_signature() {
        let request_id = hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap();
        let signature =
            hex::decode("47e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e").unwrap();
        let evm_signature = EvmSignature::new(request_id, signature);
        let result: Vec<u8> = evm_signature.try_into().unwrap();
        assert_eq!(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584047e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e",
            hex::encode(result).to_lowercase()
        );
    }

    #[test]
    fn test_decode_evm_signature() {
        let request_id = hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap();
        let cbor = hex::decode("a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584047e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e").unwrap();
        let signature =
            hex::decode("47e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e").unwrap();
        let evm_signature = EvmSignature::try_from(cbor).unwrap();
        assert_eq!(request_id, evm_signature.get_request_id());
        assert_eq!(signature, evm_signature.get_signature());
    }
}
