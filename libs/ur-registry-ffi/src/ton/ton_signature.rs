use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::registry_types::TON_SIGNATURE;
use ur_registry::ton::ton_signature::TonSignature;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseTonSignature
    fn parse_ton_signature(ur_type: &str, cbor_hex: &str) -> String {
        if TON_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let sig = TonSignature::try_from(cbor)?;
            let mut request_id = String::from("");
            if let Some(uuid) = sig.get_request_id() {
                let uuid_hex = hex::encode(uuid);
                request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            }
            let signature = hex::encode(sig.get_signature());
            let mut origin = String::from("");
            if let Some(ori) = sig.get_origin() {
                origin = ori;
            }
            Ok((request_id, signature, origin))
        };
        match parse_signature() {
            Ok((request_id, signature, origin)) => json!({
                "request_id": request_id,
                "signature": signature,
                "origin": origin,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_ton_signature() {
        let signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da650503684b657973746f6e65";
        let expect_result = "{\"origin\":\"Keystone\",\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505\"}";

        assert_eq!(
            expect_result,
            parse_ton_signature("ton-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_ton_signature_type_error() {
        let signature_cbor = "A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840B93921DB17F2F1D50BDA37B510F543151DF222E80946FEFBACFADFB2D4A79FDA4FACF0AE5B41D71EA3A7EBEA6AA88DE9577A788AEAB195B99B6A633C20E055030358207BAC671050FCBA0DD54F3930601C42AD36CC11BC0589ED8D3CEF3EFF1C49EF6E";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_ton_signature("eth-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_ton_signature_error() {
        let signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_ton_signature("ton-signature", signature_cbor)
        );
    }
}
