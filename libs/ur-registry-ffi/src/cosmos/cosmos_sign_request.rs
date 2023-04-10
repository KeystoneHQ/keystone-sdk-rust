use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::cosmos::cosmos_sign_request::{DataType, CosmosSignRequest};

use crate::export;

#[derive(Deserialize)]
struct Account {
    path: String,
    xfp: String,
    address: Option<String>,
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateCosmosSignRequest
    fn generate_cosmos_sign_request(
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
        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let cbor_bytes = match CosmosSignRequest::new(
            request_id,
            sign_date_bytes,
            data_type,
            derivation_paths,
            Some(addresses),
            origin
        ).to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = "cosmos-sign-request";
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
    fn test_generate_cosmos_sign_request() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "7B226163636F756E745F6E756D626572223A22323930353536222C22636861696E5F6964223A226F736D6F2D746573742D34222C22666565223A7B22616D6F756E74223A5B7B22616D6F756E74223A2231303032222C2264656E6F6D223A22756F736D6F227D5D2C22676173223A22313030313936227D2C226D656D6F223A22222C226D736773223A5B7B2274797065223A22636F736D6F732D73646B2F4D736753656E64222C2276616C7565223A7B22616D6F756E74223A5B7B22616D6F756E74223A223132303030303030222C2264656E6F6D223A22756F736D6F227D5D2C2266726F6D5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D222C22746F5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D227D7D5D2C2273657175656E6365223A2230227D";
        let data_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/118'/0'/0/0",
                    "xfp": "f23f9fd2",
                    "address": "4c2a59190413dff36aba8e6ac130c7a691cfb79f"
                }
            ]
            "#;
        let origin = "Keplr";

        let expect_result = "{\"cbor\":\"a601d825507afd5e09926743fba02e08c4a09417ec0259016b7b226163636f756e745f6e756d626572223a22323930353536222c22636861696e5f6964223a226f736d6f2d746573742d34222c22666565223a7b22616d6f756e74223a5b7b22616d6f756e74223a2231303032222c2264656e6f6d223a22756f736d6f227d5d2c22676173223a22313030313936227d2c226d656d6f223a22222c226d736773223a5b7b2274797065223a22636f736d6f732d73646b2f4d736753656e64222c2276616c7565223a7b22616d6f756e74223a5b7b22616d6f756e74223a223132303030303030222c2264656e6f6d223a22756f736d6f227d5d2c2266726f6d5f61646472657373223a226f736d6f31667334396a7867797a30306c78363436336534767a767838353667756c64756c6a7a6174366d222c22746f5f61646472657373223a226f736d6f31667334396a7867797a30306c78363436336534767a767838353667756c64756c6a7a6174366d227d7d5d2c2273657175656e6365223a2230227d03010481d90130a2018a182cf51876f500f500f400f4021af23f9fd2058178283463326135393139303431336466663336616261386536616331333063376136393163666237396606654b65706c72\",\"type\":\"cosmos-sign-request\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, accounts, origin
        ));
    }
    
    #[test]
    fn test_generate_cosmos_sign_request_path_error() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "7B226163636F756E745F6E756D626572223A22323930353536222C22636861696E5F6964223A226F736D6F2D746573742D34222C22666565223A7B22616D6F756E74223A5B7B22616D6F756E74223A2231303032222C2264656E6F6D223A22756F736D6F227D5D2C22676173223A22313030313936227D2C226D656D6F223A22222C226D736773223A5B7B2274797065223A22636F736D6F732D73646B2F4D736753656E64222C2276616C7565223A7B22616D6F756E74223A5B7B22616D6F756E74223A223132303030303030222C2264656E6F6D223A22756F736D6F227D5D2C2266726F6D5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D222C22746F5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D227D7D5D2C2273657175656E6365223A2230227D";
        let data_type = 1;
        let accounts = "[]";
        let origin = "Keplr";

        let expect_result = "{\"error\":\"accounts is invalid\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, accounts, origin
        ));
    }

    #[test]
    fn test_generate_cosmos_sign_request_err_sign_data() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "123412341";
        let data_type = 1;
        let accounts = r#"
            [
                {
                    "path": "m/44'/118'/0'/0/0",
                    "xfp": "f23f9fd2",
                    "address": "4c2a59190413dff36aba8e6ac130c7a691cfb79f"
                }
            ]
            "#;
        let origin = "Keplr";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(expect_result, generate_cosmos_sign_request(
            request_id, sign_data, data_type, accounts, origin
        ));
    }
}
