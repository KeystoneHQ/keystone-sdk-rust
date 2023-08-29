use hex;
use serde::Deserialize;
use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use uuid::Uuid;
use ur_registry::cosmos::evm_sign_request::{EvmSignRequest, SignDataType};
use ur_registry::registry_types::EVM_SIGN_REQUEST;

use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

#[derive(Deserialize)]
struct Account {
    path: String,
    xfp: String,
    address: Option<String>,
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateEvmSignRequest
    fn generate_evm_sign_request(
        request_id: &str,
        sign_data: &str,
        data_type: u8,
        custom_chain_identifier: u32,
        account: &str,
        origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();
        let sign_data_bytes = match hex::decode(remove_prefix_0x(sign_data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let data_type = match SignDataType::from_u8(data_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data type is invalid"}).to_string(),
        };
        let account = match serde_json::from_str::<Account>(account) {
            Ok(v) => v,
            Err(_) => return json!({"error": "account is invalid"}).to_string(),
        };
        let xfp = match hex::decode(account.xfp) {
            Ok(v) => Some(v),
            Err(_) => {
                return json!({"error": "xfp is invalid"}).to_string();
            }
        };
        if xfp.is_some() && xfp.as_ref().unwrap().len() != 4 {
            return json!({"error": "xfp is invalid"}).to_string();
        }
        let path = match CryptoKeyPath::from_path(account.path.to_string(), xfp.map(|v| v.as_slice().try_into().ok()).flatten()) {
            Ok(v) => v,
            Err(_) => {
                return json!({"error": "account is invalid"}).to_string();
            },
        };
        let address = match account.address {
            Some(v) => {
                match hex::decode(if v.starts_with("0x") {v[2..].to_string()} else {v}) {
                    Ok(b) => Some(b),
                    Err(_) => {
                        return json!({"error": "address is invalid"}).to_string();
                    }
                }
            }
            None => None
        };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let cbor_bytes: Vec<u8> = match EvmSignRequest::new(
            request_id,
            sign_data_bytes,
            data_type,
            custom_chain_identifier,
            path,
            address,
            origin
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = EVM_SIGN_REQUEST.get_type();
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
    fn test_generate_evm_sign_request() {
        let request_id = "05752335-5d51-4a64-a481-ff2200000000";
        let sign_data = "0A9D010A9A010A1C2F636F736D6F732E62616E6B2E763162657461312E4D736753656E64127A0A2C65766D6F73317363397975617230756E3736677235633871346A3736687572347179706A6B38336E786B7735122C65766D6F73317363397975617230756E3736677235633871346A3736687572347179706A6B38336E786B77351A1C0A07617465766D6F7312113130303030303030303030303030303030127E0A590A4F0A282F65746865726D696E742E63727970746F2E76312E657468736563703235366B312E5075624B657912230A21024F7A8D64E515CCF1E0A92A7C859262F425473CF09A50EBCAF3B06B156624145312040A020801181612210A1B0A07617465766D6F7312103236323530303030303030303030303010A8B4061A0C65766D6F735F393030302D342084C68E01";
        let data_type = 3;
        let custom_chain_identifier = 9000;
        let account = r#"
            {
                "path": "m/44'/60'/0'/0/0",
                "xfp": "f23f9fd2",
                "address": "0x860A4E746FE4FDA40E98382B2F6AFC1D4040CAC7"
            }
            "#;
        let origin = "Keplr Extension";

        let expect_result = "{\"cbor\":\"a701d82550057523355d514a64a481ff2200000000025901330a9d010a9a010a1c2f636f736d6f732e62616e6b2e763162657461312e4d736753656e64127a0a2c65766d6f73317363397975617230756e3736677235633871346a3736687572347179706a6b38336e786b7735122c65766d6f73317363397975617230756e3736677235633871346a3736687572347179706a6b38336e786b77351a1c0a07617465766d6f7312113130303030303030303030303030303030127e0a590a4f0a282f65746865726d696e742e63727970746f2e76312e657468736563703235366b312e5075624b657912230a21024f7a8d64e515ccf1e0a92a7c859262f425473cf09a50ebcaf3b06b156624145312040a020801181612210a1b0a07617465766d6f7312103236323530303030303030303030303010a8b4061a0c65766d6f735f393030302d342084c68e0103030419232805d90130a2018a182cf5183cf500f500f400f4021af23f9fd20654860a4e746fe4fda40e98382b2f6afc1d4040cac7076f4b65706c7220457874656e73696f6e\",\"type\":\"evm-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_evm_sign_request(request_id, sign_data, data_type, custom_chain_identifier, account, origin)
        );
    }

    #[test]
    fn test_generate_evm_sign_request_path_error() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "7B226163636F756E745F6E756D626572223A22323930353536222C22636861696E5F6964223A226F736D6F2D746573742D34222C22666565223A7B22616D6F756E74223A5B7B22616D6F756E74223A2231303032222C2264656E6F6D223A22756F736D6F227D5D2C22676173223A22313030313936227D2C226D656D6F223A22222C226D736773223A5B7B2274797065223A22636F736D6F732D73646B2F4D736753656E64222C2276616C7565223A7B22616D6F756E74223A5B7B22616D6F756E74223A223132303030303030222C2264656E6F6D223A22756F736D6F227D5D2C2266726F6D5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D222C22746F5F61646472657373223A226F736D6F31667334396A7867797A30306C78363436336534767A767838353667756C64756C6A7A6174366D227D7D5D2C2273657175656E6365223A2230227D";
        let data_type = 1;
        let custom_chain_identifier = 9000;
        let account = "{}";
        let origin = "Keplr";

        let expect_result = "{\"error\":\"account is invalid\"}";

        assert_eq!(
            expect_result,
            generate_evm_sign_request(request_id, sign_data, data_type, custom_chain_identifier, account, origin)
        );
    }

    #[test]
    fn test_generate_evm_sign_request_err_sign_data() {
        let request_id = "7AFD5E09-9267-43FB-A02E-08C4A09417EC";
        let sign_data = "123412341";
        let data_type = 1;
        let custom_chain_identifier = 9000;
        let account = r#"
            {
                "path": "m/44'/118'/0'/0/0",
                "xfp": "f23f9fd2",
                "address": "4c2a59190413dff36aba8e6ac130c7a691cfb79f"
            }
            "#;
        let origin = "Keplr";

        let expect_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(
            expect_result,
            generate_evm_sign_request(request_id, sign_data, data_type, custom_chain_identifier, account, origin)
        );
    }
}
