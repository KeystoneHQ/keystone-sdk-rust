use crate::{export, util_internal::string_helper::remove_prefix_0x};
use anyhow::{format_err, Error};
use serde_json::json;
use ur_registry::{registry_types::KASPA_PSKT, kaspa::kaspa_pskt::KaspaPskt};
use core::convert::TryInto;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateKaspaPskt
    fn generate_kaspa_pskt(
        data: &str
    ) -> String {
        if data.is_empty() {
            return json!({"error": "data is required"}).to_string();
        }

        let bytes = match hex::decode(remove_prefix_0x(data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data is invalid hex"}).to_string(),
        };

        let cbor_bytes: Vec<u8> = match KaspaPskt::new(bytes).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "CBOR encode failed"}).to_string(),
        };

        let cbor_hex = hex::encode(cbor_bytes);
        json!({
            "type": KASPA_PSKT.get_type(),
            "cbor": cbor_hex,
        }).to_string()
    }

    @Java_com_keystone_sdk_KeystoneNativeSDK_parseKaspaPskt
    fn parse_kaspa_pskt(ur_type: &str, cbor_hex: &str) -> String {
        if KASPA_PSKT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse = || -> Result<String, Error> {
            let cbor = hex::decode(remove_prefix_0x(cbor_hex).to_string())?;
            let pskt = KaspaPskt::try_from(cbor).map_err(|_| format_err!("decode failed"))?;
            let pskt_hex = hex::encode(pskt.get_pskt());
            Ok(pskt_hex)
        };

        match parse() {
            Ok(v) => json!({ "pskt": v }).to_string(),
            Err(_) => json!({"error": "PSKT is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_kaspa_pskt() {
        let data = "0102030405";
        let result = generate_kaspa_pskt(data);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert_eq!(json_result["type"], "kaspa-pskt");
        assert!(json_result["cbor"].as_str().unwrap().starts_with("a1"));
        
        assert!(generate_kaspa_pskt("").contains("data is required"));
    }

    #[test]
    fn test_parse_kaspa_pskt() {
        let data = "0102030405";
        let gen_res = generate_kaspa_pskt(data);
        let json_gen: serde_json::Value = serde_json::from_str(&gen_res).unwrap();
        let cbor_hex = json_gen["cbor"].as_str().unwrap();

        let parse_res = parse_kaspa_pskt("kaspa-pskt", cbor_hex);
        let json_parse: serde_json::Value = serde_json::from_str(&parse_res).unwrap();
        assert_eq!(json_parse["pskt"], data);
    }

    #[test]
    fn test_kaspa_pskt_error_cases() {
        assert!(generate_kaspa_pskt("0102030G").contains("invalid hex"));

        assert!(parse_kaspa_pskt("wrong-type", "a101...").contains("type not match"));

        assert!(parse_kaspa_pskt("kaspa-pskt", "ffff").contains("invalid"));
    }
}