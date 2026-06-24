use alloc::string::{String, ToString};
use core::fmt;
use ur_registry::error::{URError, URResult};

pub fn cyclic_encode(
    message: &[u8],
    max_fragment_length: usize,
    ur_type: String,
) -> URResult<UREncodeResult> {
    let mut encoder = ur::Encoder::new(message, max_fragment_length, ur_type.clone())
        .map_err(|e| URError::CborEncodeError(e.to_string()))?;
    if encoder.fragment_count() > 1 {
        Ok(UREncodeResult {
            is_multi_part: true,
            data: encoder
                .next_part()
                .map_err(|e| URError::UrEncodeError(e.to_string()))?,
            encoder: Some(KeystoneUREncoder::new(encoder)),
        })
    } else {
        let ur = ur::encode(message, ur_type);
        Ok(UREncodeResult {
            is_multi_part: false,
            data: ur,
            encoder: None,
        })
    }
}

pub fn probe_encode(
    message: &[u8],
    max_fragment_length: usize,
    ur_type: String,
) -> URResult<UREncodeResult> {
    let mut encoder = ur::Encoder::new(message, max_fragment_length, ur_type.clone())
        .map_err(|e| URError::CborEncodeError(e.to_string()))?;
    if encoder.fragment_count() > 1 {
        Ok(UREncodeResult {
            is_multi_part: true,
            data: encoder
                .next_part()
                .map_err(|e| URError::UrEncodeError(e.to_string()))?,
            encoder: Some(KeystoneUREncoder::new(encoder)),
        })
    } else {
        let ur = ur::encode(message, ur_type);
        Ok(UREncodeResult {
            is_multi_part: false,
            data: ur,
            encoder: None,
        })
    }
}

pub struct UREncodeResult {
    pub is_multi_part: bool,
    pub data: String,
    pub encoder: Option<KeystoneUREncoder>,
}

pub struct KeystoneUREncoder {
    encoder: ur::Encoder,
}

impl fmt::Debug for UREncodeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.is_multi_part)
            .field(&self.data)
            .finish()
    }
}

impl KeystoneUREncoder {
    pub fn new(encoder: ur::Encoder) -> Self {
        KeystoneUREncoder { encoder }
    }

    pub fn next_cyclic_part(&mut self) -> URResult<String> {
        self.encoder
            .next_cyclic_part()
            .map_err(|e| URError::CborEncodeError(e.to_string()))
    }

    pub fn next_part(&mut self) -> URResult<String> {
        self.encoder
            .next_part()
            .map_err(|e| URError::CborEncodeError(e.to_string()))
    }

    pub fn current_index(&self) -> usize {
        self.encoder.current_index()
    }

    pub fn fragment_count(&self) -> usize {
        self.encoder.fragment_count()
    }
}

#[cfg(test)]
mod tests {
    use crate::keystone_ur_decoder::{probe_decode, MultiURParseResult, URParseResult};
    use crate::keystone_ur_encoder::{cyclic_encode, probe_encode};
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;
    use ur_registry::crypto_psbt::CryptoPSBT;
    use ur_registry::extend::qr_hardware_call::QRHardwareCall;
    use ur_registry::traits::{RegistryItem, UR};
    use ur_registry::zcash::zcash_sign_batch::{
        ZcashSignBatch, ZcashSignMessage, ZCASH_SIGN_BATCH_NETWORK_MAINNET,
        ZCASH_SIGN_BATCH_VERSION, ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
    };
    use ur_registry::zcash::zcash_sign_result::{
        ZcashSignMessageResult, ZcashSignResult, ZCASH_SIGN_RESULT_KIND_PCZT_V1,
        ZCASH_SIGN_RESULT_VERSION, ZCASH_SIGN_STATUS_SIGNED,
    };

    #[test]
    fn test_encode_decode_zcash_sign_batch_ur() {
        let payload_digest =
            Vec::from_hex("7a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4")
                .unwrap();
        let batch = ZcashSignBatch::new(
            ZCASH_SIGN_BATCH_VERSION,
            vec![0xaa, 0xbb],
            ZCASH_SIGN_BATCH_NETWORK_MAINNET,
            vec![ZcashSignMessage::new(
                vec![0x01],
                ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                b"pczt-request".to_vec(),
                Some(payload_digest.clone()),
            )],
            Some(false),
        );
        let cbor: Vec<u8> = batch.try_into().unwrap();
        let encoded =
            probe_encode(&cbor, 400, ZcashSignBatch::get_registry_type().get_type()).unwrap();
        let literal_ur = "ur:zcash-sign-batch/onadadaofwpkrkaxadaalyoxadfpadaoadaxgsjoiaknjydpjpihjskpihjkjyamhdcxkniyvarnayknzevtiygyhslfluttcasevoadzcrosatsdilnwyfdhydtldkelgoxbdwkeccarlis";

        assert!(!encoded.is_multi_part);
        assert_eq!(encoded.data, literal_ur);

        let decoded: URParseResult<ZcashSignBatch> = probe_decode(literal_ur.to_string()).unwrap();
        let decoded_batch = decoded.data.unwrap();

        assert_eq!(decoded.ur_type.unwrap().get_type_str(), "zcash-sign-batch");
        assert_eq!(decoded_batch.get_version(), ZCASH_SIGN_BATCH_VERSION);
        assert_eq!(decoded_batch.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(
            decoded_batch.get_network(),
            ZCASH_SIGN_BATCH_NETWORK_MAINNET
        );
        assert!(!decoded_batch.get_atomic());
        assert_eq!(decoded_batch.get_messages().len(), 1);
        assert_eq!(decoded_batch.get_messages()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded_batch.get_messages()[0].get_kind(),
            ZCASH_SIGN_MESSAGE_KIND_PCZT_V1
        );
        assert_eq!(
            decoded_batch.get_messages()[0].get_payload(),
            &b"pczt-request".to_vec()
        );
        assert_eq!(
            decoded_batch.get_messages()[0].get_payload_digest(),
            Some(&payload_digest)
        );
    }

    #[test]
    fn test_registry_ur_encoder_type_is_compatible() {
        let crypto = CryptoPSBT::new(vec![0xaa, 0xbb, 0xcc]);
        let mut encoder = super::KeystoneUREncoder::new(crypto.to_ur_encoder(400));

        assert_eq!(encoder.fragment_count(), 1);

        let part = encoder.next_part().unwrap();
        assert!(part.starts_with("ur:crypto-psbt/"));
    }

    #[test]
    fn test_encode_decode_zcash_sign_result_ur() {
        let payload_digest =
            Vec::from_hex("f2dbc955d1edad3014bc907efc15e93adb4412cdee847d261cd942998693e590")
                .unwrap();
        let result = ZcashSignResult::new(
            ZCASH_SIGN_RESULT_VERSION,
            vec![0xaa, 0xbb],
            vec![ZcashSignMessageResult::signed(
                vec![0x01],
                ZCASH_SIGN_RESULT_KIND_PCZT_V1,
                b"signed-pczt-result".to_vec(),
                payload_digest.clone(),
            )],
        );
        let cbor: Vec<u8> = result.try_into().unwrap();
        let encoded =
            probe_encode(&cbor, 400, ZcashSignResult::get_registry_type().get_type()).unwrap();
        let literal_ur = "ur:zcash-sign-result/otadadaofwpkrkaxlyonadfpadaoaeaxadaagmjkiniojtihiedpjoiaknjydpjpihjkkpjzjyamhdcxwzuysogottwepmdybbrfmhkbztbzwlftuyfybgsnwylrkidscetafwnllnmuvwmhuorkvwgr";

        assert!(!encoded.is_multi_part);
        assert_eq!(encoded.data, literal_ur);

        let decoded: URParseResult<ZcashSignResult> = probe_decode(literal_ur.to_string()).unwrap();
        let decoded_result = decoded.data.unwrap();

        assert_eq!(decoded.ur_type.unwrap().get_type_str(), "zcash-sign-result");
        assert_eq!(decoded_result.get_version(), ZCASH_SIGN_RESULT_VERSION);
        assert_eq!(decoded_result.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(decoded_result.get_results().len(), 1);
        assert_eq!(decoded_result.get_results()[0].get_id(), &vec![0x01]);
        assert_eq!(
            decoded_result.get_results()[0].get_status(),
            ZCASH_SIGN_STATUS_SIGNED
        );
        assert_eq!(
            decoded_result.get_results()[0].get_kind(),
            ZCASH_SIGN_RESULT_KIND_PCZT_V1
        );
        assert_eq!(
            decoded_result.get_results()[0].get_payload(),
            &b"signed-pczt-result".to_vec()
        );
        assert_eq!(
            decoded_result.get_results()[0].get_payload_digest(),
            &payload_digest
        );
    }

    #[test]
    fn test_encode_decode_multipart_zcash_sign_batch_ur() {
        let payload = vec![0x42; 1024];
        let payload_digest =
            Vec::from_hex("7a66e6be087afee0665161828bd11dc1e201fdb8c2d72786ee485e29897c8da4")
                .unwrap();
        let batch = ZcashSignBatch::new(
            ZCASH_SIGN_BATCH_VERSION,
            vec![0xaa, 0xbb],
            ZCASH_SIGN_BATCH_NETWORK_MAINNET,
            vec![ZcashSignMessage::new(
                vec![0x01],
                ZCASH_SIGN_MESSAGE_KIND_PCZT_V1,
                payload.clone(),
                Some(payload_digest.clone()),
            )],
            Some(true),
        );
        let cbor: Vec<u8> = batch.try_into().unwrap();
        let encoded =
            probe_encode(&cbor, 100, ZcashSignBatch::get_registry_type().get_type()).unwrap();

        assert!(encoded.is_multi_part);
        let first: URParseResult<ZcashSignBatch> = probe_decode(encoded.data).unwrap();
        assert!(first.is_multi_part);
        assert!(first.data.is_none());

        let mut decoder = first.decoder.unwrap();
        let mut encoder = encoded.encoder.unwrap();
        let fragment_count = encoder.fragment_count();
        let mut decoded_batch = None;
        for _ in 1..fragment_count {
            let part = encoder.next_part().unwrap();
            let parsed: MultiURParseResult<ZcashSignBatch> = decoder.parse_ur(part).unwrap();
            if parsed.is_complete {
                decoded_batch = parsed.data;
                break;
            }
        }
        let decoded_batch = decoded_batch.unwrap();

        assert_eq!(decoded_batch.get_version(), ZCASH_SIGN_BATCH_VERSION);
        assert_eq!(decoded_batch.get_request_id(), &vec![0xaa, 0xbb]);
        assert_eq!(
            decoded_batch.get_network(),
            ZCASH_SIGN_BATCH_NETWORK_MAINNET
        );
        assert!(decoded_batch.get_atomic());
        assert_eq!(decoded_batch.get_messages().len(), 1);
        assert_eq!(
            decoded_batch.get_messages()[0].get_kind(),
            ZCASH_SIGN_MESSAGE_KIND_PCZT_V1
        );
        assert_eq!(decoded_batch.get_messages()[0].get_payload(), &payload);
        assert_eq!(
            decoded_batch.get_messages()[0].get_payload_digest(),
            Some(&payload_digest)
        );
    }

    #[test]
    fn test_encode_ada_hardware_call() {
        let data = "a3010002d90515a10182d90516a101d90130a10186182cf500f500f5d90516a201d90130a1018a182cf51901f5f500f500f500f502010400";
        let data = Vec::from_hex(data).unwrap();
        // hardware call
        let res = probe_encode(&data, 400, QRHardwareCall::get_registry_type().get_type()).unwrap();
        assert_eq!(
            "ur:qr-hardware-call/otadaeaotaahbzoyadlftaahcmoyadtaaddyoyadlncsdwykaeykaeyktaahcmoeadtaaddyoyadlecsdwykcfadykykaeykaeykaeykaoadaaaeyteyldre",
            res.data
        )
    }
    //
    #[test]
    fn test_encode_sol_hardware_call() {
        let data = "a4010002d90515a10184d90516a301d90130a10186182cf5183cf500f502000463455448d90516a301d90130a10186182cf51901f5f500f502010463534f4cd90516a301d90130a10186182cf51901f5f501f502000463534f4cd90516a301d90130a1018a182cf51901f5f500f500f400f402010463534f4c036b4c6561702057616c6c65740401";
        let data = Vec::from_hex(data).unwrap();
        // hardware call
        let res = probe_encode(&data, 400, QRHardwareCall::get_registry_type().get_type()).unwrap();
        assert_eq!(
                "ur:qr-hardware-call/oxadaeaotaahbzoyadlrtaahcmotadtaaddyoyadlncsdwykcsfnykaeykaoaeaaiafeghfdtaahcmotadtaaddyoyadlncsdwykcfadykykaeykaoadaaiagugwgstaahcmotadtaaddyoyadlncsdwykcfadykykadykaoaeaaiagugwgstaahcmotadtaaddyoyadlecsdwykcfadykykaeykaewkaewkaoadaaiagugwgsaxjegsihhsjocxhghsjzjzihjyaaadfnfxcmfy",
                res.data
            )
    }
    #[test]
    fn test_encode_cosmos_hardware_call() {
        let data = "a3010002d90515a10182d90516a101d90130a10186182cf500f500f5d90516a201d90130a1018a182cf51901f5f500f500f500f502010400";
        let data = Vec::from_hex(data).unwrap();
        // hardware call
        let res = probe_encode(&data, 400, QRHardwareCall::get_registry_type().get_type()).unwrap();
        assert_eq!(
            "ur:qr-hardware-call/otadaeaotaahbzoyadlftaahcmoyadtaaddyoyadlncsdwykaeykaeyktaahcmoeadtaaddyoyadlecsdwykcfadykykaeykaeykaeykaoadaaaeyteyldre",
            res.data
        )
    }

    #[test]
    fn test_encode() {
        let crypto = CryptoPSBT::new(
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap());
        let result: Vec<u8> = crypto.try_into().unwrap();
        let result =
            probe_encode(&result, 400, CryptoPSBT::get_registry_type().get_type()).unwrap();
        assert_eq!("ur:crypto-psbt/1-3/lpadaxcfaxiacyvwhdfhndhkadclhkaxhnlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbnychpmiy",
                   result.data);
        if result.is_multi_part {
            let mut encoder = result.encoder.unwrap();
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/2-3/lpaoaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaylbntahvo",
                       next);
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/3-3/lpaxaxcfaxiacyvwhdfhndhkadclpklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypknseoskve",
                       next
            );
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/4-3/lpaaaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbayneieyksn",
                       next);
        }
    }

    #[test]
    fn test_cyclic_encode() {
        let crypto = CryptoPSBT::new(
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap());
        let result: Vec<u8> = crypto.try_into().unwrap();
        let result =
            cyclic_encode(&result, 400, CryptoPSBT::get_registry_type().get_type()).unwrap();
        assert_eq!("ur:crypto-psbt/1-3/lpadaxcfaxiacyvwhdfhndhkadclhkaxhnlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbnychpmiy",
                    result.data);
        if result.is_multi_part {
            let mut encoder = result.encoder.unwrap();
            let next = encoder.next_cyclic_part().unwrap();
            assert_eq!("ur:crypto-psbt/2-3/lpaoaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaylbntahvo",
                        next);
            let next = encoder.next_cyclic_part().unwrap();
            assert_eq!("ur:crypto-psbt/3-3/lpaxaxcfaxiacyvwhdfhndhkadclpklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypknseoskve",
                        next
            );
            let next = encoder.next_cyclic_part().unwrap();
            assert_eq!("ur:crypto-psbt/1-3/lpadaxcfaxiacyvwhdfhndhkadclhkaxhnlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbnychpmiy",
                        next);
        }
    }
}
