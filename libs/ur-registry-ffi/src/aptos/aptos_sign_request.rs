use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::aptos::aptos_sign_request::{SignType, AptosSignRequest};

use crate::export;

#[derive(Deserialize)]
struct Account {
    path: String,
    xfp: String,
    key: Option<String>,
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateAptosSignRequest
    fn generate_aptos_sign_request(
        request_id: &str,
        sign_data: &str,
        accounts: &str,
        origin: &str,
        sign_type: u32
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();

        let mut is_accounts_err = false;
        let mut account_keys: Vec<Vec<u8>> = vec![];
        let derivation_paths: Vec<CryptoKeyPath> = match serde_json::from_str::<Vec<Account>>(accounts) {
            Ok(v) => v,
            Err(_) => return json!({"error": "accounts is invalid"}).to_string(),
        }.iter().map(|account| {
            if account.key.is_some() {
                let key = match hex::decode(account.key.clone().unwrap()) {
                    Ok(v) => v,
                    Err(_) => {
                        is_accounts_err = true;
                        vec![]
                    },
                };
                account_keys.push(key)
            }
            let xfp = match hex::decode(account.xfp.clone()) {
                Ok(v) => Some(v),
                Err(_) => {
                    is_accounts_err = true;
                    None
                },
            };
            if xfp.is_some() && xfp.as_ref().unwrap().len() != 4 {
                is_accounts_err = true;
                return None
            }
            match CryptoKeyPath::from_path(account.path.to_string(), xfp.map(|v| v.as_slice().try_into().ok()).flatten()) {
                Ok(v) => Some(v),
                Err(_) => {
                    is_accounts_err = true;
                    None
                },
            }
        }).filter_map(|x| x).collect();
        if is_accounts_err || derivation_paths.len() == 0 {
            return json!({"error": "accounts is invalid"}).to_string();
        }

        let sign_type = match SignType::from_u32(sign_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign type is invalid"}).to_string(),
        };
        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        if account_keys.len() != 0 && account_keys.len() != derivation_paths.len() {
            return json!({"error": "account and path count must match"}).to_string()
        }
        let accounts = if account_keys.len() == 0  { None } else { Some(account_keys) };

        let cbor_bytes = match AptosSignRequest::new(
            request_id,
            sign_date_bytes,
            derivation_paths,
            accounts,
            origin,
            sign_type
        ).to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = "aptos-sign-request";
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
    fn test_generate_aptos_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/637'/0'/0'/0'",
                    "xfp": "78230804",
                    "key": "aa7420c68c16645775ecf69a5e2fdaa4f89d3293aee0dd280e2d97ad7b879650"
                },
                {
                    "path": "m/44'/637'/0'/0'/1'",
                    "xfp": "78230805",
                    "key": "97f95acfb04f84d228dce9bda4ad7e2a5cb324d5efdd6a7f0b959e755ebb3a70"
                }
            ]
            "#;
        let origin = "aptosWallet";
        let expect_result = "{\"cbor\":\"a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258208e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e0382d90130a2018a182cf519027df500f500f500f5021a78230804d90130a2018a182cf519027df500f500f501f5021a7823080504825820aa7420c68c16645775ecf69a5e2fdaa4f89d3293aee0dd280e2d97ad7b879650582097f95acfb04f84d228dce9bda4ad7e2a5cb324d5efdd6a7f0b959e755ebb3a70056b6170746f7357616c6c65740601\",\"type\":\"aptos-sign-request\"}";

        assert_eq!(expect_result, generate_aptos_sign_request(
            request_id, sign_data, accounts, origin, sign_type
        ));
    }

    #[test]
    fn test_generate_aptos_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "",
                    "xfp": "78230804",
                    "key": "aa7420c68c16645775ecf69a5e2fdaa4f89d3293aee0dd280e2d97ad7b879650"
                },
                {
                    "path": "m/44'/637'/0'/0'/0'",
                    "xfp": "78230805",
                    "key": "97f95acfb04f84d228dce9bda4ad7e2a5cb324d5efdd6a7f0b959e755ebb3a70"
                }
            ]
            "#;
        let origin = "aptosWallet";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(expect_result, generate_aptos_sign_request(
            request_id, sign_data, accounts, origin, sign_type
        ));
    }

    #[test]
    fn test_generate_aptos_sign_request_account_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "",
                    "xfp": "78230804",
                    "key": "0xaa7420c68c16645775ecf69a5e2fdaa4f89d3293aee0dd280e2d97ad7b879650"
                },
                {
                    "path": "m/44'/637'/0'/0'/0'",
                    "xfp": "78230805",
                    "key": "0x97f95acfb04f84d228dce9bda4ad7e2a5cb324d5efdd6a7f0b959e755ebb3a70"
                }
            ]
            "#;
        let origin = "aptosWallet";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(expect_result, generate_aptos_sign_request(
            request_id, sign_data, accounts, origin, sign_type
        ));
    }

    #[test]
    fn test_generate_aptos_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "8e53e7b10656816de70824e3016fc1a277e";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/637'/0'/0'/0'",
                    "xfp": "78230804",
                    "key": "aa7420c68c16645775ecf69a5e2fdaa4f89d3293aee0dd280e2d97ad7b879650"
                },
                {
                    "path": "m/44'/637'/0'/0'/0'",
                    "xfp": "78230805",
                    "key": "97f95acfb04f84d228dce9bda4ad7e2a5cb324d5efdd6a7f0b959e755ebb3a70"
                }
            ]
            "#;
        let origin = "aptosWallet";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(expect_result, generate_aptos_sign_request(
            request_id, sign_data, accounts, origin, sign_type
        ));
    }
}
