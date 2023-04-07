use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::solana::sol_sign_request::{SignType, SolSignRequest};

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateSolSignRequest
    fn generate_sol_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
        address: &str,
        origin: &str,
        sign_type: u32
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
        let sign_type = match SignType::from_u32(sign_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign_type is invalid"}).to_string(),
        };
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();
        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let address = if address.len() == 0 { None } else { Some(address.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = SolSignRequest::new(
            Some(request_id),
            sign_date_bytes,
            derivation_path,
            address,
            origin,
            sign_type
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "sol-sign-request";
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
    fn test_generate_sol_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "01000103c8d842a2f17fd7aab608ce2ea535a6e958dffa20caf669b347b911c4171965530f957620b228bae2b94c82ddd4c093983a67365555b737ec7ddc1117e61c72e0000000000000000000000000000000000000000000000000000000000000000010295cc2f1f39f3604718496ea00676d6a72ec66ad09d926e3ece34f565f18d201020200010c0200000000e1f50500000000";
        let path = "m/44'/501'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "solflare";
        let sign_type = 1;

        let expect_result = "{\"cbor\":\"a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02589601000103c8d842a2f17fd7aab608ce2ea535a6e958dffa20caf669b347b911c4171965530f957620b228bae2b94c82ddd4c093983a67365555b737ec7ddc1117e61c72e0000000000000000000000000000000000000000000000000000000000000000010295cc2f1f39f3604718496ea00676d6a72ec66ad09d926e3ece34f565f18d201020200010c0200000000e1f5050000000003d90130a20188182cf51901f5f500f500f5021a121212120568736f6c666c6172650601\",\"type\":\"sol-sign-request\"}";

        assert_eq!(expect_result, generate_sol_sign_request(
            request_id, sign_data, path, xfp, address, origin, sign_type
        ));
    }
    
    #[test]
    fn test_generate_sol_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "01000103c8d842a2f17fd7aab608ce2ea535a6e958dffa20caf669b347b911c4171965530f957620b228bae2b94c82ddd4c093983a67365555b737ec7ddc1117e61c72e0000000000000000000000000000000000000000000000000000000000000000010295cc2f1f39f3604718496ea00676d6a72ec66ad09d926e3ece34f565f18d201020200010c0200000000e1f50500000000";
        let path = "";
        let xfp = "1212120";
        let address = "";
        let origin = "solflare";
        let sign_type = 1;

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_sol_sign_request(
            request_id, sign_data, path, xfp, address, origin, sign_type
        ));
    }

    #[test]
    fn test_generate_sol_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "0123h";
        let path = "m/44'/501'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "solflare";
        let sign_type = 1;
        
        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(err_result, generate_sol_sign_request(
            request_id, sign_data, path, xfp, address, origin, sign_type
        ));
    }
}
