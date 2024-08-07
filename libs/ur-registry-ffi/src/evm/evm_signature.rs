use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::{cosmos::evm_signature::EvmSignature, registry_types::EVM_SIGNATURE};
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseEvmSignature
    fn parse_evm_signature(ur_type: &str, cbor_hex: &str) -> String {
        if EVM_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let sig = EvmSignature::try_from(cbor)?;
            let uuid = sig.get_request_id();
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(sig.get_signature());
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
    fn test_parse_evm_signature() {
        let signature_cbor = "a201d82550057523355d514a64a481ff2200000000025840a0e2577ca16119a32f421c6a1c90fa2178a9382f30bf3575ff276fb820b32b3269d49d6bbfc82bae899f60c15de4b97f24a7ebb6d4712534829628ccfbef38bc";
        let expect_result = "{\"request_id\":\"05752335-5d51-4a64-a481-ff2200000000\",\"signature\":\"a0e2577ca16119a32f421c6a1c90fa2178a9382f30bf3575ff276fb820b32b3269d49d6bbfc82bae899f60c15de4b97f24a7ebb6d4712534829628ccfbef38bc\"}";

        assert_eq!(
            expect_result,
            parse_evm_signature("evm-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_evm_signature_type_error() {
        let signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_evm_signature("eth-signature", signature_cbor)
        );
    }

    #[test]
    fn test_parse_evm_signature_error() {
        let signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_evm_signature("evm-signature", signature_cbor)
        );
    }
}
