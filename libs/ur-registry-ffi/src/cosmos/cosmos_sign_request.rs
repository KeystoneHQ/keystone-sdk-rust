use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::cosmos::cosmos_sign_request::{DataType, CosmosSignRequest};

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateCosmosSignRequest
    fn generate_cosmos_sign_request(
        request_id: &str,
        sign_data: &str,
        data_type: u32,
        derivation_paths: &str,
        addresses: &str,
        origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();

        let mut is_derivation_paths_err = false;
        let derivation_paths: Vec<CryptoKeyPath> = match serde_json::from_str::<Vec<String>>(derivation_paths) {
            Ok(v) => v,
            Err(_) => return json!({"error": "derivation_paths is invalid"}).to_string(),
        }.iter().map(|path| {
            match CryptoKeyPath::from_path(path.to_string(), None) {
                Ok(v) => Some(v),
                Err(_) => {
                    is_derivation_paths_err = true;
                    None
                },
            }
        }).filter_map(|x| x).collect();
        if is_derivation_paths_err || derivation_paths.len() == 0 {
            return json!({"error": "derivation_paths is invalid"}).to_string();
        }

        let data_type = match DataType::from_u32(data_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data type is invalid"}).to_string(),
        };
        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign_data is invalid"}).to_string(),
        };

        let addresses = match serde_json::from_str::<Vec<String>>(addresses) {
            Ok(v) => Some(v),
            Err(_) => None,
        };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let cbor_bytes = match CosmosSignRequest::new(
            request_id,
            sign_date_bytes,
            data_type,
            derivation_paths,
            addresses,
            origin
        ).to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign_data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = "cosmos-sign-request";
        let ur = json!({
            "type": ur_type,
            "cbor": cbor_hex,
        });
        ur.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cosmos_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "f849808609184e72a00082271094000000000000000000000000000000000000000080a47f7465737432000000000000000000000000000000000000000000000000000000600057808080";
        let data_type = 1;
        let paths = "[\"m/44'/118'/1'/0/0\"]";
        let addresses = "";
        let origin = "keplr";

        let expect_result = "{\"cbor\":\"a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584bf849808609184e72a00082271094000000000000000000000000000000000000000080a47f746573743200000000000000000000000000000000000000000000000000000060005780808003010482d90130a1018a182cf51876f501f500f400f4d90130a1018a182cf51876f501f500f401f406656b65706c72\",\"type\":\"cosmos-sign-request\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, paths, addresses, origin
        ));
    }
    
    #[test]
    fn test_generate_cosmos_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "f849808609184e72a00082271094000000000000000000000000000000000000000080a47f7465737432000000000000000000000000000000000000000000000000000000600057808080";
        let data_type = 1;
        let paths = "[]";
        let addresses = "[]";
        let origin = "keplr";

        let expect_result = "{\"error\":\"derivation_paths is invalid\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, paths, addresses, origin
        ));
    }

    #[test]
    fn test_generate_eth_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "1234567";
        let data_type = 1;
        let paths = "[\"m/44'/1'/1'/0/1\"]";
        let addresses = "[]";
        let origin = "keplr";

        let expect_result = "{\"error\":\"sign_data is invalid\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, paths, addresses, origin
        ));
    }
}
