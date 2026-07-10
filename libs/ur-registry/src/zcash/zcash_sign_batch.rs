//! Zcash PCZT batch signing request Registry Type.
//!
//! The payload is the opaque output of
//! `pczt::roles::signer::batch::BatchSignRequest::serialize`. The PCZT crate
//! owns the versioned request encoding and its semantic validation; this
//! registry type only provides the outer UR/CBOR envelope and the request id
//! used to correlate it with a signing result.

use alloc::{string::ToString, vec::Vec};

use minicbor::data::Int;

use crate::{
    cbor::cbor_map,
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_SIGN_BATCH},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::cbor_helpers::{reject_duplicate_key, require_key};

const DATA: u8 = 1;
const REQUEST_ID: u8 = 2;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZcashSignBatch {
    data: Bytes,
    request_id: Bytes,
}

impl ZcashSignBatch {
    pub fn new(request_id: Bytes, data: Bytes) -> Self {
        Self { data, request_id }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_request_id(&self) -> &[u8] {
        &self.request_id
    }
}

impl MapSize for ZcashSignBatch {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for ZcashSignBatch {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_SIGN_BATCH
    }
}

impl<C> minicbor::Encode<C> for ZcashSignBatch {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(DATA))?.bytes(&self.data)?;
        e.int(Int::from(REQUEST_ID))?.bytes(&self.request_id)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashSignBatch {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashSignBatch::default();
        let mut seen_keys = Vec::new();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-sign-batch map",
            )?;
            match key {
                DATA => obj.data = d.bytes()?.to_vec(),
                REQUEST_ID => obj.request_id = d.bytes()?.to_vec(),
                _ => d.skip()?,
            }
            Ok(())
        })?;
        require_key(&seen_keys, DATA, d, "missing zcash-sign-batch data")?;
        require_key(
            &seen_keys,
            REQUEST_ID,
            d,
            "missing zcash-sign-batch request id",
        )?;
        Ok(result)
    }
}

impl TryFrom<Vec<u8>> for ZcashSignBatch {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = minicbor::Decoder::new(&value);
        let batch = <ZcashSignBatch as minicbor::Decode<'_, ()>>::decode(&mut decoder, &mut ())
            .map_err(|e| URError::CborDecodeError(e.to_string()))?;
        if decoder.position() != value.len() {
            return Err(URError::CborDecodeError(
                "trailing data after zcash-sign-batch".to_string(),
            ));
        }
        Ok(batch)
    }
}

impl TryInto<Vec<u8>> for ZcashSignBatch {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    // Serialization of an empty PCZT BatchSignRequest. The request format is
    // "PCZB" || batch_version_le || pczt_version_le || Postcard body.
    fn empty_batch_request() -> Vec<u8> {
        vec![b'P', b'C', b'Z', b'B', 1, 0, 0, 0, 2, 0, 0, 0, 0]
    }

    #[test]
    fn round_trip() {
        let data = empty_batch_request();
        let request_id = vec![0xaa, 0xbb];
        let batch = ZcashSignBatch::new(request_id.clone(), data.clone());

        let encoded: Vec<u8> = batch.try_into().unwrap();
        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), data);
        assert_eq!(decoded.get_request_id(), request_id);
    }

    #[test]
    fn wire_encoding_is_stable() {
        let encoded: Vec<u8> = ZcashSignBatch::new(vec![0xaa, 0xbb], empty_batch_request())
            .try_into()
            .unwrap();

        assert_eq!(
            hex::encode(encoded),
            "a2014d50435a420100000002000000000242aabb"
        );
    }

    #[test]
    fn preserves_empty_data() {
        let encoded: Vec<u8> = ZcashSignBatch::new(vec![0xaa, 0xbb], vec![])
            .try_into()
            .unwrap();
        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert!(decoded.get_data().is_empty());
    }

    #[test]
    fn preserves_empty_request_id() {
        let encoded: Vec<u8> = ZcashSignBatch::new(vec![], empty_batch_request())
            .try_into()
            .unwrap();
        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert!(decoded.get_request_id().is_empty());
    }

    #[test]
    fn rejects_missing_data() {
        let err = ZcashSignBatch::try_from(vec![0xa1, REQUEST_ID, 0x40]).unwrap_err();

        assert!(err.to_string().contains("missing zcash-sign-batch data"));
    }

    #[test]
    fn rejects_missing_request_id() {
        let err = ZcashSignBatch::try_from(vec![0xa1, DATA, 0x40]).unwrap_err();

        assert!(err
            .to_string()
            .contains("missing zcash-sign-batch request id"));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = ZcashSignBatch::try_from(vec![0xa2, 0x01, 0x40, 0x01, 0x40]).unwrap_err();

        assert!(err
            .to_string()
            .contains("duplicate key in zcash-sign-batch map"));
    }

    #[test]
    fn rejects_trailing_data() {
        let mut encoded: Vec<u8> = ZcashSignBatch::new(vec![0xaa, 0xbb], empty_batch_request())
            .try_into()
            .unwrap();
        encoded.push(0);

        let err = ZcashSignBatch::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn skips_unknown_keys() {
        let encoded = hex::decode("a3014d50435a420100000002000000000242aabb0982010a").unwrap();

        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), empty_batch_request());
        assert_eq!(decoded.get_request_id(), &[0xaa, 0xbb]);
    }

    #[test]
    fn registry_type() {
        assert_eq!(
            ZcashSignBatch::get_registry_type().get_type(),
            "zcash-sign-batch"
        );
    }
}
