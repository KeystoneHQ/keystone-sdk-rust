use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, NEAR_SIGNATURE, UUID};
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
pub struct NearSignature {
    request_id: Option<Bytes>,
    signature: Vec<Bytes>,
}

impl NearSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Vec<Bytes>) {
        self.signature = signature;
    }

    pub fn new(request_id: Option<Bytes>, signature: Vec<Bytes>) -> Self {
        NearSignature {
            request_id,
            signature,
        }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Vec<Bytes> {
        self.signature.clone()
    }
}

impl RegistryItem for NearSignature {
    fn get_registry_type() -> RegistryType<'static> {
        NEAR_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for NearSignature {
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
        let len = self.signature.len().try_into().unwrap();
        e.array(len)?;
        for ele in &self.signature {
            e.bytes(ele)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for NearSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = NearSignature::default();
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
                    obj.signature = Vec::new();
                    if len.is_some() {
                        for _ in 0..len.unwrap() {
                            obj.signature.push(d.bytes()?.to_vec());
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

impl To for NearSignature {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<NearSignature> for NearSignature {
    fn from_cbor(bytes: Vec<u8>) -> URResult<NearSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::near::near_signature::NearSignature;
    use crate::traits::{From as FromCbor, To};
    use alloc::vec;

    #[test]
    fn test_encode() {
        let request_id = Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap());
        let signature = vec![
            hex::decode("85C578F8CA68BF8D771F0346ED68C4170DF9EE9878CB76F3E2FAC425C3F5793D36A741547E245C6C7AC1B9433AD5FC523D41152CAC2A3726CBE134E0A0366802").unwrap()
        ];
        let near_signature = NearSignature::new(request_id, signature);
        assert_eq!(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0281584085c578f8ca68bf8d771f0346ed68c4170df9ee9878cb76f3e2fac425c3f5793d36a741547e245c6c7ac1b9433ad5fc523d41152cac2a3726cbe134e0a0366802",
            hex::encode(near_signature.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let request_id = hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap();
        let cbor = hex::decode("a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0281584085c578f8ca68bf8d771f0346ed68c4170df9ee9878cb76f3e2fac425c3f5793d36a741547e245c6c7ac1b9433ad5fc523d41152cac2a3726cbe134e0a0366802").unwrap();
        let signature = vec![
            hex::decode("85C578F8CA68BF8D771F0346ED68C4170DF9EE9878CB76F3E2FAC425C3F5793D36A741547E245C6C7AC1B9433AD5FC523D41152CAC2A3726CBE134E0A0366802").unwrap()
        ];
        let near_signature = NearSignature::from_cbor(cbor).unwrap();
        assert_eq!(request_id, near_signature.get_request_id().unwrap());
        assert_eq!(signature, near_signature.get_signature());
    }
}
