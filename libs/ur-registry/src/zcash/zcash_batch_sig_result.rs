//! Zcash signatures-only signing result Registry Type.
//!
//! This module implements CBOR encoding and decoding for a compact device
//! response to a Zcash signing batch. Instead of returning full (redacted)
//! PCZTs like [`super::zcash_sign_result::ZcashSignResult`], the device returns
//! only the produced signatures, correlated back to each request message by id
//! and to each spend by its pool and action index. The wallet re-applies these
//! signatures to the PCZTs it already holds, which is roughly 8-9x smaller on
//! the wire than echoing redacted PCZTs back.
//!
//! This is a registry container, not a protocol policy validator. Decode checks
//! CBOR shape, required fields, duplicate CBOR map keys, and trailing data, then
//! preserves registry values as supplied. Callers enforce policy such as request
//! correlation, supported versions, unique ids, pool validity, action-index
//! bounds, and signature length.

use super::cbor_helpers::{reject_duplicate_key, require_key};
use crate::{
    registry_types::{RegistryType, ZCASH_BATCH_SIG_RESULT},
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
pub const ZCASH_BATCH_SIG_RESULT_VERSION: u32 = 1;

/// Pool discriminant carried on each action signature so the wallet knows which
/// bundle to re-apply it to. These are Keystone-side wire discriminants, not
/// consensus values; the SDK preserves any `u32` and callers decide policy.
/// Registered pool discriminant for an Orchard action signature.
pub const ZCASH_SIG_POOL_ORCHARD: u32 = 0;
/// Registered pool discriminant for an Ironwood action signature (the
/// Keystone-side Orchard-to-Ironwood migration pool).
pub const ZCASH_SIG_POOL_IRONWOOD: u32 = 1;

/// A Zcash spend authorization signature is a 64-byte RedDSA signature. Decode
/// preserves any byte string so callers can decide policy.
pub const ZCASH_SIG_LEN: usize = 64;

// Top-level `zcash-batch-sig-result` map keys.
const VERSION: u8 = 1;
const REQUEST_ID: u8 = 2;
const RESULTS: u8 = 3;

// `msg-sig` map keys.
const MESSAGE_ID: u8 = 1;
const SIGS: u8 = 2;

// `action-sig` map keys.
const POOL: u8 = 1;
const ACTION_INDEX: u8 = 2;
const SIG: u8 = 3;

/// A signatures-only response to a Zcash signing batch: a version, the request
/// id it answers, and one [`ZcashMsgSig`] per signed message.
#[derive(Clone, Debug, Default)]
pub struct ZcashBatchSigResult {
    version: u32,
    request_id: Vec<u8>,
    results: Vec<ZcashMsgSig>,
}

impl ZcashBatchSigResult {
    /// Builds a signatures-only result container. The SDK does not validate
    /// protocol policy such as supported version, expected result count, or
    /// duplicate ids here.
    pub fn new(version: u32, request_id: Vec<u8>, results: Vec<ZcashMsgSig>) -> Self {
        Self {
            version,
            request_id,
            results,
        }
    }

    /// Returns the registered result version.
    pub fn get_version(&self) -> u32 {
        self.version
    }

    /// Returns the request id this result answers.
    pub fn get_request_id(&self) -> &Vec<u8> {
        &self.request_id
    }

    /// Returns the per-message signature results.
    pub fn get_results(&self) -> &Vec<ZcashMsgSig> {
        &self.results
    }
}

impl RegistryItem for ZcashBatchSigResult {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_BATCH_SIG_RESULT
    }
}

impl MapSize for ZcashBatchSigResult {
    fn map_size(&self) -> u64 {
        3
    }
}

/// The signatures produced for a single request message, correlated by id.
#[derive(Clone, Debug, Default)]
pub struct ZcashMsgSig {
    message_id: Vec<u8>,
    sigs: Vec<ZcashActionSig>,
}

impl ZcashMsgSig {
    /// Builds a per-message signature container. The SDK does not validate id
    /// correlation or signature policy here.
    pub fn new(message_id: Vec<u8>, sigs: Vec<ZcashActionSig>) -> Self {
        Self { message_id, sigs }
    }

    /// Returns the id of the request message these signatures answer.
    pub fn get_message_id(&self) -> &Vec<u8> {
        &self.message_id
    }

    /// Returns the per-action signatures for this message.
    pub fn get_sigs(&self) -> &Vec<ZcashActionSig> {
        &self.sigs
    }
}

impl MapSize for ZcashMsgSig {
    fn map_size(&self) -> u64 {
        2
    }
}

/// A single spend-authorization signature, located by pool and action index.
#[derive(Clone, Debug, Default)]
pub struct ZcashActionSig {
    pool: u32,
    action_index: u32,
    sig: Vec<u8>,
}

impl ZcashActionSig {
    /// Builds a single action signature container. The SDK does not validate
    /// pool validity, action-index bounds, or signature length here.
    pub fn new(pool: u32, action_index: u32, sig: Vec<u8>) -> Self {
        Self {
            pool,
            action_index,
            sig,
        }
    }

    /// Returns the pool discriminant (see [`ZCASH_SIG_POOL_ORCHARD`] /
    /// [`ZCASH_SIG_POOL_IRONWOOD`]).
    pub fn get_pool(&self) -> u32 {
        self.pool
    }

    /// Returns the index of the signed action within its pool's bundle.
    pub fn get_action_index(&self) -> u32 {
        self.action_index
    }

    /// Returns the raw signature bytes (expected length [`ZCASH_SIG_LEN`]).
    pub fn get_sig(&self) -> &Vec<u8> {
        &self.sig
    }
}

impl MapSize for ZcashActionSig {
    fn map_size(&self) -> u64 {
        3
    }
}

impl TryFrom<Vec<u8>> for ZcashBatchSigResult {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = Decoder::new(&value);
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

impl<C> minicbor::Encode<C> for ZcashBatchSigResult {
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

impl<C> minicbor::Encode<C> for ZcashMsgSig {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(MESSAGE_ID))?.bytes(&self.message_id)?;
        e.int(Int::from(SIGS))?.array(self.sigs.len() as u64)?;
        for sig in &self.sigs {
            sig.encode(e, ctx)?;
        }
        Ok(())
    }
}

impl<C> minicbor::Encode<C> for ZcashActionSig {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(POOL))?.u32(self.pool)?;
        e.int(Int::from(ACTION_INDEX))?.u32(self.action_index)?;
        e.int(Int::from(SIG))?.bytes(&self.sig)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashBatchSigResult {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashBatchSigResult::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-batch-sig-result map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-batch-sig-result map",
            )?;
            match key {
                VERSION => result.version = d.u32()?,
                REQUEST_ID => result.request_id = d.bytes()?.to_vec(),
                RESULTS => {
                    let mut results = vec![];
                    let len = d.array()?.ok_or_else(|| {
                        minicbor::decode::Error::message(
                            "indefinite zcash-batch-sig-result results array is unsupported",
                        )
                        .at(d.position())
                    })?;
                    for _ in 0..len {
                        results.push(ZcashMsgSig::decode(d, ctx)?);
                    }
                    result.results = results;
                }
                _ => d.skip()?,
            }
        }
        require_key(
            &seen_keys,
            VERSION,
            d,
            "missing zcash-batch-sig-result version",
        )?;
        require_key(
            &seen_keys,
            REQUEST_ID,
            d,
            "missing zcash-batch-sig-result request id",
        )?;
        require_key(
            &seen_keys,
            RESULTS,
            d,
            "missing zcash-batch-sig-result results",
        )?;
        Ok(result)
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashMsgSig {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashMsgSig::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-msg-sig map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(&mut seen_keys, key, d, "duplicate key in zcash-msg-sig map")?;
            match key {
                MESSAGE_ID => result.message_id = d.bytes()?.to_vec(),
                SIGS => {
                    let mut sigs = vec![];
                    let len = d.array()?.ok_or_else(|| {
                        minicbor::decode::Error::message(
                            "indefinite zcash-msg-sig sigs array is unsupported",
                        )
                        .at(d.position())
                    })?;
                    for _ in 0..len {
                        sigs.push(ZcashActionSig::decode(d, ctx)?);
                    }
                    result.sigs = sigs;
                }
                _ => d.skip()?,
            }
        }
        require_key(
            &seen_keys,
            MESSAGE_ID,
            d,
            "missing zcash-msg-sig message id",
        )?;
        require_key(&seen_keys, SIGS, d, "missing zcash-msg-sig sigs")?;
        Ok(result)
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashActionSig {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashActionSig::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-action-sig map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-action-sig map",
            )?;
            match key {
                POOL => result.pool = d.u32()?,
                ACTION_INDEX => result.action_index = d.u32()?,
                SIG => result.sig = d.bytes()?.to_vec(),
                _ => d.skip()?,
            }
        }
        require_key(&seen_keys, POOL, d, "missing zcash-action-sig pool")?;
        require_key(
            &seen_keys,
            ACTION_INDEX,
            d,
            "missing zcash-action-sig action index",
        )?;
        require_key(&seen_keys, SIG, d, "missing zcash-action-sig sig")?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_zcash_batch_sig_result_encode_decode() {
        let result = ZcashBatchSigResult::new(
            ZCASH_BATCH_SIG_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![
                ZcashMsgSig::new(
                    vec![0x01],
                    vec![
                        ZcashActionSig::new(ZCASH_SIG_POOL_ORCHARD, 0, vec![0x11; ZCASH_SIG_LEN]),
                        ZcashActionSig::new(ZCASH_SIG_POOL_IRONWOOD, 3, vec![0x22; ZCASH_SIG_LEN]),
                    ],
                ),
                ZcashMsgSig::new(
                    vec![0x02, 0x03],
                    vec![ZcashActionSig::new(
                        ZCASH_SIG_POOL_ORCHARD,
                        7,
                        vec![0x33; ZCASH_SIG_LEN],
                    )],
                ),
            ],
        );

        let encoded: Vec<u8> = result.clone().try_into().unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_BATCH_SIG_RESULT_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_results().len(), 2);

        let first = &decoded.get_results()[0];
        assert_eq!(first.get_message_id(), &vec![0x01]);
        assert_eq!(first.get_sigs().len(), 2);
        assert_eq!(first.get_sigs()[0].get_pool(), ZCASH_SIG_POOL_ORCHARD);
        assert_eq!(first.get_sigs()[0].get_action_index(), 0);
        assert_eq!(first.get_sigs()[0].get_sig(), &vec![0x11; ZCASH_SIG_LEN]);
        assert_eq!(first.get_sigs()[1].get_pool(), ZCASH_SIG_POOL_IRONWOOD);
        assert_eq!(first.get_sigs()[1].get_action_index(), 3);
        assert_eq!(first.get_sigs()[1].get_sig(), &vec![0x22; ZCASH_SIG_LEN]);

        let second = &decoded.get_results()[1];
        assert_eq!(second.get_message_id(), &vec![0x02, 0x03]);
        assert_eq!(second.get_sigs().len(), 1);
        assert_eq!(second.get_sigs()[0].get_pool(), ZCASH_SIG_POOL_ORCHARD);
        assert_eq!(second.get_sigs()[0].get_action_index(), 7);
        assert_eq!(second.get_sigs()[0].get_sig(), &vec![0x33; ZCASH_SIG_LEN]);
    }

    #[test]
    fn test_zcash_batch_sig_result_empty_results() {
        let result =
            ZcashBatchSigResult::new(ZCASH_BATCH_SIG_RESULT_VERSION, vec![0x01, 0x02], vec![]);

        let encoded: Vec<u8> = result.clone().try_into().unwrap();
        let decoded = ZcashBatchSigResult::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_BATCH_SIG_RESULT_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0x01, 0x02]);
        assert!(decoded.get_results().is_empty());
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_duplicate_keys() {
        let duplicate_version_keys = vec![0xa2, VERSION, 0x01, VERSION, 0x02];

        let err = ZcashBatchSigResult::try_from(duplicate_version_keys).unwrap_err();

        assert!(err.to_string().contains("duplicate key"));
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_trailing_data() {
        let result = ZcashBatchSigResult::new(ZCASH_BATCH_SIG_RESULT_VERSION, vec![], vec![]);
        let mut encoded: Vec<u8> = result.try_into().unwrap();
        encoded.push(0x00);

        let err = ZcashBatchSigResult::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_missing_required_top_level_key() {
        // map(2) { VERSION: 1, REQUEST_ID: h'01' } — missing RESULTS.
        let missing_results = vec![0xa2, VERSION, 0x01, REQUEST_ID, 0x41, 0x01];

        let err = ZcashBatchSigResult::try_from(missing_results).unwrap_err();

        assert!(
            err.to_string()
                .contains("missing zcash-batch-sig-result results")
        );
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_missing_required_action_key() {
        // map(3) {
        //   VERSION: 1,
        //   REQUEST_ID: h'01',
        //   RESULTS: [ map(2) { MESSAGE_ID: h'02', SIGS: [ map(2) { POOL: 0, SIG: h'' } ] } ]
        // } — the action map is missing ACTION_INDEX.
        let missing_action_index = vec![
            0xa3, VERSION, 0x01, REQUEST_ID, 0x41, 0x01, RESULTS, 0x81, 0xa2, MESSAGE_ID, 0x41,
            0x02, SIGS, 0x81, 0xa2, POOL, 0x00, SIG, 0x40,
        ];

        let err = ZcashBatchSigResult::try_from(missing_action_index).unwrap_err();

        assert!(
            err.to_string()
                .contains("missing zcash-action-sig action index")
        );
    }

    #[test]
    fn test_zcash_batch_sig_result_decodes_literal_cbor_fixture() {
        // { 1: 1, 2: h'aabb', 3: [ { 1: h'01', 2: [ { 1: 0, 2: 3, 3: h'11'*64 } ] } ] }
        let mut cbor = hex::decode("a301010242aabb0381a20141010281a301000203035840").unwrap();
        cbor.extend(vec![0x11u8; ZCASH_SIG_LEN]);

        let decoded = ZcashBatchSigResult::try_from(cbor.clone()).unwrap();
        assert_eq!(decoded.get_version(), ZCASH_BATCH_SIG_RESULT_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_results().len(), 1);
        assert_eq!(decoded.get_results()[0].get_message_id(), &vec![0x01]);
        assert_eq!(decoded.get_results()[0].get_sigs().len(), 1);
        let action = &decoded.get_results()[0].get_sigs()[0];
        assert_eq!(action.get_pool(), ZCASH_SIG_POOL_ORCHARD);
        assert_eq!(action.get_action_index(), 3);
        assert_eq!(action.get_sig(), &vec![0x11u8; ZCASH_SIG_LEN]);

        // An independent wallet implementation consumes this wire format, so pin
        // it: re-encoding the equivalent value must reproduce these exact bytes.
        let reencoded: Vec<u8> = ZcashBatchSigResult::new(
            ZCASH_BATCH_SIG_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![ZcashMsgSig::new(
                vec![0x01],
                vec![ZcashActionSig::new(
                    ZCASH_SIG_POOL_ORCHARD,
                    3,
                    vec![0x11u8; ZCASH_SIG_LEN],
                )],
            )],
        )
        .try_into()
        .unwrap();
        assert_eq!(reencoded, cbor);
    }

    #[test]
    fn test_zcash_batch_sig_result_skips_unknown_fields() {
        // Base value { 1:1, 2:h'', 3:[ { 1:h'', 2:[ { 1:0, 2:0, 3:h'' } ] } ] } with an
        // extra unknown key 9 -> [1, {"x": true}]: first at the top level (map a3->a4),
        // then inside the action-sig map (map a3->a4). Both must decode the known fields.
        let fixtures = [
            "a4010102400381a201400281a3010002000340098201a16178f5",
            "a3010102400381a201400281a4010002000340098201a16178f5",
        ];
        for cbor_hex in fixtures {
            let decoded = ZcashBatchSigResult::try_from(hex::decode(cbor_hex).unwrap()).unwrap();
            assert_eq!(decoded.get_version(), ZCASH_BATCH_SIG_RESULT_VERSION);
            assert_eq!(decoded.get_results().len(), 1);
            assert_eq!(decoded.get_results()[0].get_sigs().len(), 1);
            let action = &decoded.get_results()[0].get_sigs()[0];
            assert_eq!(action.get_pool(), ZCASH_SIG_POOL_ORCHARD);
            assert_eq!(action.get_action_index(), 0);
        }
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_indefinite_top_level_map() {
        let err = ZcashBatchSigResult::try_from(vec![0xbf, 0xff]).unwrap_err();

        assert!(
            err.to_string()
                .contains("indefinite zcash-batch-sig-result map")
        );
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_indefinite_results_array() {
        let indefinite_results = vec![0xa3, VERSION, 0x01, REQUEST_ID, 0x40, RESULTS, 0x9f, 0xff];

        let err = ZcashBatchSigResult::try_from(indefinite_results).unwrap_err();

        assert!(err.to_string().contains("results array"));
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_indefinite_msg_sig_map() {
        let indefinite_msg_sig =
            vec![0xa3, VERSION, 0x01, REQUEST_ID, 0x40, RESULTS, 0x81, 0xbf, 0xff];

        let err = ZcashBatchSigResult::try_from(indefinite_msg_sig).unwrap_err();

        assert!(err.to_string().contains("indefinite zcash-msg-sig map"));
    }

    #[test]
    fn test_zcash_batch_sig_result_rejects_duplicate_msg_sig_keys() {
        // A msg-sig map carrying MESSAGE_ID twice.
        let duplicate_message_id = vec![
            0xa3, VERSION, 0x01, REQUEST_ID, 0x40, RESULTS, 0x81, 0xa2, MESSAGE_ID, 0x40,
            MESSAGE_ID, 0x40,
        ];

        let err = ZcashBatchSigResult::try_from(duplicate_message_id).unwrap_err();

        assert!(
            err.to_string()
                .contains("duplicate key in zcash-msg-sig map")
        );
    }

    #[test]
    fn test_registry_type() {
        assert_eq!(
            ZcashBatchSigResult::get_registry_type().get_type(),
            "zcash-batch-sig-result"
        );
    }
}
