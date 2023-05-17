use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::sui::sui_signature::SuiSignature;
use ur_registry::registry_types::SUI_SIGNATURE;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseSuiSignature
    fn parse_sui_signature(ur_type: &str, cbor_hex: &str) -> String {
        if SUI_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let sig = SuiSignature::try_from(cbor)?;
            let mut request_id = String::from("");
            if let Some(uuid) = sig.get_request_id() {
                let uuid_hex = hex::encode(uuid);
                request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            }
            let signature = hex::encode(sig.get_signature());
            let mut public_key = String::from("");
            if let Some(pk) = sig.get_public_key() {
                public_key = hex::encode(pk);
            }
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
    fn test_parse_sui_signature() {
        let signature_cbor = "A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840B93921DB17F2F1D50BDA37B510F543151DF222E80946FEFBACFADFB2D4A79FDA4FACF0AE5B41D71EA3A7EBEA6AA88DE9577A788AEAB195B99B6A633C20E055030358207BAC671050FCBA0DD54F3930601C42AD36CC11BC0589ED8D3CEF3EFF1C49EF6E";
        let expect_result = "{\"public_key\":\"7bac671050fcba0dd54f3930601c42ad36cc11bc0589ed8d3cef3eff1c49ef6e\",\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"b93921db17f2f1d50bda37b510f543151df222e80946fefbacfadfb2d4a79fda4facf0ae5b41d71ea3a7ebea6aa88de9577a788aeab195b99b6a633c20e05503\"}";

        assert_eq!(
            expect_result,
            parse_sui_signature("sui-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_sui_signature_type_error() {
        let signature_cbor = "A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840B93921DB17F2F1D50BDA37B510F543151DF222E80946FEFBACFADFB2D4A79FDA4FACF0AE5B41D71EA3A7EBEA6AA88DE9577A788AEAB195B99B6A633C20E055030358207BAC671050FCBA0DD54F3930601C42AD36CC11BC0589ED8D3CEF3EFF1C49EF6E";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_sui_signature("eth-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_sui_signature_error() {
        let signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_sui_signature("sui-signature", signature_cbor)
        );
    }
}
