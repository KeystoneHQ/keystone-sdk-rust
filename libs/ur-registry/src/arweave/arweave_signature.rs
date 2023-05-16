use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, ARWEAVE_SIGNATURE, UUID};
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
pub struct ArweaveSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
}

impl ArweaveSignature {
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
        ArweaveSignature {
            request_id,
            signature,
        }
    }

    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_signature(&self) -> Bytes {
        self.signature.clone()
    }
}

impl RegistryItem for ArweaveSignature {
    fn get_registry_type() -> RegistryType<'static> {
        ARWEAVE_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for ArweaveSignature {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
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
        e.int(Int::from(SIGNATURE))?.bytes(&self.signature)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ArweaveSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ArweaveSignature::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
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

impl To for ArweaveSignature {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<ArweaveSignature> for ArweaveSignature {
    fn from_cbor(bytes: Vec<u8>) -> URResult<ArweaveSignature> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::solana::sol_signature::SolSignature;
    use crate::traits::{From as FromCbor, To};
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let request_id = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );
        let signature = hex::decode("80337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575").unwrap();
        let sol_signature = SolSignature::new(request_id, signature);
        assert_eq!(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0259020080337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575",
            hex::encode(sol_signature.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0259020080337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575",
        )
            .unwrap();
        let sol_signature = SolSignature::from_cbor(bytes).unwrap();
        assert_eq!(
            [155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec(),
            sol_signature.get_request_id().unwrap()
        );
        assert_eq!(hex::decode("80337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575").unwrap(), sol_signature.get_signature());
    }
}
