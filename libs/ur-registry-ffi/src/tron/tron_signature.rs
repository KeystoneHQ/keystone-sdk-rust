use anyhow::format_err;
use anyhow::Error;
use hex;
use protobuf::Message;
use serde_json::json;
use ur_registry::keystone::keystone_sign_result::KeystoneSignResult;
use ur_registry::pb::protobuf_parser::parse_protobuf;
use ur_registry::pb::protobuf_parser::unzip;
use ur_registry::pb::protoc::payload::Content;
use ur_registry::pb::protoc::Base;
use ur_registry::registry_types::KEYSTONE_SIGN_RESULT;
use ur_registry::traits::From;

use crate::export;
use crate::tron::types::tron::Transaction;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseTronSignature
    fn parse_tron_signature(ur_type: &str, cbor_hex: &str) -> String {
        if KEYSTONE_SIGN_RESULT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let keystone_sign_result = KeystoneSignResult::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let ziped_sign_result = keystone_sign_result.get_sign_result();
            let sign_result = unzip(ziped_sign_result).map_err(|_| format_err!(""))?;
            let sign_result_base = parse_protobuf::<Base>(sign_result).map_err(|_| format_err!(""))?;
            let payload = sign_result_base.data.unwrap_or_default();
            let content = payload.content.unwrap();
            match content {
                Content::SignTxResult(sign_tx_result) => {
                    let request_id = sign_tx_result.sign_id;
                    match Transaction::parse_from_bytes(&hex::decode(sign_tx_result.raw_tx)?) {
                        Ok(tron_tx) => {
                            Ok((request_id, hex::encode(&tron_tx.signature[0])))
                        },
                        Err(_) => Err(format_err!(""))
                    }
                },
                _ => Err(format_err!(""))
            }
        };
        match parse_signature() {
            Ok((request_id, signature)) => json!({
                "request_id": request_id,
                "signature": signature,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_tron_signature() {
        let tron_signature_cbor = "a1015901bd1f8b08000000000000008d523d8e94310cd54aac342024a429b7a2a0a019c9761c3ba6a24053738538761a24462c349c85821b5071324e80bf866e259c2acf3fc97bcfa7bbf3ab4ff9fdebb7dbe77cfde571dd221f7edd9f9e9f4f576a57bb7ea0773fef5fbc31c748e7b834d7b8b0cfb898475cc821d45b2c9738bf4708696e0b1612766308304cd71d996da5ae4d2e991464440b779f16da1c3a590eebfcf0fb19cc60409840a006443012dd227b3791be449161c062d361eeb4fa9c59f50337d2b4a3b3a1b29a82744a51d975549674c1c24c5ba1adb0a0ada0245bb96e07b2283b176255afd2957a93a82e52e6239f7a64519a321606133be3889d0b5724ee9469b9da98284b436a7acb563c46c70587205cfa14f36563f65c13eac54aeebdcd9a9930b5b545963527622e36dd963b3c112533a96df498b6577da746a1f7b61a88000de1de5a6a7baaff3f623331282c28d760e8a1b701e000e3f458d09018990edad967efe527378d12aa1c09d8f51b825d245bcf42dc04dd8753a9422327e8e251aa90501fd29a61b0899197db0344b725c0613cd7aa64d916633a1869bd85735755772df7dffef873472fff6df1c7c7db5f8509bb54d6020000";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"42a9ece5a555a9437de74108d0fb5320f20835e108b961bb8b230228ea07c485412625863391d49692be558067f9e00559641f5ee63d8ab09275a51afe555b7e01\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("keystone-sign-result", tron_signature_cbor)
        );
    }

    #[test]
    fn test_parse_tron_signature_type_error() {
        let tron_signature_cbor = "a1015901bd1f8b08000000000000008d523d8e94310cd54aac342024a429b7a2a0a019c9761c3ba6a24053738538761a24462c349c85821b5071324e80bf866e259c2acf3fc97bcfa7bbf3ab4ff9fdebb7dbe77cfde571dd221f7edd9f9e9f4f576a57bb7ea0773fef5fbc31c748e7b834d7b8b0cfb898475cc821d45b2c9738bf4708696e0b1612766308304cd71d996da5ae4d2e991464440b779f16da1c3a590eebfcf0fb19cc60409840a006443012dd227b3791be449161c062d361eeb4fa9c59f50337d2b4a3b3a1b29a82744a51d975549674c1c24c5ba1adb0a0ada0245bb96e07b2283b176255afd2957a93a82e52e6239f7a64519a321606133be3889d0b5724ee9469b9da98284b436a7acb563c46c70587205cfa14f36563f65c13eac54aeebdcd9a9930b5b545963527622e36dd963b3c112533a96df498b6577da746a1f7b61a88000de1de5a6a7baaff3f623331282c28d760e8a1b701e000e3f458d09018990edad967efe527378d12aa1c09d8f51b825d245bcf42dc04dd8753a9422327e8e251aa90501fd29a61b0899197db0344b725c0613cd7aa64d916633a1869bd85735755772df7dffef873472fff6df1c7c7db5f8509bb54d6020000";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("eth-signature", tron_signature_cbor)
        );
    }

    #[test]
    fn test_parse_tron_signature_error() {
        let tron_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("keystone-sign-result", tron_signature_cbor)
        );
    }
}
