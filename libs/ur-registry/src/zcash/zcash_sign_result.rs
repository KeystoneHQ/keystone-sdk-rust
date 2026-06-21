//! Zcash signing result Registry Type.
//!
//! This module implements CBOR encoding and decoding for a device response to a
//! Zcash signing batch. Each result is correlated to the input message by id.
//!
//! This is a registry container, not a protocol policy validator. Decode checks
//! CBOR shape, required fields, duplicate CBOR map keys, and trailing data, then
//! preserves registry values as supplied. Callers enforce policy such as request
//! correlation, supported versions, result status, result kind, unique ids,
//! digest validity, and expected result count.

use super::{
    cbor_helpers::{reject_duplicate_key, require_key},
    zcash_sign_batch::ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
};
use crate::{
    registry_types::{RegistryType, ZCASH_SIGN_RESULT},
    traits::{MapSize, RegistryItem},
};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::{Decoder, Encoder};

use crate::error::{URError, URResult};

/// Registered result version used by producers. Decode preserves any `u32`
/// version so callers can decide protocol policy.
pub const ZCASH_SIGN_RESULT_VERSION: u32 = 1;
/// Registered PCZT v1 result kind used by producers. Result kind mirrors the
/// request message kind so callers can correlate request and response policy.
pub const ZCASH_SIGN_RESULT_KIND_PCZT_V1: u32 = ZCASH_SIGN_MESSAGE_KIND_PCZT_V1;
/// Registered status for a signed result. Decode preserves any `u32` status so
/// callers can decide protocol policy.
pub const ZCASH_SIGN_STATUS_SIGNED: u32 = 0;

const VERSION: u8 = 1;
const REQUEST_ID: u8 = 2;
const RESULTS: u8 = 3;

const MESSAGE_ID: u8 = 1;
const RESULT_STATUS: u8 = 2;
const RESULT_KIND: u8 = 3;
const RESULT_PAYLOAD: u8 = 4;
const RESULT_PAYLOAD_DIGEST: u8 = 6;

#[derive(Clone, Debug, Default)]
pub struct ZcashSignResult {
    version: u32,
    request_id: Vec<u8>,
    results: Vec<ZcashSignMessageResult>,
}

impl ZcashSignResult {
    /// Builds a signing result container. The SDK does not validate protocol
    /// policy such as supported version, expected result count, or duplicate ids
    /// here.
    pub fn new(version: u32, request_id: Vec<u8>, results: Vec<ZcashSignMessageResult>) -> Self {
        Self {
            version,
            request_id,
            results,
        }
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_request_id(&self) -> &Vec<u8> {
        &self.request_id
    }

    pub fn get_results(&self) -> &Vec<ZcashSignMessageResult> {
        &self.results
    }
}

impl RegistryItem for ZcashSignResult {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_SIGN_RESULT
    }
}

impl MapSize for ZcashSignResult {
    fn map_size(&self) -> u64 {
        3
    }
}

#[derive(Clone, Debug, Default)]
pub struct ZcashSignMessageResult {
    id: Vec<u8>,
    status: u32,
    kind: u32,
    payload: Vec<u8>,
    payload_digest: Vec<u8>,
}

impl ZcashSignMessageResult {
    /// Builds a message result container. The SDK does not validate protocol
    /// policy such as supported status, kind, id uniqueness, or digest length
    /// here.
    pub fn new(
        id: Vec<u8>,
        status: u32,
        kind: u32,
        payload: Vec<u8>,
        payload_digest: Vec<u8>,
    ) -> Self {
        Self {
            id,
            status,
            kind,
            payload,
            payload_digest,
        }
    }

    /// Builds a signed message result container with the registered signed
    /// status. Callers still validate kind, id correlation, and digest policy.
    pub fn signed(id: Vec<u8>, kind: u32, payload: Vec<u8>, payload_digest: Vec<u8>) -> Self {
        Self {
            id,
            status: ZCASH_SIGN_STATUS_SIGNED,
            kind,
            payload,
            payload_digest,
        }
    }

    pub fn get_id(&self) -> &Vec<u8> {
        &self.id
    }

    pub fn get_status(&self) -> u32 {
        self.status
    }

    pub fn get_kind(&self) -> u32 {
        self.kind
    }

    pub fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }

    pub fn get_payload_digest(&self) -> &Vec<u8> {
        &self.payload_digest
    }
}

impl MapSize for ZcashSignMessageResult {
    fn map_size(&self) -> u64 {
        5
    }
}

impl TryFrom<Vec<u8>> for ZcashSignResult {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = Decoder::new(&value);
        let result = <ZcashSignResult as minicbor::Decode<'_, ()>>::decode(&mut decoder, &mut ())
            .map_err(|e| URError::CborDecodeError(e.to_string()))?;
        if decoder.position() != value.len() {
            return Err(URError::CborDecodeError(
                "trailing data after zcash-sign-result".to_string(),
            ));
        }
        Ok(result)
    }
}

impl TryInto<Vec<u8>> for ZcashSignResult {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl<C> minicbor::Encode<C> for ZcashSignResult {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(VERSION))?.u32(self.version)?;
        e.int(Int::from(REQUEST_ID))?.bytes(&self.request_id)?;
        e.int(Int::from(RESULTS))?
            .array(self.results.len() as u64)?;
        for result in &self.results {
            result.encode(e, ctx)?;
        }
        Ok(())
    }
}

impl<C> minicbor::Encode<C> for ZcashSignMessageResult {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(MESSAGE_ID))?.bytes(&self.id)?;
        e.int(Int::from(RESULT_STATUS))?.u32(self.status)?;
        e.int(Int::from(RESULT_KIND))?.u32(self.kind)?;
        e.int(Int::from(RESULT_PAYLOAD))?.bytes(&self.payload)?;
        e.int(Int::from(RESULT_PAYLOAD_DIGEST))?
            .bytes(&self.payload_digest)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashSignResult {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashSignResult::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-sign-result map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-sign-result map",
            )?;
            match key {
                VERSION => result.version = d.u32()?,
                REQUEST_ID => result.request_id = d.bytes()?.to_vec(),
                RESULTS => {
                    let mut results = vec![];
                    let len = d.array()?.ok_or_else(|| {
                        minicbor::decode::Error::message(
                            "indefinite zcash-sign-result results array is unsupported",
                        )
                        .at(d.position())
                    })?;
                    for _ in 0..len {
                        results.push(ZcashSignMessageResult::decode(d, ctx)?);
                    }
                    result.results = results;
                }
                _ => d.skip()?,
            }
        }
        require_key(&seen_keys, VERSION, d, "missing zcash-sign-result version")?;
        require_key(
            &seen_keys,
            REQUEST_ID,
            d,
            "missing zcash-sign-result request id",
        )?;
        require_key(&seen_keys, RESULTS, d, "missing zcash-sign-result results")?;
        Ok(result)
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashSignMessageResult {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashSignMessageResult::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message(
                "indefinite zcash-sign-message-result map is unsupported",
            )
            .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-sign-message-result map",
            )?;
            match key {
                MESSAGE_ID => result.id = d.bytes()?.to_vec(),
                RESULT_STATUS => result.status = d.u32()?,
                RESULT_KIND => result.kind = d.u32()?,
                RESULT_PAYLOAD => result.payload = d.bytes()?.to_vec(),
                RESULT_PAYLOAD_DIGEST => result.payload_digest = d.bytes()?.to_vec(),
                _ => d.skip()?,
            }
        }
        require_key(
            &seen_keys,
            MESSAGE_ID,
            d,
            "missing zcash-sign-message-result id",
        )?;
        require_key(
            &seen_keys,
            RESULT_STATUS,
            d,
            "missing zcash-sign-message-result status",
        )?;
        require_key(
            &seen_keys,
            RESULT_KIND,
            d,
            "missing zcash-sign-message-result kind",
        )?;
        require_key(
            &seen_keys,
            RESULT_PAYLOAD,
            d,
            "missing zcash-sign-message-result payload",
        )?;
        require_key(
            &seen_keys,
            RESULT_PAYLOAD_DIGEST,
            d,
            "missing zcash-sign-message-result payload digest",
        )?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_zcash_sign_result_encode_decode() {
        let payload_digest =
            hex::decode("ee9040f65c341855e070ff438eb0ea9d5b831b2a2c270fb7ef592d750408e3b3")
                .unwrap();
        let result = ZcashSignResult::new(
            ZCASH_SIGN_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![ZcashSignMessageResult::signed(
                vec![0x01],
                ZCASH_SIGN_RESULT_KIND_PCZT_V1,
                vec![0x02, 0x03],
                payload_digest.clone(),
            )],
        );

        let encoded: Vec<u8> = result.clone().try_into().unwrap();
        let decoded = ZcashSignResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_SIGN_RESULT_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_results().len(), 1);
        assert_eq!(decoded.get_results()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded.get_results()[0].get_status(),
            ZCASH_SIGN_STATUS_SIGNED
        );
        assert_eq!(
            decoded.get_results()[0].get_kind(),
            ZCASH_SIGN_RESULT_KIND_PCZT_V1
        );
        assert_eq!(decoded.get_results()[0].get_payload(), &vec![0x02, 0x03]);
        assert_eq!(
            decoded.get_results()[0].get_payload_digest(),
            &payload_digest
        );
    }

    #[test]
    fn test_zcash_sign_result_decodes_literal_cbor_fixture() {
        let cbor = hex::decode(
            "a301010242aabb0381a50141010200030104527369676e65642d70637a742d726573756c74065820f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590",
        )
        .unwrap();
        let payload_digest =
            hex::decode("f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590")
                .unwrap();

        let decoded = ZcashSignResult::try_from(cbor.clone()).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_SIGN_RESULT_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_results().len(), 1);
        assert_eq!(decoded.get_results()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded.get_results()[0].get_status(),
            ZCASH_SIGN_STATUS_SIGNED
        );
        assert_eq!(
            decoded.get_results()[0].get_kind(),
            ZCASH_SIGN_RESULT_KIND_PCZT_V1
        );
        assert_eq!(
            decoded.get_results()[0].get_payload(),
            &b"signed-pczt-result".to_vec()
        );
        assert_eq!(
            decoded.get_results()[0].get_payload_digest(),
            &payload_digest
        );
        assert_eq!(decoded.get_results()[0].get_payload_digest().len(), 32);

        let result = ZcashSignResult::new(
            ZCASH_SIGN_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![ZcashSignMessageResult::signed(
                vec![0x01],
                ZCASH_SIGN_RESULT_KIND_PCZT_V1,
                b"signed-pczt-result".to_vec(),
                payload_digest,
            )],
        );
        let encoded: Vec<u8> = result.try_into().unwrap();

        assert_eq!(encoded, cbor);
    }

    #[test]
    fn test_zcash_sign_result_skips_unknown_fields() {
        let fixtures = [
            // Unknown top level key 9 with value [1, {"x": true}].
            "a401010242aabb0381a50141010200030104527369676e65642d70637a742d726573756c74065820f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590098201a16178f5",
            // Unknown nested result key 9 with value [1, {"x": true}].
            "a301010242aabb0381a60141010200030104527369676e65642d70637a742d726573756c74098201a16178f5065820f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590",
        ];
        let payload_digest =
            hex::decode("f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590")
                .unwrap();

        for cbor_hex in fixtures {
            let decoded = ZcashSignResult::try_from(hex::decode(cbor_hex).unwrap()).unwrap();

            assert_eq!(decoded.get_version(), ZCASH_SIGN_RESULT_VERSION);
            assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
            assert_eq!(decoded.get_results().len(), 1);
            assert_eq!(decoded.get_results()[0].get_id(), &vec![0x01]);
            assert_eq!(
                decoded.get_results()[0].get_status(),
                ZCASH_SIGN_STATUS_SIGNED
            );
            assert_eq!(
                decoded.get_results()[0].get_kind(),
                ZCASH_SIGN_RESULT_KIND_PCZT_V1
            );
            assert_eq!(
                decoded.get_results()[0].get_payload(),
                &b"signed-pczt-result".to_vec()
            );
            assert_eq!(
                decoded.get_results()[0].get_payload_digest(),
                &payload_digest
            );
        }
    }

    #[test]
    fn test_zcash_sign_result_decodes_unknown_policy_values() {
        let payload_digest =
            hex::decode("ee9040f65c341855e070ff438eb0ea9d5b831b2a2c270fb7ef592d750408e3b3")
                .unwrap();
        let result = ZcashSignResult::new(
            99,
            vec![0xaa, 0xbb],
            vec![ZcashSignMessageResult::new(
                vec![0x01],
                42,
                77,
                b"policy-is-external".to_vec(),
                payload_digest.clone(),
            )],
        );
        let encoded: Vec<u8> = result.try_into().unwrap();

        let decoded = ZcashSignResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), 99);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_results().len(), 1);
        assert_eq!(decoded.get_results()[0].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_results()[0].get_status(), 42);
        assert_eq!(decoded.get_results()[0].get_kind(), 77);
        assert_eq!(
            decoded.get_results()[0].get_payload(),
            &b"policy-is-external".to_vec()
        );
        assert_eq!(
            decoded.get_results()[0].get_payload_digest(),
            &payload_digest
        );
    }

    #[test]
    fn test_zcash_sign_result_decodes_duplicate_result_ids() {
        let first_digest =
            hex::decode("7a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4")
                .unwrap();
        let second_digest =
            hex::decode("f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590")
                .unwrap();
        let result = ZcashSignResult::new(
            ZCASH_SIGN_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![
                ZcashSignMessageResult::signed(
                    vec![0x01],
                    ZCASH_SIGN_RESULT_KIND_PCZT_V1,
                    b"first".to_vec(),
                    first_digest.clone(),
                ),
                ZcashSignMessageResult::signed(
                    vec![0x01],
                    ZCASH_SIGN_RESULT_KIND_PCZT_V1,
                    b"second".to_vec(),
                    second_digest.clone(),
                ),
            ],
        );
        let encoded: Vec<u8> = result.try_into().unwrap();

        let decoded = ZcashSignResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_results().len(), 2);
        assert_eq!(decoded.get_results()[0].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_results()[1].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_results()[0].get_payload(), &b"first".to_vec());
        assert_eq!(decoded.get_results()[1].get_payload(), &b"second".to_vec());
        assert_eq!(decoded.get_results()[0].get_payload_digest(), &first_digest);
        assert_eq!(
            decoded.get_results()[1].get_payload_digest(),
            &second_digest
        );
    }

    #[test]
    fn test_zcash_sign_result_rejects_duplicate_keys() {
        let duplicate_version_keys = vec![0xa2, VERSION, 0x01, VERSION, 0x02];

        let err = ZcashSignResult::try_from(duplicate_version_keys).unwrap_err();

        assert!(err.to_string().contains("duplicate key"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_duplicate_message_result_keys() {
        let duplicate_message_id_keys = vec![
            0xa3,
            VERSION,
            0x01,
            REQUEST_ID,
            0x40,
            RESULTS,
            0x81,
            0xa6,
            MESSAGE_ID,
            0x40,
            MESSAGE_ID,
            0x40,
            RESULT_STATUS,
            0x00,
            RESULT_KIND,
            0x01,
            RESULT_PAYLOAD,
            0x40,
            RESULT_PAYLOAD_DIGEST,
            0x40,
        ];

        let err = ZcashSignResult::try_from(duplicate_message_id_keys).unwrap_err();

        assert!(err
            .to_string()
            .contains("duplicate key in zcash-sign-message-result map"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_trailing_data() {
        let result = ZcashSignResult::new(ZCASH_SIGN_RESULT_VERSION, vec![], vec![]);
        let mut encoded: Vec<u8> = result.try_into().unwrap();
        encoded.push(0x00);

        let err = ZcashSignResult::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_indefinite_top_level_map() {
        let indefinite_map = vec![0xbf, 0xff];

        let err = ZcashSignResult::try_from(indefinite_map).unwrap_err();

        assert!(err.to_string().contains("indefinite zcash-sign-result map"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_indefinite_results_array() {
        let indefinite_results = vec![0xa3, VERSION, 0x01, REQUEST_ID, 0x40, RESULTS, 0x9f, 0xff];

        let err = ZcashSignResult::try_from(indefinite_results).unwrap_err();

        assert!(err.to_string().contains("results array"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_indefinite_message_result_map() {
        let indefinite_message_result = vec![
            0xa3, VERSION, 0x01, REQUEST_ID, 0x40, RESULTS, 0x81, 0xbf, 0xff,
        ];

        let err = ZcashSignResult::try_from(indefinite_message_result).unwrap_err();

        assert!(err
            .to_string()
            .contains("indefinite zcash-sign-message-result map"));
    }

    #[test]
    fn test_zcash_sign_result_rejects_missing_required_top_level_key() {
        for (cbor_hex, message) in [
            ("a202400380", "missing zcash-sign-result version"),
            ("a201010380", "missing zcash-sign-result request id"),
            ("a20101024101", "missing zcash-sign-result results"),
        ] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let err = ZcashSignResult::try_from(cbor).unwrap_err();

            assert!(err.to_string().contains(message));
        }
    }

    #[test]
    fn test_zcash_sign_result_rejects_missing_required_message_key() {
        for (cbor_hex, message) in [
            (
                "a3010102400381a40200030104400640",
                "missing zcash-sign-message-result id",
            ),
            (
                "a3010102400381a40140030104400640",
                "missing zcash-sign-message-result status",
            ),
            (
                "a3010102400381a40140020004400640",
                "missing zcash-sign-message-result kind",
            ),
            (
                "a3010102400381a40140020003010640",
                "missing zcash-sign-message-result payload",
            ),
            (
                "a3010102400381a40140020003010440",
                "missing zcash-sign-message-result payload digest",
            ),
        ] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let err = ZcashSignResult::try_from(cbor).unwrap_err();

            assert!(err.to_string().contains(message));
        }
    }

    #[test]
    fn test_registry_type() {
        assert_eq!(
            ZcashSignResult::get_registry_type().get_type(),
            "zcash-sign-result"
        );
    }
}
