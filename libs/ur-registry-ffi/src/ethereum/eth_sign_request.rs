use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::ethereum::eth_sign_request::{EthSignRequest, DataType};

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateETHSignRequest
    fn generate_eth_sign_request(
        request_id: &str,
        sign_data: &str,
        data_type: u32,
        chain_id: i32,
        path: &str,
        xfp: &str,
        address: &str,
        origin: &str
    ) -> String {
        let xfp_bytes = match hex::decode(xfp) {
            Ok(v) => v,
            Err(_) => return json!({"error": "xfp is invalid"}).to_string(),
        };
        let xfp_slice: [u8; 4] = match xfp_bytes.as_slice().try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "length of xfp must be exactly 8"}).to_string(),
        };
        let derivation_path = match CryptoKeyPath::from_path(path.to_string(), Some(xfp_slice)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "path is invalid"}).to_string(),
        };
        let data_type = match DataType::from_u32(data_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data type is invalid"}).to_string(),
        };
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();
        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign_data is invalid"}).to_string(),
        };

        let chain_id = if chain_id != 0 { Some(i128::from(chain_id)) } else { None };
        let address = if address.len() == 0 { None } else { Some(address.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = EthSignRequest::new(
            Some(request_id),
            sign_date_bytes,
            data_type,
            chain_id,
            derivation_path,
            address,
            origin
        );
        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "eth-sign-request";
        let ur = json!({
            "type": ur_type,
            "cbor": cbor,
        });
        ur.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_eth_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "f849808609184e72a00082271094000000000000000000000000000000000000000080a47f7465737432000000000000000000000000000000000000000000000000000000600057808080";
        let path = "m/44'/1'/1'/0/1";
        let xfp = "12345678";
        let chain_id: i32 = 1;
        let address = "";
        let origin = "metamask";
        let data_type = 1;

        let expect_result = "{\"cbor\":\"a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584bf849808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000578080800301040105d90130a2018a182cf501f501f500f401f4021a1234567807686d6574616d61736b\",\"type\":\"eth-sign-request\"}";

        assert_eq!(expect_result, generate_eth_sign_request(
            request_id, sign_data, data_type, chain_id, path, xfp, address, origin
        ));
    }
    
    #[test]
    fn test_generate_eth_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "f849808609184e72a00082271094000000000000000000000000000000000000000080a47f7465737432000000000000000000000000000000000000000000000000000000600057808080";
        let path = "";
        let xfp = "12345678";
        let chain_id: i32 = 1;
        let address = "";
        let origin = "metamask";
        let data_type = 1;

        let expect_result = "{\"error\":\"path is invalid\"}";

        assert_eq!(expect_result, generate_eth_sign_request(
            request_id, sign_data, data_type, chain_id, path, xfp, address, origin
        ));
    }

    #[test]
    fn test_generate_eth_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "f8498086091";
        let path = "m/44'/1'/1'/0/1";
        let xfp = "12345678";
        let chain_id: i32 = 1;
        let address = "";
        let origin = "metamask";
        let data_type = 1;

        let expect_result = "{\"error\":\"sign_data is invalid\"}";

        assert_eq!(expect_result, generate_eth_sign_request(
            request_id, sign_data, data_type, chain_id, path, xfp, address, origin
        ));
    }
}
