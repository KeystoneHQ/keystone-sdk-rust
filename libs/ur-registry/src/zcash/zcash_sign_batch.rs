//! Zcash signing batch Registry Type.
//!
//! This module implements CBOR encoding and decoding for batches of Zcash
//! signing messages. Each message carries a stable message id, a kind, and the
//! raw payload that should be signed.
//!
//! This is a registry container, not a protocol policy validator. Decode checks
//! CBOR shape, required fields, duplicate CBOR map keys, and trailing data, then
//! preserves registry values as supplied. Callers enforce policy such as
//! supported versions, networks, message kinds, unique message ids, digest
//! validity, and batch signing semantics.

use super::cbor_helpers::{reject_duplicate_key, require_key};
use crate::{
    registry_types::{RegistryType, ZCASH_SIGN_BATCH},
    traits::{MapSize, RegistryItem},
};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::{Decoder, Encoder};

use crate::error::{URError, URResult};

/// Registered batch version used by producers. Decode preserves any `u32`
/// version so callers can decide protocol policy.
pub const ZCASH_SIGN_BATCH_VERSION: u32 = 1;
/// Registered mainnet network value used by producers. Decode preserves any
/// `u32` network so callers can decide protocol policy.
pub const ZCASH_SIGN_BATCH_NETWORK_MAINNET: u32 = 1;
/// Registered PCZT v1 message kind used by producers. Decode preserves any
/// `u32` kind so callers can decide protocol policy.
pub const ZCASH_SIGN_MESSAGE_KIND_PCZT_V1: u32 = 1;

const VERSION: u8 = 1;
const REQUEST_ID: u8 = 2;
const NETWORK: u8 = 3;
const MESSAGES: u8 = 4;
const ATOMIC: u8 = 11;

const MESSAGE_ID: u8 = 1;
const MESSAGE_KIND: u8 = 2;
const MESSAGE_PAYLOAD: u8 = 3;
const MESSAGE_PAYLOAD_DIGEST: u8 = 6;

#[derive(Clone, Debug, Default)]
pub struct ZcashSignBatch {
    version: u32,
    request_id: Vec<u8>,
    network: u32,
    messages: Vec<ZcashSignMessage>,
    atomic: Option<bool>,
}

impl ZcashSignBatch {
    /// Builds a signing batch container. The SDK does not validate protocol
    /// policy such as supported version, network, or duplicate message ids here.
    pub fn new(
        version: u32,
        request_id: Vec<u8>,
        network: u32,
        messages: Vec<ZcashSignMessage>,
        atomic: Option<bool>,
    ) -> Self {
        Self {
            version,
            request_id,
            network,
            messages,
            atomic,
        }
    }

    pub fn get_version(&self) -> u32 {
        self.version
    }

    pub fn get_request_id(&self) -> &Vec<u8> {
        &self.request_id
    }

    pub fn get_network(&self) -> u32 {
        self.network
    }

    pub fn get_messages(&self) -> &Vec<ZcashSignMessage> {
        &self.messages
    }

    /// Returns the optional `atomic` field exactly as it appeared in the
    /// registry container. Use `get_atomic` for the effective default.
    pub fn get_atomic_field(&self) -> Option<bool> {
        self.atomic
    }

    /// Returns the effective batch semantics. An omitted `atomic` field
    /// defaults to `true`.
    pub fn get_atomic(&self) -> bool {
        self.atomic.unwrap_or(true)
    }
}

impl RegistryItem for ZcashSignBatch {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_SIGN_BATCH
    }
}

impl MapSize for ZcashSignBatch {
    fn map_size(&self) -> u64 {
        if self.atomic.is_some() {
            5
        } else {
            4
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ZcashSignMessage {
    id: Vec<u8>,
    kind: u32,
    payload: Vec<u8>,
    payload_digest: Option<Vec<u8>>,
}

impl ZcashSignMessage {
    /// Builds a signing message container. The SDK does not validate protocol
    /// policy such as supported kind, id uniqueness, or digest length here.
    pub fn new(id: Vec<u8>, kind: u32, payload: Vec<u8>, payload_digest: Option<Vec<u8>>) -> Self {
        Self {
            id,
            kind,
            payload,
            payload_digest,
        }
    }

    pub fn get_id(&self) -> &Vec<u8> {
        &self.id
    }

    pub fn get_kind(&self) -> u32 {
        self.kind
    }

    pub fn get_payload(&self) -> &Vec<u8> {
        &self.payload
    }

    pub fn get_payload_digest(&self) -> Option<&Vec<u8>> {
        self.payload_digest.as_ref()
    }
}

impl MapSize for ZcashSignMessage {
    fn map_size(&self) -> u64 {
        if self.payload_digest.is_some() {
            4
        } else {
            3
        }
    }
}

impl TryFrom<Vec<u8>> for ZcashSignBatch {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = Decoder::new(&value);
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

impl<C> minicbor::Encode<C> for ZcashSignBatch {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(VERSION))?.u32(self.version)?;
        e.int(Int::from(REQUEST_ID))?.bytes(&self.request_id)?;
        e.int(Int::from(NETWORK))?.u32(self.network)?;
        e.int(Int::from(MESSAGES))?
            .array(self.messages.len() as u64)?;
        for message in &self.messages {
            message.encode(e, ctx)?;
        }
        if let Some(atomic) = self.atomic {
            e.int(Int::from(ATOMIC))?.bool(atomic)?;
        }
        Ok(())
    }
}

impl<C> minicbor::Encode<C> for ZcashSignMessage {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(MESSAGE_ID))?.bytes(&self.id)?;
        e.int(Int::from(MESSAGE_KIND))?.u32(self.kind)?;
        e.int(Int::from(MESSAGE_PAYLOAD))?.bytes(&self.payload)?;
        if let Some(payload_digest) = self.payload_digest.as_ref() {
            e.int(Int::from(MESSAGE_PAYLOAD_DIGEST))?
                .bytes(payload_digest)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashSignBatch {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashSignBatch::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-sign-batch map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-sign-batch map",
            )?;
            match key {
                VERSION => result.version = d.u32()?,
                REQUEST_ID => result.request_id = d.bytes()?.to_vec(),
                NETWORK => result.network = d.u32()?,
                MESSAGES => {
                    let mut messages = vec![];
                    let len = d.array()?.ok_or_else(|| {
                        minicbor::decode::Error::message(
                            "indefinite zcash-sign-batch messages array is unsupported",
                        )
                        .at(d.position())
                    })?;
                    for _ in 0..len {
                        messages.push(ZcashSignMessage::decode(d, ctx)?);
                    }
                    result.messages = messages;
                }
                ATOMIC => result.atomic = Some(d.bool()?),
                _ => d.skip()?,
            }
        }
        require_key(&seen_keys, VERSION, d, "missing zcash-sign-batch version")?;
        require_key(
            &seen_keys,
            REQUEST_ID,
            d,
            "missing zcash-sign-batch request id",
        )?;
        require_key(&seen_keys, NETWORK, d, "missing zcash-sign-batch network")?;
        require_key(&seen_keys, MESSAGES, d, "missing zcash-sign-batch messages")?;
        Ok(result)
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashSignMessage {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashSignMessage::default();
        let len = d.map()?.ok_or_else(|| {
            minicbor::decode::Error::message("indefinite zcash-sign-message map is unsupported")
                .at(d.position())
        })?;
        let mut seen_keys = Vec::new();
        for _ in 0..len {
            let key = d.u8()?;
            reject_duplicate_key(
                &mut seen_keys,
                key,
                d,
                "duplicate key in zcash-sign-message map",
            )?;
            match key {
                MESSAGE_ID => result.id = d.bytes()?.to_vec(),
                MESSAGE_KIND => result.kind = d.u32()?,
                MESSAGE_PAYLOAD => result.payload = d.bytes()?.to_vec(),
                MESSAGE_PAYLOAD_DIGEST => result.payload_digest = Some(d.bytes()?.to_vec()),
                _ => d.skip()?,
            }
        }
        require_key(&seen_keys, MESSAGE_ID, d, "missing zcash-sign-message id")?;
        require_key(
            &seen_keys,
            MESSAGE_KIND,
            d,
            "missing zcash-sign-message kind",
        )?;
        require_key(
            &seen_keys,
            MESSAGE_PAYLOAD,
            d,
            "missing zcash-sign-message payload",
        )?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_zcash_sign_batch_encode_decode() {
        let payload_digest =
            hex::decode("ee9040f65c341855e070ff438eb0ea9d5b831b2a2c270fb7ef592d750408e3b3")
                .unwrap();
        let batch = ZcashSignBatch::new(
            ZCASH_SIGN_BATCH_VERSION,
            vec![0xaa, 0xbb],
            ZCASH_SIGN_BATCH_NETWORK_MAINNET,
            vec![ZcashSignMessage::new(
                vec![0x01],
                ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                vec![0x02, 0x03],
                Some(payload_digest.clone()),
            )],
            Some(false),
        );

        let encoded: Vec<u8> = batch.clone().try_into().unwrap();
        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_SIGN_BATCH_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_network(), ZCASH_SIGN_BATCH_NETWORK_MAINNET);
        assert_eq!(decoded.get_atomic_field(), Some(false));
        assert!(!decoded.get_atomic());
        assert_eq!(decoded.get_messages().len(), 1);
        assert_eq!(decoded.get_messages()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded.get_messages()[0].get_kind(),
            ZCASH_SIGN_MESSAGE_KIND_PCZT_V1
        );
        assert_eq!(decoded.get_messages()[0].get_payload(), &vec![0x02, 0x03]);
        assert_eq!(
            decoded.get_messages()[0].get_payload_digest(),
            Some(&payload_digest)
        );
    }

    #[test]
    fn test_zcash_sign_batch_decodes_literal_cbor_fixture() {
        let cbor = hex::decode(
            "a501010242aabb03010481a40141010201034c70637a742d726571756573740658207a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da40bf4",
        )
        .unwrap();
        let payload_digest =
            hex::decode("7a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4")
                .unwrap();

        let decoded = ZcashSignBatch::try_from(cbor.clone()).unwrap();

        assert_eq!(decoded.get_version(), ZCASH_SIGN_BATCH_VERSION);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_network(), ZCASH_SIGN_BATCH_NETWORK_MAINNET);
        assert_eq!(decoded.get_atomic_field(), Some(false));
        assert!(!decoded.get_atomic());
        assert_eq!(decoded.get_messages().len(), 1);
        assert_eq!(decoded.get_messages()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded.get_messages()[0].get_kind(),
            ZCASH_SIGN_MESSAGE_KIND_PCZT_V1
        );
        assert_eq!(
            decoded.get_messages()[0].get_payload(),
            &b"pczt-request".to_vec()
        );
        assert_eq!(
            decoded.get_messages()[0].get_payload_digest(),
            Some(&payload_digest)
        );
        assert_eq!(
            decoded.get_messages()[0]
                .get_payload_digest()
                .unwrap()
                .len(),
            32
        );

        let batch = ZcashSignBatch::new(
            ZCASH_SIGN_BATCH_VERSION,
            vec![0xaa, 0xbb],
            ZCASH_SIGN_BATCH_NETWORK_MAINNET,
            vec![ZcashSignMessage::new(
                vec![0x01],
                ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                b"pczt-request".to_vec(),
                Some(payload_digest),
            )],
            Some(false),
        );
        let encoded: Vec<u8> = batch.try_into().unwrap();

        assert_eq!(encoded, cbor);
    }

    #[test]
    fn test_zcash_sign_batch_skips_unknown_fields() {
        let fixtures = [
            // Unknown top level key 9 with value [1, {"x": true}].
            "a601010242aabb03010481a40141010201034c70637a742d726571756573740658207a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4098201a16178f50bf4",
            // Unknown nested message key 9 with value [1, {"x": true}].
            "a501010242aabb03010481a50141010201034c70637a742d72657175657374098201a16178f50658207a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da40bf4",
        ];
        let payload_digest =
            hex::decode("7a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4")
                .unwrap();

        for cbor_hex in fixtures {
            let decoded = ZcashSignBatch::try_from(hex::decode(cbor_hex).unwrap()).unwrap();

            assert_eq!(decoded.get_version(), ZCASH_SIGN_BATCH_VERSION);
            assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
            assert_eq!(decoded.get_network(), ZCASH_SIGN_BATCH_NETWORK_MAINNET);
            assert_eq!(decoded.get_atomic_field(), Some(false));
            assert_eq!(decoded.get_messages().len(), 1);
            assert_eq!(decoded.get_messages()[0].get_id(), &vec![0x01]);
            assert_eq!(
                decoded.get_messages()[0].get_kind(),
                ZCASH_SIGN_MESSAGE_KIND_PCZT_V1
            );
            assert_eq!(
                decoded.get_messages()[0].get_payload(),
                &b"pczt-request".to_vec()
            );
            assert_eq!(
                decoded.get_messages()[0].get_payload_digest(),
                Some(&payload_digest)
            );
        }
    }

    #[test]
    fn test_zcash_sign_batch_decodes_unknown_policy_values() {
        let batch = ZcashSignBatch::new(
            99,
            vec![0xaa, 0xbb],
            42,
            vec![ZcashSignMessage::new(
                vec![0x01],
                77,
                b"policy-is-external".to_vec(),
                None,
            )],
            Some(true),
        );
        let encoded: Vec<u8> = batch.try_into().unwrap();

        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_version(), 99);
        assert_eq!(decoded.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded.get_network(), 42);
        assert_eq!(decoded.get_atomic_field(), Some(true));
        assert!(decoded.get_atomic());
        assert_eq!(decoded.get_messages().len(), 1);
        assert_eq!(decoded.get_messages()[0].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_messages()[0].get_kind(), 77);
        assert_eq!(
            decoded.get_messages()[0].get_payload(),
            &b"policy-is-external".to_vec()
        );
    }

    #[test]
    fn test_zcash_sign_batch_decodes_duplicate_message_ids() {
        let batch = ZcashSignBatch::new(
            ZCASH_SIGN_BATCH_VERSION,
            vec![0xaa, 0xbb],
            ZCASH_SIGN_BATCH_NETWORK_MAINNET,
            vec![
                ZcashSignMessage::new(
                    vec![0x01],
                    ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                    b"first".to_vec(),
                    None,
                ),
                ZcashSignMessage::new(
                    vec![0x01],
                    ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                    b"second".to_vec(),
                    None,
                ),
            ],
            Some(true),
        );
        let encoded: Vec<u8> = batch.try_into().unwrap();

        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_messages().len(), 2);
        assert_eq!(decoded.get_messages()[0].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_messages()[1].get_id(), &vec![0x01]);
        assert_eq!(decoded.get_messages()[0].get_payload(), &b"first".to_vec());
        assert_eq!(decoded.get_messages()[1].get_payload(), &b"second".to_vec());
    }

    #[test]
    fn test_zcash_sign_batch_defaults_to_atomic() {
        let batch = ZcashSignBatch::new(1, vec![], 1, vec![], None);
        let encoded: Vec<u8> = batch.try_into().unwrap();
        let decoded = ZcashSignBatch::try_from(encoded).unwrap();

        assert_eq!(decoded.get_atomic_field(), None);
        assert!(decoded.get_atomic());
    }

    #[test]
    fn test_zcash_sign_batch_rejects_duplicate_keys() {
        let duplicate_version_keys = vec![0xa2, VERSION, 0x01, VERSION, 0x02];

        let err = ZcashSignBatch::try_from(duplicate_version_keys).unwrap_err();

        assert!(err.to_string().contains("duplicate key"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_duplicate_message_keys() {
        let duplicate_message_id_keys = vec![
            0xa4,
            VERSION,
            0x01,
            REQUEST_ID,
            0x40,
            NETWORK,
            0x01,
            MESSAGES,
            0x81,
            0xa4,
            MESSAGE_ID,
            0x40,
            MESSAGE_ID,
            0x40,
            MESSAGE_KIND,
            0x01,
            MESSAGE_PAYLOAD,
            0x40,
        ];

        let err = ZcashSignBatch::try_from(duplicate_message_id_keys).unwrap_err();

        assert!(err
            .to_string()
            .contains("duplicate key in zcash-sign-message map"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_trailing_data() {
        let batch = ZcashSignBatch::new(1, vec![], 1, vec![], None);
        let mut encoded: Vec<u8> = batch.try_into().unwrap();
        encoded.push(0x00);

        let err = ZcashSignBatch::try_from(encoded).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_indefinite_top_level_map() {
        let indefinite_map = vec![0xbf, 0xff];

        let err = ZcashSignBatch::try_from(indefinite_map).unwrap_err();

        assert!(err.to_string().contains("indefinite zcash-sign-batch map"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_indefinite_messages_array() {
        let indefinite_messages = vec![
            0xa4, VERSION, 0x01, REQUEST_ID, 0x40, NETWORK, 0x01, MESSAGES, 0x9f, 0xff,
        ];

        let err = ZcashSignBatch::try_from(indefinite_messages).unwrap_err();

        assert!(err.to_string().contains("messages array"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_indefinite_message_map() {
        let indefinite_message = vec![
            0xa4, VERSION, 0x01, REQUEST_ID, 0x40, NETWORK, 0x01, MESSAGES, 0x81, 0xbf, 0xff,
        ];

        let err = ZcashSignBatch::try_from(indefinite_message).unwrap_err();

        assert!(err
            .to_string()
            .contains("indefinite zcash-sign-message map"));
    }

    #[test]
    fn test_zcash_sign_batch_rejects_missing_required_top_level_key() {
        for (cbor_hex, message) in [
            ("a3024003010480", "missing zcash-sign-batch version"),
            ("a3010103010480", "missing zcash-sign-batch request id"),
            ("a3010102400480", "missing zcash-sign-batch network"),
            ("a3010102400301", "missing zcash-sign-batch messages"),
        ] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let err = ZcashSignBatch::try_from(cbor).unwrap_err();

            assert!(err.to_string().contains(message));
        }
    }

    #[test]
    fn test_zcash_sign_batch_rejects_missing_required_message_key() {
        for (cbor_hex, message) in [
            (
                "a40101024003010481a202010340",
                "missing zcash-sign-message id",
            ),
            (
                "a40101024003010481a201400340",
                "missing zcash-sign-message kind",
            ),
            (
                "a40101024003010481a201400201",
                "missing zcash-sign-message payload",
            ),
        ] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let err = ZcashSignBatch::try_from(cbor).unwrap_err();

            assert!(err.to_string().contains(message));
        }
    }

    #[test]
    fn test_registry_type() {
        assert_eq!(
            ZcashSignBatch::get_registry_type().get_type(),
            "zcash-sign-batch"
        );
    }
}
