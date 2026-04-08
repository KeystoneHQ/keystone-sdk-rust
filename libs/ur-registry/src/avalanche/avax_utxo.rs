use crate::cbor::{cbor_array, cbor_map};
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, AVAX_SIGN_REQUEST, UUID, CRYPTO_KEYPATH, AVAX_UTXO};
use crate::crypto_key_path::CryptoKeyPath;
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const TX_ID: u8 = 1;
const OUTPUT_INDEX: u8 = 2;
const DERIVATION_PATH: u8 = 3;

#[derive(Debug, Clone, Default)]
pub struct AvaxUtxo  {
    pub tx_id: Bytes,
    pub output_index: u32,
    derivation_path: CryptoKeyPath,
}

impl AvaxUtxo {
    pub fn new(
        tx_id: Bytes,
        output_index: u32,
        derivation_path: CryptoKeyPath,
    ) -> Self {
        AvaxUtxo {
            tx_id,
            output_index,
            derivation_path,
        }
    }

    pub fn get_tx_id(&self) -> Bytes {
        self.tx_id.clone()
    }

    pub fn get_output_index(&self) -> u32 {
        self.output_index
    }

    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }
}

impl RegistryItem for AvaxUtxo {
    fn get_registry_type() -> RegistryType<'static> {
        AVAX_UTXO
    }
}

impl<C> minicbor::Encode<C> for AvaxUtxo {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(3)?;
        e.int(Int::from(TX_ID))?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.tx_id)?;
        
        e.int(Int::from(OUTPUT_INDEX))?
            .int(Int::from(self.output_index))?;
        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for AvaxUtxo {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = AvaxUtxo::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                TX_ID => {
                    d.tag()?;
                    obj.tx_id = d.bytes()?.to_vec();
                }
                OUTPUT_INDEX => {
                    obj.output_index = u32::try_from(d.int()?)
                        .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, _ctx)?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for AvaxUtxo {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<AvaxUtxo> for AvaxUtxo {
    fn from_cbor(bytes: Vec<u8>) -> URResult<AvaxUtxo> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::RegistryItem;
    use alloc::vec::Vec;
    use hex::FromHex;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use alloc::vec;
    extern crate std;
    use std::println;

    #[test]
    fn test_avax_utxo_cbor() {
        let tx_id = Vec::from_hex("a3b1c5d6e7f8091a2b3c4d5e6f708192a3b1c5d6e7f8091a2b3c4d5e6f708192").unwrap();
        let output_index = 1u32;
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(9000), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
        ];

        let derivation_path = CryptoKeyPath::new(
            components,
            Some(Fingerprint::from_hex("f23a9fd2").unwrap()),
            None,
        );

        let utxo = AvaxUtxo::new(tx_id.clone(), output_index, derivation_path.clone());

        let cbor_bytes = utxo.to_bytes().unwrap();

        let decoded_utxo = AvaxUtxo::from_cbor(cbor_bytes).unwrap();

        assert_eq!(decoded_utxo.get_tx_id(), tx_id);
        assert_eq!(decoded_utxo.get_output_index(), output_index);
    }
}
