use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::traits::From;
use uuid::Uuid;
use ur_registry::registry_types::TRON_SIGNATURE;
use ur_registry::tron::tron_signature::TronSignature;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseTronSignature
	fn parse_tron_signature(
		ur_type: &str,
		cbor_hex: &str
	) -> String {
        if TRON_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let tron_signature = TronSignature::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let uuid = tron_signature.get_request_id().ok_or(format_err!(""))?;
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(tron_signature.get_signature());
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
    fn test_parse_tron_signature() {
        let tron_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584147b1f77b3e30cfbbfa41d795dd34475865240617dd1c5a7bad526f5fd89e52cd057c80b665cc2431efab53520e2b1b92a0425033baee915df858ca1c588b0a1800";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"47b1f77b3e30cfbbfa41d795dd34475865240617dd1c5a7bad526f5fd89e52cd057c80b665cc2431efab53520e2b1b92a0425033baee915df858ca1c588b0a1800\"}";

        assert_eq!(expect_result, parse_tron_signature("tron-signature", tron_signature_cbor));
    }

    #[test]
    fn test_parse_tron_signature_type_error() {
        let tron_signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(expect_result, parse_tron_signature("eth-signature", tron_signature_cbor));
    }

    #[test]
    fn test_parse_tron_signature_error() {
        let tron_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(expect_result, parse_tron_signature("tron-signature", tron_signature_cbor));
    }
}
