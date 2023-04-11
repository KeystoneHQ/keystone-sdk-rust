use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::tron::tron_sign_request::TronSignRequest;

use crate::export;

use super::tron_transfer::raw_to_json;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronSignRequest
    fn generate_tron_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
        token_info: &str,
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
        let mut sign_data_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

        sign_data_bytes = match raw_to_json(sign_data_bytes, token_info) {
            Ok(v) => v,
            Err(err) => {
                return json!({"error": err}).to_string();
            }
        };

        let address = if address.len() == 0 { None } else { Some(address.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        // Print sign_data as json:
        println!("JSON: {}", String::from_utf8(sign_data_bytes.clone()).unwrap());

        let result = TronSignRequest::new(
            Some(request_id),
            sign_data_bytes,
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
    fn test_generate_tron_sign_request_trc20() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 7, 144, 34, 8, 225, 185, 222, 85, 150, 101, 198, 113, 64, 128, 196, 151,
            137, 187, 44, 90, 174, 1, 8, 31, 18, 169, 1, 10, 49, 116, 121, 112, 101, 46, 103,
            111, 111, 103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111,
            116, 111, 99, 111, 108, 46, 84, 114, 105, 103, 103, 101, 114, 83, 109, 97, 114,
            116, 67, 111, 110, 116, 114, 97, 99, 116, 18, 116, 10, 21, 65, 199, 159, 4, 94, 77,
            72, 173, 141, 174, 0, 230, 166, 113, 77, 174, 30, 0, 10, 223, 205, 18, 21, 65, 13,
            41, 44, 152, 165, 236, 160, 108, 32, 133, 255, 249, 147, 153, 100, 35, 207, 102,
            201, 59, 34, 68, 169, 5, 156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155, 188,
            229, 32, 217, 132, 195, 185, 90, 209, 12, 180, 227, 42, 146, 148, 230, 51, 141,
            163, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 15, 66, 64, 112, 192, 182, 224, 135, 187, 44, 144, 1, 128, 148, 235, 220,
            3
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "name": "TONE",
            "symbol": "TronOne",
            "decimals": 8
        }"#;

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901907b22636f6e747261637441646472657373223a225442416f37504e794b6f393459575571314373324c4246786b685470686e41453454222c22666565223a313030303030303030302c2266726f6d223a22545541687877334d674d795239726879724d446e564a626f33626b79314753557248222c226c6174657374426c6f636b223a7b2268617368223a2230303030303030303030303030303030653162396465353539363635633637313030303030303030303030303030303030303030303030303030303030303030222c226e756d626572223a313933362c2274696d657374616d70223a313532373638323434303030307d2c226f76657272696465223a7b22646563696d616c73223a382c22746f6b656e5f66756c6c5f6e616d65223a22544f4e45222c22746f6b656e5f73686f72745f6e616d65223a2254726f6e4f6e65227d2c22746f223a2254514167325432764a634841583973624b54456f616f577a7435313279556a694644222c22746f6b656e223a6e756c6c2c2276616c7565223a2231303030303030227d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_trc10() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 46, 30, 34, 8, 84, 62, 84, 106, 67, 173, 78, 95, 64, 248, 187, 213, 157,
            248, 45, 90, 115, 8, 2, 18, 111, 10, 50, 116, 121, 112, 101, 46, 103, 111, 111,
            103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111,
            99, 111, 108, 46, 84, 114, 97, 110, 115, 102, 101, 114, 65, 115, 115, 101, 116, 67,
            111, 110, 116, 114, 97, 99, 116, 18, 57, 10, 7, 49, 48, 48, 49, 48, 57, 48, 18, 21,
            65, 238, 109, 31, 251, 168, 114, 87, 57, 113, 86, 42, 112, 249, 173, 29, 194, 212,
            175, 140, 139, 26, 21, 65, 101, 78, 180, 64, 193, 160, 100, 10, 202, 51, 122, 217,
            235, 243, 161, 34, 151, 106, 145, 5, 32, 1
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901277b22636f6e747261637441646472657373223a6e756c6c2c22666565223a302c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c226c6174657374426c6f636b223a7b2268617368223a2230303030303030303030303030303030353433653534366134336164346535663030303030303030303030303030303030303030303030303030303030303030222c226e756d626572223a31313830362c2274696d657374616d70223a313537383435393639393030307d2c226f76657272696465223a6e756c6c2c22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c22746f6b656e223a2231303031303930222c2276616c7565223a2231227d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
        ));
    }
    
    #[test]
    fn test_generate_tron_sign_request_tx() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 102, 92, 34, 8, 236, 39, 182, 57, 84, 245, 145, 61, 64, 216, 135, 189, 212,
            247, 45, 90, 103, 8, 1, 18, 99, 10, 45, 116, 121, 112, 101, 46, 103, 111, 111, 103,
            108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99,
            111, 108, 46, 84, 114, 97, 110, 115, 102, 101, 114, 67, 111, 110, 116, 114, 97, 99,
            116, 18, 50, 10, 21, 65, 194, 172, 29, 42, 41, 234, 39, 185, 187, 240, 73, 55, 12,
            53, 5, 19, 156, 124, 157, 144, 18, 21, 65, 238, 109, 31, 251, 168, 114, 87, 57,
            113, 86, 42, 112, 249, 173, 29, 194, 212, 175, 140, 139, 24, 128, 137, 122
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901297b22636f6e747261637441646472657373223a6e756c6c2c22666565223a302c2266726f6d223a225454695947786237594e6655514a416e4c464d643170766d6f79774d375078694732222c226c6174657374426c6f636b223a7b2268617368223a2230303030303030303030303030303030656332376236333935346635393133643030303030303030303030303030303030303030303030303030303030303030222c226e756d626572223a32363230342c2274696d657374616d70223a313537383330363230373030307d2c226f76657272696465223a6e756c6c2c22746f223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c22746f6b656e223a22545258222c2276616c7565223a2232303030303030227d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
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
        let token_info = "";

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_tron_sign_request(
            request_id, sign_data, path, xfp, token_info, address, origin
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
        let token_info = "";

        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(err_result, generate_tron_sign_request(
            request_id, sign_data, path, xfp, token_info, address, origin
        ));
    }
}
