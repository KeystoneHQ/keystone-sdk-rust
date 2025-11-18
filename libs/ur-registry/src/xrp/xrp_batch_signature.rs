use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, XRP_BATCH_SIGNATURE, UUID};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const SIGNATURE: u8 = 1;

#[derive(Clone, Debug, Default)]
pub struct XrpBatchSignature {
    signatures: Vec<Bytes>,
}

impl XrpBatchSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_signature(&mut self, signature: Vec<Bytes>) {
        self.signatures = signature;
    }

    pub fn new(signatures: Vec<Bytes>) -> Self {
        XrpBatchSignature {
            signatures,
        }
    }

    pub fn get_signature(&self) -> Vec<Bytes> {
        self.signatures.clone()
    }
}

impl RegistryItem for XrpBatchSignature {
    fn get_registry_type() -> RegistryType<'static> {
        XRP_BATCH_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for XrpBatchSignature {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 1;
        e.map(size)?;
        e.int(Int::from(SIGNATURE))?;
        let len = self.signatures.len().try_into().unwrap();
        e.array(len)?;
        for ele in &self.signatures {
            e.bytes(ele)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for XrpBatchSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = XrpBatchSignature::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SIGNATURE => {
                    let len = d.array()?;
                    obj.signatures = Vec::new();
                    if len.is_some() {
                        for _ in 0..len.unwrap() {
                            obj.signatures.push(d.bytes()?.to_vec());
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for XrpBatchSignature {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<XrpBatchSignature> for XrpBatchSignature {
    fn from_cbor(bytes: Vec<u8>) -> URResult<XrpBatchSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
