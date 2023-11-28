use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::stellar::stellar_sign_request::{DataType, StellarSignRequest};
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
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateStellarSignRequest
    fn generate_stellar_sign_request(
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

        let cbor_bytes: Vec<u8> = match StellarSignRequest::new(
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
        let ur_type = "stellar-sign-request";
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
    fn test_generate_stellar_sign_request() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "0000000200000000b4152f0e761e32152a5ab1e2b5b1830c55d4e9542266ca5189a4c798bbd2ce28000000c80001c7c6000000860000000100000000000000000000000065601b8b000000000000000200000000000000070000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c00000001414243000000000100000000000000010000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c0000000000000000d0b099870000000000000000";
        let data_type = 8;
        let accounts = r#"
            [
                {
                    "path": "m/44'/148'/0'",
                    "xfp": "f23f9fd2",
                    "address": "GAMZNSHDTWAGLYAPNSCIURL6PVJBEBGGC7DSKX77NF2IGG6SFFGMZIY7"
                }
            ]
            "#;
        let origin = "Lobstr";

        let expect_result = "{\"cbor\":\"a601d825507afd5e09926743fba02e08c4a09417ec0258c80000000200000000b4152f0e761e32152a5ab1e2b5b1830c55d4e9542266ca5189a4c798bbd2ce28000000c80001c7c6000000860000000100000000000000000000000065601b8b000000000000000200000000000000070000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c00000001414243000000000100000000000000010000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c0000000000000000d0b09987000000000000000003080481d90130a20186182cf51894f500f5021af23f9fd20581783847414d5a4e534844545741474c5941504e53434955524c3650564a4245424747433744534b5837374e463249474736534646474d5a49593706664c6f62737472\",\"type\":\"stellar-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_stellar_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_stellar_sign_request_path_error() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "0000000200000000b4152f0e761e32152a5ab1e2b5b1830c55d4e9542266ca5189a4c798bbd2ce28000000c80001c7c6000000860000000100000000000000000000000065601b8b000000000000000200000000000000070000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c00000001414243000000000100000000000000010000000011144aea7add6c85858be9dbc4d4a5f756037925941675926c69b11ebe7f1f8c0000000000000000d0b099870000000000000000";
        let data_type = 8;
        let accounts = "[]";
        let origin = "Lobstr";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(
            expect_result,
            generate_stellar_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }

    #[test]
    fn test_generate_stellar_sign_request_err_sign_data() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "123412341";
        let data_type = 8;
        let accounts = r#"
            [
                {
                    "path": "m/44'/148'/0'",
                    "xfp": "f23f9fd2",
                    "address": "GAMZNSHDTWAGLYAPNSCIURL6PVJBEBGGC7DSKX77NF2IGG6SFFGMZIY7"
                }
            ]
            "#;
        let origin = "Lobstr";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            expect_result,
            generate_stellar_sign_request(request_id, sign_data, data_type, accounts, origin)
        );
    }
}
