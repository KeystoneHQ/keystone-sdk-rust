use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::bitcoin::btc_sign_request::{BtcSignRequest, DataType};
use ur_registry::crypto_key_path::CryptoKeyPath;
use uuid::Uuid;

use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

#[derive(Deserialize)]
struct Account {
    path: String,
    xfp: String,
    address: Option<String>,
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateBTCSignRequest
    fn generate_btc_sign_request(
        request_id: &str,
        sign_data: &str,
        data_type: u32,
        accounts: &str,
        origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();

        let mut is_accounts_err = false;
        let mut addresses: Vec<String> = vec![];
        let derivation_paths: Vec<CryptoKeyPath> = match serde_json::from_str::<Vec<Account>>(accounts) {
            Ok(v) => v,
            Err(_) => return json!({"error": "accounts is invalid"}).to_string(),
        }.iter().map(|account| {
            addresses.push(if account.address.is_none() { "".to_string() } else { account.address.clone().unwrap() });
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

        let data_type = match DataType::from_u32(data_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data type is invalid"}).to_string(),
        };
        let sign_date_bytes = match hex::decode(remove_prefix_0x(sign_data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let cbor_bytes: Vec<u8> = match BtcSignRequest::new(
            request_id,
            sign_date_bytes,
            data_type,
            derivation_paths,
            Some(addresses),
            origin
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = "btc-sign-request";
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
    fn test_generate_btc_sign_request() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "48656c6c6f2063727970746f20776f726c6421";
        let data_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/0'/0'/0/0",
                    "xfp": "f23f9fd2",
                    "address": "1X5vtf4FeK8e8nvhVuvBg7Khkez7Sp3bd"
                }
            ]
            "#;
        let origin = "BTC Wallet";

        let expect_result = "{\"cbor\":\"a601d825507afd5e09926743fba02e08c4a09417ec025348656c6c6f2063727970746f20776f726c642103010481d90130a2018a182cf500f500f500f400f4021af23f9fd2058178213158357674663446654b3865386e76685675764267374b686b657a375370336264066a4254432057616c6c6574\",\"type\":\"btc-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_btc_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_btc_sign_request_path_error() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "48656c6c6f2063727970746f20776f726c6421";
        let data_type = 1;
        let accounts = "[]";
        let origin = "BTC Wallet";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(
            expect_result,
            generate_btc_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_btc_sign_request_err_sign_data() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "123412341";
        let data_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/0'/0'/0/0",
                    "xfp": "f23f9fd2",
                    "address": "4c2a59190413dff36aba8e6ac130c7a691cfb79f"
                }
            ]
            "#;
        let origin = "BTC Wallet";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            expect_result,
            generate_btc_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }
}
