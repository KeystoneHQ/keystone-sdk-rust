//! Zcash PCZT batch signature result Registry Type.
//!
//! The payload is the opaque output of
//! `pczt::roles::signer::batch::BatchSignResponse::serialize`. The PCZT crate
//! owns the versioned response encoding and its semantic validation; this
//! registry type only provides the outer UR/CBOR envelope, the request id
//! copied from the corresponding signing request, and the signing device's
//! firmware version.

use alloc::{string::ToString, vec::Vec};

use minicbor::data::Int;

use crate::{
    error::{URError, URResult},
    registry_types::{RegistryType, ZCASH_BATCH_SIG_RESULT},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::cbor_helpers::{decode_definite_u8_map, reject_duplicate_key, require_key};

const DATA: u8 = 1;
const REQUEST_ID: u8 = 2;
const FIRMWARE_VERSION: u8 = 3;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ZcashBatchSigResult {
    data: Bytes,
    request_id: Bytes,
    firmware_version: Bytes,
}

impl ZcashBatchSigResult {
    pub fn new(request_id: Bytes, data: Bytes, firmware_version: Bytes) -> Self {
        Self {
            data,
            request_id,
            firmware_version,
        }
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    pub fn get_request_id(&self) -> &[u8] {
        &self.request_id
    }

    pub fn get_firmware_version(&self) -> &[u8] {
        &self.firmware_version
    }
}

impl MapSize for ZcashBatchSigResult {
    fn map_size(&self) -> u64 {
        3
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
        e.int(Int::from(REQUEST_ID))?.bytes(&self.request_id)?;
        e.int(Int::from(FIRMWARE_VERSION))?
            .bytes(&self.firmware_version)?;
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
        decode_definite_u8_map(
            d,
            &mut result,
            "indefinite zcash-batch-sig-result map is unsupported",
            |key, obj, d| {
                reject_duplicate_key(
                    &mut seen_keys,
                    key,
                    d,
                    "duplicate key in zcash-batch-sig-result map",
                )?;
                match key {
                    DATA => obj.data = d.bytes()?.to_vec(),
                    REQUEST_ID => obj.request_id = d.bytes()?.to_vec(),
                    FIRMWARE_VERSION => obj.firmware_version = d.bytes()?.to_vec(),
                    _ => d.skip()?,
                }
                Ok(())
            },
        )?;
        require_key(&seen_keys, DATA, d, "missing zcash-batch-sig-result data")?;
        require_key(
            &seen_keys,
            REQUEST_ID,
            d,
            "missing zcash-batch-sig-result request id",
        )?;
        require_key(
            &seen_keys,
            FIRMWARE_VERSION,
            d,
            "missing zcash-batch-sig-result firmware version",
        )?;
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

    fn firmware_version() -> Vec<u8> {
        vec![1, 2, 3]
    }

    #[test]
    fn round_trip() {
        let data = empty_batch_response();
        let request_id = vec![0xaa, 0xbb];
        let firmware_version = firmware_version();
        let result =
            ZcashBatchSigResult::new(request_id.clone(), data.clone(), firmware_version.clone());

        let encoded: Vec<u8> = result.try_into().unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), data);
        assert_eq!(decoded.get_request_id(), request_id);
        assert_eq!(decoded.get_firmware_version(), firmware_version);
    }

    #[test]
    fn wire_encoding_is_stable() {
        let encoded: Vec<u8> =
            ZcashBatchSigResult::new(vec![0xaa, 0xbb], empty_batch_response(), firmware_version())
                .try_into()
                .unwrap();

        assert_eq!(
            hex::encode(encoded),
            "a3014950435a5301000000000242aabb0343010203"
        );
    }

    #[test]
    fn preserves_empty_data() {
        let encoded: Vec<u8> =
            ZcashBatchSigResult::new(vec![0xaa, 0xbb], vec![], firmware_version())
                .try_into()
                .unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert!(decoded.get_data().is_empty());
    }

    #[test]
    fn preserves_empty_request_id() {
        let encoded: Vec<u8> =
            ZcashBatchSigResult::new(vec![], empty_batch_response(), firmware_version())
                .try_into()
                .unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert!(decoded.get_request_id().is_empty());
    }

    #[test]
    fn rejects_missing_data() {
        let err = ZcashBatchSigResult::try_from(vec![
            0xa2,
            REQUEST_ID,
            0x40,
            FIRMWARE_VERSION,
            0x43,
            1,
            2,
            3,
        ])
        .unwrap_err();

        assert!(err
            .to_string()
            .contains("missing zcash-batch-sig-result data"));
    }

    #[test]
    fn rejects_missing_request_id() {
        let err =
            ZcashBatchSigResult::try_from(vec![0xa2, DATA, 0x40, FIRMWARE_VERSION, 0x43, 1, 2, 3])
                .unwrap_err();

        assert!(err
            .to_string()
            .contains("missing zcash-batch-sig-result request id"));
    }

    #[test]
    fn rejects_missing_firmware_version() {
        let err =
            ZcashBatchSigResult::try_from(vec![0xa2, DATA, 0x40, REQUEST_ID, 0x40]).unwrap_err();

        assert!(err
            .to_string()
            .contains("missing zcash-batch-sig-result firmware version"));
    }

    #[test]
    fn rejects_duplicate_firmware_version() {
        let err = ZcashBatchSigResult::try_from(vec![
            0xa4,
            DATA,
            0x40,
            REQUEST_ID,
            0x40,
            FIRMWARE_VERSION,
            0x43,
            1,
            2,
            3,
            FIRMWARE_VERSION,
            0x43,
            1,
            2,
            3,
        ])
        .unwrap_err();

        assert!(err
            .to_string()
            .contains("duplicate key in zcash-batch-sig-result map"));
    }

    #[test]
    fn rejects_indefinite_map() {
        let err = ZcashBatchSigResult::try_from(vec![
            0xbf,
            DATA,
            0x40,
            REQUEST_ID,
            0x40,
            FIRMWARE_VERSION,
            0x43,
            1,
            2,
            3,
            0xff,
        ])
        .unwrap_err();

        assert!(err
            .to_string()
            .contains("indefinite zcash-batch-sig-result map is unsupported"));
    }

    #[test]
    fn rejects_trailing_data() {
        let result =
            ZcashBatchSigResult::new(vec![0xaa, 0xbb], empty_batch_response(), firmware_version());
        let mut encoded: Vec<u8> = result.try_into().unwrap();
        encoded.push(0);

        let err = ZcashBatchSigResult::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn skips_unknown_keys() {
        let encoded = hex::decode("a4014950435a5301000000000242aabb03430102030982010a").unwrap();

        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_data(), empty_batch_response());
        assert_eq!(decoded.get_request_id(), &[0xaa, 0xbb]);
        assert_eq!(decoded.get_firmware_version(), &[1, 2, 3]);
    }

    #[test]
    fn registry_type() {
        assert_eq!(
            ZcashBatchSigResult::get_registry_type().get_type(),
            "zcash-batch-sig-result"
        );
    }
}
