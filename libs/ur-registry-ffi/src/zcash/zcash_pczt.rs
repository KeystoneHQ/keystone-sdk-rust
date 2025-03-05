use crate::{export, util_internal::string_helper::remove_prefix_0x};
use anyhow::{format_err, Error};
use serde_json::json;
use ur_registry::{registry_types::ZCASH_PCZT, zcash::zcash_pczt::ZcashPczt};

// This module provides FFI (Foreign Function Interface) functions for handling Zcash PCZT
// data structures. It allows for conversion between raw binary data and CBOR-encoded Uniform Resources (URs).
//
// The module exports two main functions:
// 1. generate_zcash_pczt: Converts hex-encoded PCZT data into a UR-encoded format
// 2. parse_zcash_pczt: Parses a UR-encoded PCZT back into its hex representation

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateZcashPczt
    fn generate_zcash_pczt(
        data: &str
    ) -> String {
        let data = match data {
            "" => return json!({"error": "data is required"}).to_string(),
            _x => _x.to_string()
        };

        let bytes = match hex::decode(data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data is invalid"}).to_string(),
        };

        let cbor_bytes: Vec<u8> = match ZcashPczt::new(
            bytes,
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = ZCASH_PCZT.get_type();
        let ur = json!({
            "type": ur_type,
            "cbor": cbor_hex,
        });
        ur.to_string()
    }

    @Java_com_keystone_sdk_KeystoneNativeSDK_parseZcashPczt
    fn parse_zcash_pczt(ur_type: &str, cbor_hex: &str) -> String {
        if ZCASH_PCZT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse = || -> Result<String, Error> {
            let cbor = hex::decode(remove_prefix_0x(cbor_hex).to_string())?;
            let pczt = ZcashPczt::try_from(cbor).map_err(|_| format_err!(""))?;
            let pczt_hex = hex::encode(pczt.get_data());
            Ok(pczt_hex)
        };
        match parse() {
            Ok(v) => json!({
                "pczt": v,
            }).to_string(),
            Err(_) => json!({"error": "PCZT is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use minicbor;
    use ur_registry::zcash::zcash_pczt::ZcashPczt as UrZcashPczt;

    #[test]
    fn test_generate_zcash_pczt() {
        // Test with valid data
        let data = "d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1";
        let result = generate_zcash_pczt(data);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(json_result.get("error").is_none());
        assert_eq!(json_result["type"], "zcash-pczt");
        assert!(json_result["cbor"].is_string());
        
        // Test with empty data
        let result = generate_zcash_pczt("");
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json_result["error"], "data is required");
        
        // Test with invalid hex data
        let result = generate_zcash_pczt("invalid_hex");
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        assert_eq!(json_result["error"], "data is invalid");
    }
    
    #[test]
    fn test_parse_zcash_pczt() {
        // Create a valid PCZT
        let data = vec![0xd1; 16];
        let ur_pczt = UrZcashPczt::new(data.clone());
        let cbor = minicbor::to_vec(&ur_pczt).unwrap();
        let cbor_hex = hex::encode(&cbor);
        
        // Test with valid data
        let result = parse_zcash_pczt(&ZCASH_PCZT.get_type(), &cbor_hex);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(json_result.get("error").is_none());
        assert_eq!(json_result["pczt"], hex::encode(&data));
        
        // Test with 0x prefix
        let result = parse_zcash_pczt(&ZCASH_PCZT.get_type(), &format!("0x{}", cbor_hex));
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(json_result.get("error").is_none());
        assert_eq!(json_result["pczt"], hex::encode(&data));
        
        // Test with type mismatch
        let result = parse_zcash_pczt("wrong-type", &cbor_hex);
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "type not match");
        
        // Test with invalid CBOR
        let result = parse_zcash_pczt(&ZCASH_PCZT.get_type(), "invalid");
        let json_result: serde_json::Value = serde_json::from_str(&result).unwrap();
        
        assert!(json_result.get("error").is_some());
        assert_eq!(json_result["error"], "PCZT is invalid");
    }
    
    #[test]
    fn test_roundtrip() {
        // Generate a PCZT
        let data = "d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1d1";
        let generate_result = generate_zcash_pczt(data);
        let json_generate: serde_json::Value = serde_json::from_str(&generate_result).unwrap();
        
        // Parse the generated PCZT
        let ur_type = json_generate["type"].as_str().unwrap();
        let cbor_hex = json_generate["cbor"].as_str().unwrap();
        let parse_result = parse_zcash_pczt(ur_type, cbor_hex);
        let json_parse: serde_json::Value = serde_json::from_str(&parse_result).unwrap();
        
        // Verify the roundtrip
        assert_eq!(json_parse["pczt"], data);
    }
}
