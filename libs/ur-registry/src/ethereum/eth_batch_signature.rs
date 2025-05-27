use alloc::vec::Vec;
use alloc::{string::ToString, vec};
use minicbor::{
    data::{Int, Tag},
    encode::Write,
    Decoder, Encoder,
};

use crate::{
    cbor::{cbor_array, cbor_map},
    ethereum::eth_signature::EthSignature,
    registry_types::{RegistryType, ETH_BATCH_SIGNATURE, ETH_SIGNATURE},
    traits::{MapSize, RegistryItem},
};

const SIGNATURES: u8 = 1;

#[derive(Debug, Default, Clone)]
pub struct EthBatchSignature {
    signatures: Vec<EthSignature>,
}

impl EthBatchSignature {
    pub fn new(signatures: Vec<EthSignature>) -> Self {
        Self { signatures }
    }

    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_signatures(&mut self, signatures: Vec<EthSignature>) {
        self.signatures = signatures;
    }

    pub fn get_signatures(&self) -> &Vec<EthSignature> {
        &self.signatures
    }
}

impl RegistryItem for EthBatchSignature {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_BATCH_SIGNATURE
    }
}

impl MapSize for EthBatchSignature {
    fn map_size(&self) -> u64 {
        1
    }
}

impl<C> minicbor::Encode<C> for EthBatchSignature {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(SIGNATURES))?
            .array(self.signatures.len() as u64)?;
        for signature in &self.signatures {
            e.tag(Tag::Unassigned(ETH_SIGNATURE.get_tag()))?;
            signature.encode(e, _ctx)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for EthBatchSignature {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = EthBatchSignature::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SIGNATURES => {
                    let mut signatures = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        signatures.push(EthSignature::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.set_signatures(signatures);
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

    #[test]
    fn test_decode() {
        let cbor = hex::decode("a10181d90192a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258416ff1602c5074594261319e77d58697565aeebebac538628434b13c939b9ac7f52d8fb08e7892e26ebcf901bf8c5b38d5900dbc6466b9b9e976287324e92c1aa12403686b657973746f6e65").unwrap();
        let batch_signature = EthBatchSignature::try_from(cbor).unwrap();
        assert_eq!(batch_signature.signatures.len(), 1);
        assert_eq!(hex::encode(batch_signature.signatures[0].get_signature()), "6ff1602c5074594261319e77d58697565aeebebac538628434b13c939b9ac7f52d8fb08e7892e26ebcf901bf8c5b38d5900dbc6466b9b9e976287324e92c1aa124");
    }
}
