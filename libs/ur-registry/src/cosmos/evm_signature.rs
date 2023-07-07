// use alloc::string::ToString;
// use alloc::vec::Vec;
// use minicbor::data::{Int, Tag};
//
// use crate::cbor::cbor_map;
// use crate::error::{URError, URResult};
// use crate::registry_types::{RegistryType, COSMOS_SIGNATURE, UUID};
// use crate::traits::{From, RegistryItem, To};
// use crate::types::Bytes;
//
// const REQUEST_ID: u8 = 1;
// const SIGNATURE: u8 = 2;
// const PUBLIC_KEY: u8 = 3;
//
// #[derive(Clone, Debug, Default)]
// pub struct CosmosSignature {
//     request_id: Bytes,
//     signature: Bytes,
//     public_key: Bytes,
// }
//
// impl CosmosSignature {
//     pub fn default() -> Self {
//         Default::default()
//     }
//
//     pub fn set_request_id(&mut self, id: Bytes) {
//         self.request_id = id;
//     }
//
//     pub fn set_signature(&mut self, signature: Bytes) {
//         self.signature = signature;
//     }
//
//     pub fn set_public_key(&mut self, public_key: Bytes) {
//         self.public_key = public_key;
//     }
//
//     pub fn new(request_id: Bytes, signature: Bytes, public_key: Bytes) -> Self {
//         CosmosSignature {
//             request_id,
//             signature,
//             public_key,
//         }
//     }
//
//     pub fn get_request_id(&self) -> Bytes {
//         self.request_id.clone()
//     }
//     pub fn get_signature(&self) -> Bytes {
//         self.signature.clone()
//     }
//     pub fn get_public_key(&self) -> Bytes {
//         self.public_key.clone()
//     }
// }
//
// impl RegistryItem for CosmosSignature {
//     fn get_registry_type() -> RegistryType<'static> {
//         COSMOS_SIGNATURE
//     }
// }
//
// impl<C> minicbor::Encode<C> for CosmosSignature {
//     fn encode<W: minicbor::encode::Write>(
//         &self,
//         e: &mut minicbor::Encoder<W>,
//         _ctx: &mut C,
//     ) -> Result<(), minicbor::encode::Error<W::Error>> {
//         e.map(3)?;
//         e.int(
//             Int::try_from(REQUEST_ID)
//                 .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
//         )?
//         .tag(Tag::Unassigned(UUID.get_tag()))?
//         .bytes(&self.get_request_id())?;
//         e.int(
//             Int::try_from(SIGNATURE)
//                 .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
//         )?
//         .bytes(&self.get_signature())?;
//         e.int(
//             Int::try_from(PUBLIC_KEY)
//                 .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
//         )?
//         .bytes(&self.get_public_key())?;
//         Ok(())
//     }
// }
//
// impl<'b, C> minicbor::Decode<'b, C> for CosmosSignature {
//     fn decode(
//         d: &mut minicbor::Decoder<'b>,
//         _ctx: &mut C,
//     ) -> Result<Self, minicbor::decode::Error> {
//         let mut result = CosmosSignature::default();
//
//         cbor_map(d, &mut result, |key, obj, d| {
//             let key =
//                 u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
//             match key {
//                 REQUEST_ID => {
//                     let tag = d.tag()?;
//                     if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
//                         return Result::Err(minicbor::decode::Error::message("UUID tag is invalid"));
//                     }
//                     obj.request_id = d.bytes()?.to_vec();
//                 }
//                 SIGNATURE => {
//                     obj.signature = d.bytes()?.to_vec();
//                 }
//                 PUBLIC_KEY => {
//                     obj.public_key = d.bytes()?.to_vec();
//                 }
//                 _ => {}
//             }
//             Ok(())
//         })?;
//
//         Ok(result)
//     }
// }
//
// impl To for CosmosSignature {
//     fn to_bytes(&self) -> URResult<Vec<u8>> {
//         minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
//     }
// }
//
// impl From<CosmosSignature> for CosmosSignature {
//     fn from_cbor(bytes: Vec<u8>) -> URResult<CosmosSignature> {
//         minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
//     }
// }
