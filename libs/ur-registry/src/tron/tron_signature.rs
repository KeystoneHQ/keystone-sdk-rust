use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, TRON_SIGNATURE, UUID};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct TronSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
}

impl TronSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Bytes) {
        self.signature = signature;
    }

    pub fn new(request_id: Option<Bytes>, signature: Bytes) -> Self {
        TronSignature { request_id, signature }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
}

impl RegistryItem for TronSignature {
    fn get_registry_type() -> RegistryType<'static> {
        TRON_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for TronSignature {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 1;
        if let Some(_) = &self.request_id {
            size = size + 1;
        }
        e.map(size)?;
        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }
        e.int(Int::from(SIGNATURE))?
            .bytes(&self.signature)?;
        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for TronSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = TronSignature::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
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

impl To for TronSignature {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<TronSignature> for TronSignature {
    fn from_cbor(bytes: Vec<u8>) -> URResult<TronSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, To};
    use hex::FromHex;
    use crate::tron::tron_signature::TronSignature;

    #[test]
    fn test_encode() {
        let request_id = Some([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec());
        let signature = [71, 177, 247, 123, 62, 48, 207, 187, 250, 65, 215, 149, 221, 52, 71, 88, 101, 36, 6, 23, 221, 28, 90, 123, 173, 82, 111, 95, 216, 158, 82, 205, 5, 124, 128, 182, 101, 204, 36, 49, 239, 171, 83, 82, 14, 43, 27, 146, 160, 66, 80, 51, 186, 238, 145, 93, 248, 88, 202, 28, 88, 139, 10, 24, 0].to_vec();
        let tron_signature = TronSignature::new(request_id, signature);
        assert_eq!(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584147b1f77b3e30cfbbfa41d795dd34475865240617dd1c5a7bad526f5fd89e52cd057c80b665cc2431efab53520e2b1b92a0425033baee915df858ca1c588b0a1800",
            hex::encode(tron_signature.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584147b1f77b3e30cfbbfa41d795dd34475865240617dd1c5a7bad526f5fd89e52cd057c80b665cc2431efab53520e2b1b92a0425033baee915df858ca1c588b0a1800",
        ).unwrap();

        let tron_signature = TronSignature::from_cbor(bytes).unwrap();
        assert_eq!([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec(), tron_signature.get_request_id().unwrap());
        assert_eq!([71, 177, 247, 123, 62, 48, 207, 187, 250, 65, 215, 149, 221, 52, 71, 88, 101, 36, 6, 23, 221, 28, 90, 123, 173, 82, 111, 95, 216, 158, 82, 205, 5, 124, 128, 182, 101, 204, 36, 49, 239, 171, 83, 82, 14, 43, 27, 146, 160, 66, 80, 51, 186, 238, 145, 93, 248, 88, 202, 28, 88, 139, 10, 24, 0].to_vec(), tron_signature.get_signature());
    }
}
