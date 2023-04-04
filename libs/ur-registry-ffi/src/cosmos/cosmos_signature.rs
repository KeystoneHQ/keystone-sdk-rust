use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::cosmos::cosmos_signature::CosmosSignature;
use ur_registry::traits::From;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseCosmosSignature
	fn parse_cosmos_signature(
		cbor_hex: &str
	) -> String {
        let parse_signature = || -> Result<(String, String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let sig = CosmosSignature::from_cbor(cbor)?;
            let uuid = sig.get_request_id();
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(sig.get_signature());
            let public_key = hex::encode(sig.get_public_key());
            Ok((request_id, signature, public_key))
        };
        match parse_signature() {
            Ok((request_id, signature, public_key)) => json!({
                "request_id": request_id,
                "signature": signature,
                "public_key": public_key,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_cosmos_signature() {
        let signature_cbor = "a301d825507afd5e09926743fba02e08c4a09417ec02584078325c2ea8d1841dbcd962e894ca6ecd5890aa4c1aa9e1eb789cd2d0e9c22ec737c2b4fb9c2defd863cadf914f538330ec42d6c30c04857ee1f06e7f2589d7d903582103f3ded94f2969d76200c6ed5db836041cc815fa62aa791e047905186c07e00275";
        let expect_result = "{\"public_key\":\"03f3ded94f2969d76200c6ed5db836041cc815fa62aa791e047905186c07e00275\",\"request_id\":\"7afd5e09-9267-43fb-a02e-08c4a09417ec\",\"signature\":\"78325c2ea8d1841dbcd962e894ca6ecd5890aa4c1aa9e1eb789cd2d0e9c22ec737c2b4fb9c2defd863cadf914f538330ec42d6c30c04857ee1f06e7f2589d7d9\"}";

        assert_eq!(expect_result, parse_cosmos_signature(signature_cbor));
    }

    #[test]
    fn test_parse_cosmos_signature_error() {
        let signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(expect_result, parse_cosmos_signature(signature_cbor));
    }
}
