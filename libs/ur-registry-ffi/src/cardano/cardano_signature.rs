use anyhow::format_err;
use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::cardano::cardano_signature::CardanoSignature;
use ur_registry::registry_types::CARDANO_SIGNATURE;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseCardanoSignature
    fn parse_cardano_signature(ur_type: &str, cbor_hex: &str) -> String {
        if CARDANO_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let cardano_signature = CardanoSignature::try_from(cbor).map_err(|_| format_err!(""))?;
            let uuid = cardano_signature.get_request_id().ok_or(format_err!(""))?;
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let witness_set = hex::encode(cardano_signature.get_witness_set());
            Ok((request_id, witness_set))
        };
        match parse_signature() {
            Ok((request_id, witness_set)) => json!({
                "request_id": request_id,
                "witness_set": witness_set,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_cardano_signature() {
        let cardano_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258cda100828258207233f4cd5f24fa554e1ea4ed9251e39f4e18b2e0efd909b27ca01333c22ac49a5840725d8d98bab67eec8bf2704153f725f35ff7b0c9fabee135d97cf6c6b0885b14aa8748d9ba236abd19560b43afb0c5ac6d03359a1ef71b0712fc300d73e23e07825820c4af2472a9b27acad95967b1f5ff224cf3065824f6f1f0df7dbf4b52b819b1e85840c1ba75df625c7f657633f85f07d0bfd67f4e8ffb6b81b4b65a0ab186b459c4434971c25191b2725bff3f29bb9c1d247aabd60e63f0ea6ba53db0624ae1bcc101";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"witness_set\":\"a100828258207233f4cd5f24fa554e1ea4ed9251e39f4e18b2e0efd909b27ca01333c22ac49a5840725d8d98bab67eec8bf2704153f725f35ff7b0c9fabee135d97cf6c6b0885b14aa8748d9ba236abd19560b43afb0c5ac6d03359a1ef71b0712fc300d73e23e07825820c4af2472a9b27acad95967b1f5ff224cf3065824f6f1f0df7dbf4b52b819b1e85840c1ba75df625c7f657633f85f07d0bfd67f4e8ffb6b81b4b65a0ab186b459c4434971c25191b2725bff3f29bb9c1d247aabd60e63f0ea6ba53db0624ae1bcc101\"}";

        assert_eq!(
            expect_result,
            parse_cardano_signature("cardano-signature", cardano_signature_cbor)
        );
    }

    #[test]
    fn test_parse_cardano_signature_type_error() {
        let cardano_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258cda100828258207233f4cd5f24fa554e1ea4ed9251e39f4e18b2e0efd909b27ca01333c22ac49a5840725d8d98bab67eec8bf2704153f725f35ff7b0c9fabee135d97cf6c6b0885b14aa8748d9ba236abd19560b43afb0c5ac6d03359a1ef71b0712fc300d73e23e07825820c4af2472a9b27acad95967b1f5ff224cf3065824f6f1f0df7dbf4b52b819b1e85840c1ba75df625c7f657633f85f07d0bfd67f4e8ffb6b81b4b65a0ab186b459c4434971c25191b2725bff3f29bb9c1d247aabd60e63f0ea6ba53db0624ae1bcc101";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_cardano_signature("sol-signature", cardano_signature_cbor)
        );
    }

    #[test]
    fn test_parse_cardano_signature_error() {
        let cardano_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_cardano_signature("cardano-signature", cardano_signature_cbor)
        );
    }
}
