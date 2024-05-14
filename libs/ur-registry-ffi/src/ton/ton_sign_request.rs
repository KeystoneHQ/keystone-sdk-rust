use hex;
use serde_json::json;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::registry_types::TON_SIGN_REQUEST;
use ur_registry::ton::ton_sign_request::{DataType, TonSignRequest};
use uuid::Uuid;

use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTonSignRequest
    fn generate_ton_sign_request(
        request_id: &str, //optional
        sign_data: &str,
        data_type: u32,
        address: &str,
        derivation_path: &str, //optional
        xfp: &str, //optional
        origin: &str //optional
    ) -> String {
        let address = match address {
            "" => return json!({"error": "address is required"}).to_string(),
            _x => _x.to_string()
        };

        let sign_data = match sign_data {
            "" => return json!({"error": "sign data is required"}).to_string(),
            _x => _x
        };


        let request_id = match request_id {
            "" => None,
            _ => match Uuid::parse_str(request_id) {
                Ok(v) => Some(v.as_bytes().to_vec()),
                Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
            },
        };

        let derivation_path = match (derivation_path, xfp) {
            ("", _) | (_, "") => None,
            (_path, _xfp) => {
                let xfp = match hex::decode(_xfp) {
                    Ok(v) => Some(v),
                    Err(_) => {
                        return json!({"error": "xfp is invalid"}).to_string()
                    },
                };
                match CryptoKeyPath::from_path(_path.to_string(), xfp.map(|v| v.as_slice().try_into().ok()).flatten()) {
                    Ok(v) => Some(v),
                    Err(_) => return json!({"error": "path is invalid"}).to_string(),
                }
            },
        };

        let origin = match origin {
            "" => None,
            _ => Some(origin.to_string()),
        };

        let sign_data_bytes = match hex::decode(remove_prefix_0x(sign_data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "intent message is invalid"}).to_string(),
        };

        let data_type = match DataType::from_u32(data_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data type is invalid"}).to_string(),
        };

        let cbor_bytes: Vec<u8> = match TonSignRequest::new(
            request_id,
            sign_data_bytes,
            data_type,
            derivation_path,
            address,
            origin,
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = TON_SIGN_REQUEST.get_type();
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
    fn test_generate_ton_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f";
        let address = "UQC1IywyQwixSOU8pezOZDC9rv2xCV4CGJzOWH6RX8BTsGJx";
        let data_type = 1;
        let origin = "TonKeeper";
        let expect_result = "{\"cbor\":\"a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025856b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f0301057830555143314979777951776978534f553870657a4f5a4443397276327843563443474a7a4f574836525838425473474a780669546f6e4b6565706572\",\"type\":\"ton-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_ton_sign_request(request_id, sign_data, data_type, address, "", "", origin)
        );
    }

    #[test]
    fn test_generate_ton_sign_request_data_type_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f";
        let address = "UQC1IywyQwixSOU8pezOZDC9rv2xCV4CGJzOWH6RX8BTsGJx";
        let data_type = 0;
        let origin = "TonKeeper";

        let expect_result = "{\"error\":\"data type is invalid\"}";

        assert_eq!(
            expect_result,
            generate_ton_sign_request(request_id, sign_data, data_type, address, "", "", origin)
        );
    }

    #[test]
    fn test_generate_ton_sign_request_err_address() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f";
        let address = "";
        let data_type = 0;
        let origin = "TonKeeper";

        let expect_result = "{\"error\":\"address is required\"}";

        assert_eq!(
            expect_result,
            generate_ton_sign_request(request_id, sign_data, data_type, address, "", "", origin)
        );
    }

    #[test]
    fn test_generate_ton_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "";
        let address = "UQC1IywyQwixSOU8pezOZDC9rv2xCV4CGJzOWH6RX8BTsGJx";
        let data_type = 0;
        let origin = "TonKeeper";

        let expect_result = "{\"error\":\"sign data is required\"}";

        assert_eq!(
            expect_result,
            generate_ton_sign_request(request_id, sign_data, data_type, address, "", "", origin)
        );
    }
}
