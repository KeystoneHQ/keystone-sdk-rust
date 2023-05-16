use hex;
use serde_json::json;
use ur_registry::keystone::keystone_sign_request::KeystoneSignRequest;
use ur_registry::traits::To;

use crate::export;
use crate::keystone::keystone_tx_transfer::construct_tx;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateKeystoneSignRequest
    fn generate_keystone_sign_request(
        request_id: &str,
        coin_type: i32,
        sign_data: &str,
        xfp: &str,
        origin: &str,
        timestamp: i64
    ) -> String {
        let coin_type = match coin_type.try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "coin not supported"}).to_string(),
        };
        if xfp.len() != 8 {
            return json!({"error": "length of xfp must be exactly 8"}).to_string();
        };
        let sign_data = match construct_tx(coin_type, request_id, sign_data, xfp, timestamp) {
            Ok(tx) => tx,
            Err(err) => return err,
        };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = KeystoneSignRequest::new(
            sign_data,
            origin,
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
    fn test_generate_keystone_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let coin_type = 2;
        let sign_data = r#"
            {
                "fee": 2250,
                "dust_threshold":5460,
                "memo":"",
                "inputs":[{
                    "hash":"a59bcbaaae11ba5938434e2d4348243e5e392551156c4a3e88e7bdc0b2a8f663",
                    "index":1,
                    "owner_key_path":"m/44'/2'/0'/0/0",
                    "utxo":{ "public_key":"0296106b8bba9c7870673756c86e7d883b6979182403a61afe917fb550ebdb93c3", "value":18519750, "script": ""}
                }],
                "outputs":[
                    {"address":"LYrLobZH4c9TTsnqDbdswWpvq9Y2Q3LuY7","value":10000,"is_change":false,"change_address_path":""},
                    {"address":"LZ9NnnLex9dDDeqBsewv2BSf8mojjRAaW3","value":18507500,"is_change":true,"change_address_path":"M/44'/2'/0'/0/0"}
                ]
            }
        "#;
        let xfp = "F23F9FD2";
        let origin = "";

        let expect_result = "{\"cbor\":\"a1015901521f8b0800000000000003554dbd4ac35018a57109591a3b954e25089542c8fdcbfd01076d435188c56aa1b4dbfd726f86621bdb6aeb63f4117c0171f701140437717655c4ddcdacc2e1c0399c1fd7a95507cb6e616cf36c595c175971d978774ad7ed11da53bd8404cf8eb7930ebbb53d05d8586026a4204cc8409b5081312101640450930137cd879f8fc75fb4efc2d6715f76fdb7767057f10e75ac2003adb5c5184a4125a3cc1253b2248cdad85245e218e398674c532ba51560320444cb9c73ea571a275e0711c531e22001b4ca8414880b2aca8ae4561829297025142e1711d51cebdc2a2c72886364c180a219ad3fdd7fb941751631d68a482b422522d46e79413a5ea6054c8e59a686c3d57c9180596d4657eb851a93014d6fc6c2dff6db076570a2faf3796a6f954912bbe8acec664d3a17b99c15d3e9f9911e51fffbf5d3ad5782eae9ff9b3fce1eec266c010000\",\"type\":\"keystone-sign-request\"}";
        let timestamp = 1681871353647;

        assert_eq!(
            expect_result,
            generate_keystone_sign_request(
                request_id, coin_type, sign_data, xfp, origin, timestamp
            )
        );
    }

    #[test]
    fn test_generate_keystone_sign_request_sign_data_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let coin_type = 2;
        let sign_data = r#"{ "fee": 2250,"dust_threshold":5460,"memo":""}"#;
        let xfp = "F23F9FD2";
        let timestamp = 1681871353647;
        let origin = "";

        let err_result_derivation_path = "{\"error\":\"transaction data is invalid\"}";

        assert_eq!(
            err_result_derivation_path,
            generate_keystone_sign_request(
                request_id, coin_type, sign_data, xfp, origin, timestamp
            )
        );
    }
}
