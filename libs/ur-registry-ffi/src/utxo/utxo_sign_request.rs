use hex;
use serde_json::json;
use ur_registry::traits::To;
use ur_registry::utxo::utxo_sign_request::UtxoSignRequest;

use crate::export;
use crate::utxo::utxo_transfer::{construct_tx};

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateUtxoSignRequest
    fn generate_utxo_sign_request(
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

        let result = UtxoSignRequest::new(
            sign_data,
            origin,
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "utxo-sign-request";
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
    fn test_generate_utxo_sign_request() {
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

        let expect_result = "{\"cbor\":\"a1015901581f8b08000000000000ff554d3f4b23411c256bb36c93d52aa40a8b10092c9999dfecfc812beee21214d6603410926e7e33b345305993dcc5fb187e04bf805c7f1f4041b03bacafbd43eceddc56783c788ff7270c0e9ae3cd71e57ce77c537daf6c75d57e096a371c3218ea61ce92c720da2b26c707871aa9f3c85d0a285dcad1b854a3732943e22482b3285ce7d7dbdfdfefe428c4db207cda8ffff492bb46f4d5641a2d1a633ca5580b501cb867ae66c538f8cc8366594669262c37e095f2129d25c88c2a8580b8d13e8d0684694189408568b4954a122141d61525bc744a010a2d35ad170918414de93595256619f1e8508385d6c3fdff30692efb9c77fbacdb2735faa4d78d9262b6292a9c9f70ab2793ed6a9da3dbde4caf776b3d6363287ecc647c3bea7da983733d5aad0aff53bb3cf7ebc1d6dfecd8e0b254cb6ab1b8f866a610bf3eff0b5b8da479f6f9e603ce1eec266c010000\",\"type\":\"utxo-sign-request\"}";
        let timestamp = 1681871353647;

        assert_eq!(expect_result, generate_utxo_sign_request(
            request_id, coin_type, sign_data, xfp, origin, timestamp
        ));
    }

    #[test]
    fn test_generate_utxo_sign_request_sign_data_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let coin_type = 2;
        let sign_data = r#"{ "fee": 2250,"dust_threshold":5460,"memo":""}"#;
        let xfp = "F23F9FD2";
        let timestamp = 1681871353647;
        let origin = "";

        let err_result_derivation_path = "{\"error\":\"transaction data is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_utxo_sign_request(
            request_id, coin_type, sign_data, xfp, origin, timestamp
        ));
    }
}
