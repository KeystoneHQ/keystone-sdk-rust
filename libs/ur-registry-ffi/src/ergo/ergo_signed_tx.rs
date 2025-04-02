use anyhow::{format_err, Error};
use serde_json::json;
use hex;
use uuid::Uuid;
use ur_registry::ergo::ergo_signed_tx::ErgoSignedTx;
use ur_registry::registry_types::ERGO_SIGNED_TX;
use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseErgoSignedTx
    fn parse_ergo_signed_tx(ur_type: &str, cbor_hex: &str) -> String {
        if ERGO_SIGNED_TX.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signed_tx = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let ergo_signed_tx = ErgoSignedTx::try_from(cbor).map_err(|_| format_err!(""))?;
            let uuid = ergo_signed_tx.get_request_id();
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signed_transaction = hex::encode(ergo_signed_tx.get_signed_tx());
            Ok((request_id, signed_transaction))
        };
        match parse_signed_tx() {
            Ok((request_id, signed_transaction)) => json!({
                "request_id": request_id,
                "signed_tx": signed_transaction,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ergo_signed_tx() {
        let signed_tx_cbor = "A201D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D0259014C011A9F15BFAC9379C882FE0B7ECB2288153CE4F2DEF4F272214FB80F8E2630F04C38A3F024D30E683EE9BD4E2B65DE9EE3C4B29F051D65A7DC0D670E75A962326CF9DAA0D8BF32B067ED3E426B9BC29A3FF9C937F96E02FB9CE1000001FBBAAC7337D051C10FC3DA0CCB864F4D32D40027551E1C3EA3CE361F39B91E4003C0843D0008CD02DC5B9D9D2081889EF00E6452FB5AD1730DF42444CECCB9EA02258256D2FBD262E4F25601006400C0843D1005040004000E36100204A00B08CD0279BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798EA02D192A39A8CC7A701730073011001020402D19683030193A38CC7B2A57300000193C2B2A57301007473027303830108CDEEAC93B1A57304E4F2560000809BEE020008CD0388FA54338147371023AACB846C96C57E72CDCD73BC85D20250467E5B79DFA2AAE4F25601006400";
        let expected_result =  "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signed_tx\":\"011a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c38a3f024d30e683ee9bd4e2b65de9ee3c4b29f051d65a7dc0d670e75a962326cf9daa0d8bf32b067ed3e426b9bc29a3ff9c937f96e02fb9ce1000001fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e4003c0843d0008cd02dc5b9d9d2081889ef00e6452fb5ad1730df42444ceccb9ea02258256d2fbd262e4f25601006400c0843d1005040004000e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a701730073011001020402d19683030193a38cc7b2a57300000193c2b2a57301007473027303830108cdeeac93b1a57304e4f2560000809bee020008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aae4f25601006400\"}";

        assert_eq!(
            expected_result,
            parse_ergo_signed_tx("ergo-signed-tx", signed_tx_cbor)
        );
    }

    #[test]
    fn test_parse_ergo_signed_tx_type_error() {
        let signed_tx_cbor = "A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840B93921DB17F2F1D50BDA37B510F543151DF222E80946FEFBACFADFB2D4A79FDA4FACF0AE5B41D71EA3A7EBEA6AA88DE9577A788AEAB195B99B6A633C20E055030358207BAC671050FCBA0DD54F3930601C42AD36CC11BC0589ED8D3CEF3EFF1C49EF6E";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_ergo_signed_tx("eth-signature", signed_tx_cbor)
        );
    }


    #[test]
    fn test_parse_ergo_signed_tx_error() {
        let signed_tx_cbor = "a201";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_ergo_signed_tx("ergo-signature", signed_tx_cbor)
        );
    }
}

