use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::aptos::aptos_signature::AptosSignature;
use ur_registry::traits::From;
use uuid::Uuid;
use ur_registry::registry_types::APTOS_SIGNATURE;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseAptosSignature
	fn parse_aptos_signature(ur_type: &str, cbor_hex: &str) -> String {
        if APTOS_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let sig = AptosSignature::from_cbor(cbor)?;
            let uuid = sig.get_request_id();
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(sig.get_signature());
            let authentication_public_key = hex::encode(sig.get_authentication_public_key());
            Ok((request_id, signature, authentication_public_key))
        };
        match parse_signature() {
            Ok((request_id, signature, authentication_public_key)) => json!({
                "request_id": request_id,
                "signature": signature,
                "authentication_public_key": authentication_public_key,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_aptos_signature() {
        let signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584047e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e0358208e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e";
        let expect_result = "{\"authentication_public_key\":\"8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e\",\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"47e7b510784406dfa14d9fd13c3834128b49c56ddfc28edb02c5047219779adeed12017e2f9f116e83762e86f805c7311ea88fb403ff21900e069142b1fb310e\"}";

        assert_eq!(expect_result, parse_aptos_signature("aptos-signature", signature_cbor));
    }

    #[test]
    fn test_parse_aptos_signature_type_error() {
        let signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(expect_result, parse_aptos_signature("eth-signature", signature_cbor));
    }

    #[test]
    fn test_parse_aptos_signature_error() {
        let signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(expect_result, parse_aptos_signature("aptos-signature", signature_cbor));
    }
}
