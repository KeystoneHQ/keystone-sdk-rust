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

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct XrpBatchSignature {
    request_id: Option<Bytes>,
    signatures: Vec<Bytes>,
}

impl XrpBatchSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Vec<Bytes>) {
        self.signatures = signature;
    }

    pub fn new(request_id: Option<Bytes>, signatures: Vec<Bytes>) -> Self {
        XrpBatchSignature {
            request_id,
            signatures,
        }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
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
        if self.request_id.is_some() {
            size += 1;
        }
        e.map(size)?;
        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }
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
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
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
