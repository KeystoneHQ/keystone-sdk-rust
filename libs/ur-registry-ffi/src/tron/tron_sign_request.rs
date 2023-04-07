use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::tron::tron_sign_request::TronSignRequest;

use crate::export;
use crate::tron::tron_transfer::TronTransfer;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronSignRequest
    fn generate_tron_sign_request(
        request_id: &str,
        sign_data: &str,
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

        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();
        let sign_date_bytes = match serde_json::from_str::<TronTransfer>(sign_data) {
            Ok(_) => sign_data.as_bytes().to_vec(),
            Err(err) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let address = if address.len() == 0 { None } else { Some(address.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = TronSignRequest::new(
            Some(request_id),
            sign_date_bytes,
            derivation_path,
            address,
            origin
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "tron-sign-request";
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
    fn test_generate_tron_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#;
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, sign_data, path, xfp, address, origin
        ));
    }
    
    #[test]
    fn test_generate_tron_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#;
        let path = "";
        let xfp = "1212120";
        let address = "";
        let origin = "";

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_tron_sign_request(
            request_id, sign_data, path, xfp, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h"}"#;
        let path = "m/44'/501'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        
        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(err_result, generate_tron_sign_request(
            request_id, sign_data, path, xfp, address, origin
        ));
    }
}
