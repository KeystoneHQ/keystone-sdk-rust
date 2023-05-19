use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::registry_types::SUI_SIGN_REQUEST;
use ur_registry::sui::sui_sign_request::{SignType, SuiSignRequest};
use uuid::Uuid;

use crate::export;

#[derive(Deserialize)]
struct Account {
    path: String,
    xfp: String,
    address: Option<String>,
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateSuiSignRequest
    fn generate_sui_sign_request(
        request_id: &str,
        sign_data: &str,
        sign_type: u32,
        accounts: &str,
        origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => Some(v.as_bytes().to_vec()),
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        };

        let mut is_accounts_err = false;
        let mut account_addresses: Vec<Vec<u8>> = vec![];
        let derivation_paths: Vec<CryptoKeyPath> = match serde_json::from_str::<Vec<Account>>(accounts) {
            Ok(v) => v,
            Err(_) => return json!({"error": "accounts is invalid"}).to_string(),
        }.iter().map(|account| {
            if let Some(addr_str) = account.address.clone() {
                let addr = match hex::decode(if addr_str.starts_with("0x") { addr_str[2..].to_string() } else { addr_str }) {
                    Ok(v) => v,
                    Err(_) => {
                        is_accounts_err = true;
                        vec![]
                    },
                };
                account_addresses.push(addr)
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

        if account_addresses.len() != 0 && account_addresses.len() != derivation_paths.len() {
            return json!({"error": "account and path count must match"}).to_string()
        }
        let addresses = if account_addresses.len() == 0  { None } else { Some(account_addresses) };

        let cbor_bytes: Vec<u8> = match SuiSignRequest::new(
            request_id,
            sign_date_bytes,
            sign_type,
            derivation_paths,
            addresses,
            origin,
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = SUI_SIGN_REQUEST.get_type();
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
    fn test_generate_sui_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "00000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e803000000000000640000000000000000";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/784'/0'/0'/0'",
                    "xfp": "78230804",
                    "address": "0xebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869"
                },
                {
                    "path": "m/44'/784'/0'/0'/1'",
                    "xfp": "78230805",
                    "address": "1ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d64"
                }
            ]
            "#;
        let origin = "Sui Wallet";
        let expect_result = "{\"cbor\":\"a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258d900000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e80300000000000064000000000000000003010482d90130a2018a182cf5190310f500f500f500f5021a78230804d90130a2018a182cf5190310f500f500f501f5021a7823080505825820ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886958201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d64066a5375692057616c6c6574\",\"type\":\"sui-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_sui_sign_request(request_id, sign_data, sign_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_sui_sign_request_account_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "000002002086ac6179ca6ad9a7b1ccb47202d06ae09a131e66309944922af9c73d3c203b66000810270000000000000202000101010001010200000100000e4d9313fb5b3f166bb6f2aea587edbe21fb1c094472ccd002f34b9d0633c71901d833a8eabc697a0b2e23740aca7be9b0b9e1560a39d2f390cf2534e94429f91ced0c00000000000020190ca0d64215ac63f50dbffa47563404182304e0c10ea30b5e4d671b7173a34c0e4d9313fb5b3f166bb6f2aea587edbe21fb1c094472ccd002f34b9d0633c719e803000000000000640000000000000000";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "",
                    "xfp": "78230804",
                    "address": "0x0e4d9313fb5b3f166bb6f2aea587edbe21fb1c094472ccd002f34b9d0633c719"
                },
                {
                    "path": "m/44'/784'/0'/0'/0'",
                    "xfp": "78230805",
                    "address": "0x0e4d9313fb5b3f166bb6f2aea587edbe21fb1c094472ccd002f34b9d0633c719"
                }
            ]
            "#;
        let origin = "Sui Wallet";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(
            expect_result,
            generate_sui_sign_request(request_id, sign_data, sign_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_sui_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "8e53e7b10656816de70824e3016fc1a277e";
        let sign_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/784'/0'/0'/0'",
                    "xfp": "78230804",
                    "address": "0x0e4d9313fb5b3f166bb6f2aea587edbe21fb1c094472ccd002f34b9d0633c719"
                },
                {
                    "path": "m/44'/784'/0'/0'/1'",
                    "xfp": "78230805",
                    "address": "68a42711caf03f82e5e45452eb4f1223675aeed4a80b4465892495c48648e3c7"
                }
            ]
            "#;
        let origin = "Sui Wallet";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            expect_result,
            generate_sui_sign_request(request_id, sign_data, sign_type, accounts, origin)
        );
    }
}
