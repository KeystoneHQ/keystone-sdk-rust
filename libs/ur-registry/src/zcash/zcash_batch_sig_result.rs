//! Zcash PCZT batch signature result Registry Type.
//!
//! The payload is the opaque output of
//! `pczt::roles::signer::batch::BatchSignResponse::serialize`. The PCZT crate
//! owns the versioned response encoding and its semantic validation; this
//! registry type only provides the outer UR/CBOR envelope.

use alloc::{string::ToString, vec::Vec};

use minicbor::data::Int;

use crate::{
    cbor::cbor_map,
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_BATCH_SIG_RESULT},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::cbor_helpers::{reject_duplicate_key, require_key};

const DATA: u8 = 1;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZcashBatchSigResult {
    data: Bytes,
}

impl ZcashBatchSigResult {
    pub fn new(data: Bytes) -> Self {
        Self { data }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
}

impl MapSize for ZcashBatchSigResult {
    fn map_size(&self) -> u64 {
        1
    }
}

impl RegistryItem for ZcashBatchSigResult {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_BATCH_SIG_RESULT
    }
}

impl<C> minicbor::Encode<C> for ZcashBatchSigResult {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(DATA))?.bytes(&self.data)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashBatchSigResult {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashBatchSigResult::default();
        let mut seen_keys = Vec::new();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-batch-sig-result map",
            )?;
            match key {
                DATA => obj.data = d.bytes()?.to_vec(),
                _ => d.skip()?,
            }
            Ok(())
        })?;
        require_key(&seen_keys, DATA, d, "missing zcash-batch-sig-result data")?;
        Ok(result)
    }
}

impl TryFrom<Vec<u8>> for ZcashBatchSigResult {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = minicbor::Decoder::new(&value);
        let result =
            <ZcashBatchSigResult as minicbor::Decode<'_, ()>>::decode(&mut decoder, &mut ())
                .map_err(|e| URError::CborDecodeError(e.to_string()))?;
        if decoder.position() != value.len() {
            return Err(URError::CborDecodeError(
                "trailing data after zcash-batch-sig-result".to_string(),
            ));
        }
        Ok(result)
    }
}

impl TryInto<Vec<u8>> for ZcashBatchSigResult {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    // Serialization of an empty PCZT BatchSignResponse. The response format is
    // "PCZS" || batch_version_le || Postcard body.
    fn empty_batch_response() -> Vec<u8> {
        vec![b'P', b'C', b'Z', b'S', 1, 0, 0, 0, 0]
    }

    #[test]
    fn round_trip() {
        let data = empty_batch_response();
        let result = ZcashBatchSigResult::new(data.clone());

        let encoded: Vec<u8> = result.try_into().unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), data);
    }

    #[test]
    fn wire_encoding_is_stable() {
        let encoded: Vec<u8> = ZcashBatchSigResult::new(empty_batch_response())
            .try_into()
            .unwrap();

        assert_eq!(hex::encode(encoded), "a1014950435a530100000000");
    }

    #[test]
    fn preserves_empty_data() {
        let encoded: Vec<u8> = ZcashBatchSigResult::new(vec![]).try_into().unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert!(decoded.get_data().is_empty());
    }

    #[test]
    fn rejects_missing_data() {
        let err = ZcashBatchSigResult::try_from(vec![0xa1, 0x09, 0x00]).unwrap_err();

        assert!(err
            .to_string()
            .contains("missing zcash-batch-sig-result data"));
    }

    #[test]
    fn rejects_duplicate_keys() {
        let err = ZcashBatchSigResult::try_from(vec![0xa2, 0x01, 0x40, 0x01, 0x40]).unwrap_err();

        assert!(err
            .to_string()
            .contains("duplicate key in zcash-batch-sig-result map"));
    }

    #[test]
    fn rejects_trailing_data() {
        let mut encoded: Vec<u8> = ZcashBatchSigResult::new(empty_batch_response())
            .try_into()
            .unwrap();
        encoded.push(0);

        let err = ZcashBatchSigResult::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn skips_unknown_keys() {
        let encoded = hex::decode("a2014950435a5301000000000982010a").unwrap();

        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), empty_batch_response());
    }

    #[test]
    fn registry_type() {
        assert_eq!(
            ZcashBatchSigResult::get_registry_type().get_type(),
            "zcash-batch-sig-result"
        );
    }
}
