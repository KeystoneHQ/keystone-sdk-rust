use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use crate::cbor::cbor_map;
use crate::error::{URError, UrResult};
use crate::registry_types::{ETH_SIGNATURE, RegistryType, UUID};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;
const ORIGIN: u8 = 3;

#[derive(Clone, Debug, Default)]
pub struct EthSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
    origin: Option<String>,
}

impl EthSignature {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_signature(&mut self, signature: Bytes) {
        self.signature = signature;
    }

    pub fn set_origin(&mut self, origin: String) { self.origin = Some(origin) }

    pub fn new(request_id: Option<Bytes>, signature: Bytes, origin: Option<String>) -> Self {
        EthSignature { request_id, signature, origin }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
    pub fn get_origin(&self) -> Option<String> { self.origin.clone() }
}

impl RegistryItem for EthSignature {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_SIGNATURE
    }
}


impl<C> minicbor::Encode<C> for EthSignature {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 1;
        if let Some(_) = &self.request_id {
            size = size + 1;
        }
        if let Some(_) = &self.origin {
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

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?
                .str(origin)?;
        }

        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for EthSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = EthSignature::default();
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
                ORIGIN => {
                    obj.origin = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for EthSignature {
    fn to_bytes(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<EthSignature> for EthSignature {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<EthSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, To};
    use hex::FromHex;
    use crate::ethereum::eth_signature::EthSignature;

    #[test]
    fn test_encode() {
        let request_id = Some([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec());
        let signature = [212, 240, 167, 188, 217, 91, 186, 31, 187, 16, 81, 136, 80, 84, 115, 14, 63, 71, 6, 66, 136, 87, 90, 172, 193, 2, 251, 191, 106, 154, 20, 218, 160, 102, 153, 30, 54, 13, 62, 52, 6, 194, 12, 0, 164, 9, 115, 239, 243, 124, 125, 100, 30, 91, 53, 30, 196, 169, 155, 254, 134, 243, 53, 247, 19].to_vec();
        let eth_signature = EthSignature::new(request_id, signature, Some("keystone".to_string()));
        assert_eq!(
            "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65",
            hex::encode(eth_signature.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65",
        )
            .unwrap();
        let eth_signature = EthSignature::from_cbor(bytes).unwrap();
        assert_eq!([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec(), eth_signature.get_request_id().unwrap());
        assert_eq!([212, 240, 167, 188, 217, 91, 186, 31, 187, 16, 81, 136, 80, 84, 115, 14, 63, 71, 6, 66, 136, 87, 90, 172, 193, 2, 251, 191, 106, 154, 20, 218, 160, 102, 153, 30, 54, 13, 62, 52, 6, 194, 12, 0, 164, 9, 115, 239, 243, 124, 125, 100, 30, 91, 53, 30, 196, 169, 155, 254, 134, 243, 53, 247, 19].to_vec(), eth_signature.get_signature());
        assert_eq!("keystone", eth_signature.get_origin().unwrap())
    }
}