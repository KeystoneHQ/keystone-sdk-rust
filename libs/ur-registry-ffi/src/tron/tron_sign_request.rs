use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::keystone::keystone_sign_request::KeystoneSignRequest;
use ur_registry::pb::protobuf_parser::{serialize_protobuf, zip};
use ur_registry::pb::protoc::payload::Type::SignTx;
use ur_registry::pb::protoc::sign_transaction::Transaction;
use ur_registry::pb::protoc::{payload, Base, Payload, SignTransaction};
use ur_registry::traits::To;
use uuid::Uuid;

use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

use super::tron_transfer::raw_to_tx;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronSignRequest
    fn generate_tron_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
        token_info: &str,
        origin: &str,
        timestamp: i64
    ) -> String {
        let xfp_bytes = match hex::decode(xfp) {
            Ok(v) => v,
            Err(_) => return json!({"error": "xfp is invalid"}).to_string(),
        };
        let xfp_slice: [u8; 4] = match xfp_bytes.as_slice().try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "length of xfp must be exactly 8"}).to_string(),
        };
        match CryptoKeyPath::from_path(path.to_string(), Some(xfp_slice)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "path is invalid"}).to_string(),
        };
        match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        };
        let sign_data_bytes = match hex::decode(remove_prefix_0x(sign_data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let tx = match raw_to_tx(sign_data_bytes, token_info) {
            Ok(v) => v,
            Err(err) => {
                return json!({"error": err}).to_string();
            }
        };

        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let coin_code = "TRON".to_string();

        let base = Base {
            version: 2,
            description: "QrCode Protocol".to_string(),
            content: None,
            device_type: "".to_string(),
            data: Some(
                Payload {
                    r#type: SignTx as i32,
                    xfp: hex::encode(xfp_bytes).to_uppercase(),
                    content: Some(payload::Content::SignTx(
                        SignTransaction {
                            coin_code,
                            sign_id: request_id.to_string(),
                            hd_path: path.to_string(),
                            timestamp,
                            decimal: 6,
                            transaction: Some(Transaction::TronTx(tx)),
                        }
                    )),
                }
            ),
        };

        let sign_data_bytes = serialize_protobuf(base);
        let ziped_sign_data_bytes = match zip(&sign_data_bytes) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

        let result = KeystoneSignRequest::new(
            ziped_sign_data_bytes,
            origin
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "keystone-sign-request";
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
            10, 2, 7, 144, 34, 8, 225, 185, 222, 85, 150, 101, 198, 113, 64, 128, 196, 151, 137,
            187, 44, 90, 174, 1, 8, 31, 18, 169, 1, 10, 49, 116, 121, 112, 101, 46, 103, 111, 111,
            103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99,
            111, 108, 46, 84, 114, 105, 103, 103, 101, 114, 83, 109, 97, 114, 116, 67, 111, 110,
            116, 114, 97, 99, 116, 18, 116, 10, 21, 65, 141, 254, 193, 205, 225, 254, 106, 158,
            195, 138, 22, 199, 214, 112, 115, 227, 2, 8, 81, 192, 18, 21, 65, 166, 20, 248, 3, 182,
            253, 120, 9, 134, 164, 44, 120, 236, 156, 127, 119, 230, 222, 209, 60, 34, 68, 169, 5,
            156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 156, 2, 121, 241, 189, 169, 252, 64, 168,
            95, 27, 83, 195, 6, 96, 40, 100, 83, 62, 115, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 152, 150, 128, 112, 192, 182, 224, 135,
            187, 44, 144, 1, 128, 148, 235, 220, 3,
        ]);
        let path = "m/44'/195'/0'/0/0";
        let xfp = "F23F9FD2";
        let origin = "";
        let token_info = r#"{
            "name": "TRON_USDT",
            "symbol": "USDT",
            "decimals": 6
        }"#;
        let timestamp = 1681871353647;

        let expect_result = "{\"cbor\":\"a1015901381f8b08000000000000037590bd4ac3501cc5b19452bb54e3523a9520d4c190efa4571cec77b03136ed35922e929b7f880635368da6f6055c7d0c379d2ae8e28b087d02773753c4c1a13f0e9cc3e14c279fa18a66d40cc1abf4a3300eddf0b2fc9a49db7c47103ba8d312e8a74c218b07c706b58d080f1e918011890a8c441c6010016004c2814a44708902e5cd2b5692aa2c8fe42acba562b9caf3d7e2ed9bdbc9353ed7281a0f54431b075ecf9c7671f3a236ae8d6ce946af85f170e6077aa0c4651a1b773c4e0e5d211dfbedeb9e3f3a3abff5da8966d9a4ddabcfeeeb348dcda6da174fb52872acbadd25e05853259c887a6204316af81310f23cf7cb9e5e3848ed1f1e4f1078b28c14457615f56fba928dc762e963be7878df6d6c15b227c316a6d697b79c2d6329a7bdccf77f00edd27a504d010000\",\"type\":\"keystone-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_tron_sign_request(
                request_id,
                &sign_data_hex,
                path,
                xfp,
                token_info,
                origin,
                timestamp
            )
        );
    }

    #[test]
    fn test_generate_tron_sign_request_trc10() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 232, 21, 34, 8, 172, 58, 148, 70, 62, 191, 149, 244, 64, 176, 176, 248, 129,
            255, 48, 90, 117, 8, 2, 18, 113, 10, 50, 116, 121, 112, 101, 46, 103, 111, 111, 103,
            108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99, 111,
            108, 46, 84, 114, 97, 110, 115, 102, 101, 114, 65, 115, 115, 101, 116, 67, 111, 110,
            116, 114, 97, 99, 116, 18, 59, 10, 7, 49, 48, 48, 50, 48, 48, 48, 18, 21, 65, 141, 254,
            193, 205, 225, 254, 106, 158, 195, 138, 22, 199, 214, 112, 115, 227, 2, 8, 81, 192, 26,
            21, 65, 156, 2, 121, 241, 189, 169, 252, 64, 168, 95, 27, 83, 195, 6, 96, 40, 100, 83,
            62, 115, 32, 128, 137, 122,
        ]);
        let path = "m/44'/195'/0'/0/0";
        let xfp = "F23F9FD2";
        let origin = "";
        let token_info = r#"{
            "name": "TRON_BTT",
            "symbol": "BTT",
            "decimals": 6
        }"#;
        let timestamp = 1681871353647;

        let expect_result = "{\"cbor\":\"a1015901231f8b08000000000000037d8f314bc3400085b1953664d1662a994a10ea1272b9bb269c3898d4c4606d6ccb11a98be4726944d4405a8d3ab93bbb38fa07747270d27fe04ff01788ab935e47977e3c788f373c785245591916dd9ca7ad4191cff2243f551f2aa2957c887ce26f43edae222fd3d17ea8ac1166f29461ae2366731db398eb8471ae4306b8cd104f98c5d5c6998171db3049a76d002103b49ebe3e5f7fc07acd7d5b92eb26001000a06a34bc3469b99b403ab233efbc971df68f2f52af0ca231f37acecdb5a36974d8b507e820288a3872c63b8cc7d195954fd15e199ecc889b4d39accfd7041b7d794bd83fe204c504630ba56c423a132caa85acde7f549bdf8fefb7bfc06dc8559752459a5f3f12a9590b9e5f36ff0090902b4e30010000\",\"type\":\"keystone-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_tron_sign_request(
                request_id,
                &sign_data_hex,
                path,
                xfp,
                token_info,
                origin,
                timestamp
            )
        );
    }

    #[test]
    fn test_generate_tron_sign_request_tx() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 102, 92, 34, 8, 236, 39, 182, 57, 84, 245, 145, 61, 64, 216, 135, 189, 212, 247,
            45, 90, 103, 8, 1, 18, 99, 10, 45, 116, 121, 112, 101, 46, 103, 111, 111, 103, 108,
            101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99, 111, 108,
            46, 84, 114, 97, 110, 115, 102, 101, 114, 67, 111, 110, 116, 114, 97, 99, 116, 18, 50,
            10, 21, 65, 141, 254, 193, 205, 225, 254, 106, 158, 195, 138, 22, 199, 214, 112, 115,
            227, 2, 8, 81, 192, 18, 21, 65, 156, 2, 121, 241, 189, 169, 252, 64, 168, 95, 27, 83,
            195, 6, 96, 40, 100, 83, 62, 115, 24, 237, 162, 1,
        ]);
        let path = "m/44'/195'/0'/0/0";
        let xfp = "F23F9FD2";
        let origin = "";
        let token_info = "";
        let timestamp = 1681871353647;

        let expect_result = "{\"cbor\":\"a10159010f1f8b08000000000000037dc8bf4e83401cc0f150ffa461513b19a68698d48570dc1de01907690589b5d8920b5a378e1f628c4a42aba89b83efe01b38eae4e033b8b93af8040e2e46a73277e837dfe953af359606452787b4d92ff2719ee4e7ca7dadd2ba8789c7bc1dacfe48f23c0f0f82c61a1306a4828246840d1a1531684c00685820b005814458a0ac5ce894b67483992d1d55eba8f9fcfdf5f687d617db4f923cc7c32345e5c1b5c1cbbd04f3d0cedccb6e76dc3bbd4addd28f86c2ed3a77b78eaaf241c7ee9343bf28e2c819ee0a88a31b2b1f91fd32381bb3763602bc80d10635377bf2369a2a4db02d2cc24c7a6232834045335bfe7c97561fff1f3e7e35ffe5756b02470643de17010000\",\"type\":\"keystone-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_tron_sign_request(
                request_id,
                &sign_data_hex,
                path,
                xfp,
                token_info,
                origin,
                timestamp
            )
        );
    }

    #[test]
    fn test_generate_tron_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#;
        let path = "";
        let xfp = "1212120";
        let origin = "";
        let token_info = "";
        let timestamp = 1681871353647;

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(
            err_result_derivation_path,
            generate_tron_sign_request(
                request_id, sign_data, path, xfp, token_info, origin, timestamp
            )
        );
    }

    #[test]
    fn test_generate_tron_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h"}"#;
        let path = "m/44'/501'/0'/0'";
        let xfp = "12121212";
        let origin = "";
        let token_info = "";
        let timestamp = 1681871353647;

        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            err_result,
            generate_tron_sign_request(
                request_id, sign_data, path, xfp, token_info, origin, timestamp
            )
        );
    }
}
