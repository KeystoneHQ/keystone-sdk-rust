use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::near::near_signature::NearSignature;
use ur_registry::traits::From;
use uuid::Uuid;
use ur_registry::registry_types::NEAR_SIGNATURE;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseNearSignature
	fn parse_near_signature(ur_type: &str, cbor_hex: &str) -> String {
        if NEAR_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, Vec<String>), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let near_signature = NearSignature::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let uuid = near_signature.get_request_id().ok_or(format_err!(""))?;
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = near_signature.get_signature().iter().map(|b| { hex::encode(b) }).collect();
            Ok((request_id, signature))
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
    fn test_parse_near_signature() {
        let near_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0281584085c578f8ca68bf8d771f0346ed68c4170df9ee9878cb76f3e2fac425c3f5793d36a741547e245c6c7ac1b9433ad5fc523d41152cac2a3726cbe134e0a0366802";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":[\"85c578f8ca68bf8d771f0346ed68c4170df9ee9878cb76f3e2fac425c3f5793d36a741547e245c6c7ac1b9433ad5fc523d41152cac2a3726cbe134e0a0366802\"]}";

        assert_eq!(expect_result, parse_near_signature("near-signature", near_signature_cbor));
    }

    #[test]
    fn test_parse_near_signature_type_error() {
        let near_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0281584085c578f8ca68bf8d771f0346ed68c4170df9ee9878cb76f3e2fac425c3f5793d36a741547e245c6c7ac1b9433ad5fc523d41152cac2a3726cbe134e0a0366802";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(expect_result, parse_near_signature("eth-signature", near_signature_cbor));
    }

    #[test]
    fn test_parse_near_signature_error() {
        let near_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(expect_result, parse_near_signature("near-signature", near_signature_cbor));
    }
}
