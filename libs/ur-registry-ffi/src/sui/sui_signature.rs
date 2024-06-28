use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::registry_types::SUI_SIGNATURE;
use ur_registry::sui::sui_signature::SuiSignature;
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
        let signature_cbor = "A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840F4B79835417490958C72492723409289B444F3AF18274BA484A9EEACA9E760520E453776E5975DF058B537476932A45239685F694FC6362FE5AF6BA714DA6505035820AEB28ECACE5C664C080E71B9EFD3D071B3DAC119A26F4E830DD6BD06712ED93F";
        let expect_result = "{\"public_key\":\"aeb28ecace5c664c080e71b9efd3d071b3dac119a26f4e830dd6bd06712ed93f\",\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505\"}";

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
