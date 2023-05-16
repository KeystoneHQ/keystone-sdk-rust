use hex;
use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::near::near_sign_request::NearSignRequest;
use ur_registry::traits::To;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateNearSignRequest
    fn generate_near_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
        account: &str,
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
        let sign_date_bytes = match serde_json::from_str::<Vec<String>>(sign_data) {
            Ok(v) => {
                if v.len() == 0 {
                    return json!({"error": "sign data is invalid"}).to_string()
                }
                let mut is_decode_error = false;
                let data_list = v.iter().map(|s| {
                    match hex::decode(s) {
                        Ok(bytes) => bytes,
                        Err(_) => {
                            is_decode_error = true;
                            vec![]
                        }
                    }
                }).collect();
                if is_decode_error {
                    return json!({"error": "sign data is invalid"}).to_string()
                }
                data_list
            },
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let account = if account.len() == 0 { None } else { Some(account.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = NearSignRequest::new(
            Some(request_id),
            sign_date_bytes,
            derivation_path,
            account,
            origin,
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "near-sign-request";
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
    fn test_generate_near_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"["4000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009FCC0720A016D3C1E849D86B16D7139E043EFC48ADD1C78F39C3D2F00EE98C07823E0CA1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037F0787E1CB1C22A1C63C24A37E4C6C656DD3CB049E6B7C17F75D01F0859EFB7D80100000003000000A1EDCCCE1BC2D3000000000000"]"#;
        let path = "m/44'/397'/0'";
        let xfp = "F23F9FD2";
        let account = "";
        let origin = "nearwallet";

        let expect_result = "{\"cbor\":\"a401d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d028158e64000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009fcc0720a016d3c1e849d86b16d7139e043efc48add1c78f39c3d2f00ee98c07823e0ca1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037f0787e1cb1c22a1c63c24a37e4c6c656dd3cb049e6b7c17f75d01f0859efb7d80100000003000000a1edccce1bc2d300000000000003d90130a20186182cf519018df500f5021af23f9fd2056a6e65617277616c6c6574\",\"type\":\"near-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_near_sign_request(request_id, sign_data, path, xfp, account, origin)
        );
    }

    #[test]
    fn test_generate_near_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"["4000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009FCC0720A016D3C1E849D86B16D7139E043EFC48ADD1C78F39C3D2F00EE98C07823E0CA1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037F0787E1CB1C22A1C63C24A37E4C6C656DD3CB049E6B7C17F75D01F0859EFB7D80100000003000000A1EDCCCE1BC2D3000000000000"]"#;
        let path = "m/44'/397'/0'";
        let xfp = "F23F9FD";
        let account = "";
        let origin = "nearwallet";

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(
            err_result_derivation_path,
            generate_near_sign_request(request_id, sign_data, path, xfp, account, origin)
        );
    }

    #[test]
    fn test_generate_near_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"[]"#;
        let path = "m/44'/397'/0'";
        let xfp = "F23F9FD2";
        let account = "";
        let origin = "nearwallet";

        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            err_result,
            generate_near_sign_request(request_id, sign_data, path, xfp, account, origin)
        );
    }
}
