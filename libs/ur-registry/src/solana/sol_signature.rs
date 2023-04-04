use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::cbor_map;
use crate::error::{URError, UrResult};
use crate::registry_types::{RegistryType, SOL_SIGNATURE, UUID};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct SolSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
}

impl SolSignature {
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
        SolSignature { request_id, signature }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
}

impl RegistryItem for SolSignature {
    fn get_registry_type() -> RegistryType<'static> {
        SOL_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for SolSignature {
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


impl<'b, C> minicbor::Decode<'b, C> for SolSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = SolSignature::default();
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

impl To for SolSignature {
    fn to_bytes(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<SolSignature> for SolSignature {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<SolSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, To};
    use hex::FromHex;
    use crate::solana::sol_signature::SolSignature;

    #[test]
    fn test_encode() {
        let request_id = Some([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec());
        let signature = [212, 240, 167, 188, 217, 91, 186, 31, 187, 16, 81, 136, 80, 84, 115, 14, 63, 71, 6, 66, 136, 87, 90, 172, 193, 2, 251, 191, 106, 154, 20, 218, 160, 102, 153, 30, 54, 13, 62, 52, 6, 194, 12, 0, 164, 9, 115, 239, 243, 124, 125, 100, 30, 91, 53, 30, 196, 169, 155, 254, 134, 243, 53, 247].to_vec();
        let sol_signature = SolSignature::new(request_id, signature);
        assert_eq!(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7",
            hex::encode(sol_signature.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7",
        )
            .unwrap();
        let sol_signature = SolSignature::from_cbor(bytes).unwrap();
        assert_eq!([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec(), sol_signature.get_request_id().unwrap());
        assert_eq!([212, 240, 167, 188, 217, 91, 186, 31, 187, 16, 81, 136, 80, 84, 115, 14, 63, 71, 6, 66, 136, 87, 90, 172, 193, 2, 251, 191, 106, 154, 20, 218, 160, 102, 153, 30, 54, 13, 62, 52, 6, 194, 12, 0, 164, 9, 115, 239, 243, 124, 125, 100, 30, 91, 53, 30, 196, 169, 155, 254, 134, 243, 53, 247].to_vec(), sol_signature.get_signature());
    }
}