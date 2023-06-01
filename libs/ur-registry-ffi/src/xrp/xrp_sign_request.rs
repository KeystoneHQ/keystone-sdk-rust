use hex;
use serde_json::json;
use ur_registry::bytes::Bytes;
use ur_registry::traits::To;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateSignRequest
    fn generate_xrp_sign_request(tx_json_str: &str) -> String {
        let transaction = Bytes::new(Vec::from(tx_json_str));
        match transaction.to_bytes() {
            Ok(v) => {
                let result = hex::encode(&v);
                return json!({"result": result}).to_string()
            },
            Err(_) => return json!({"error": "transaction is invalid"}).to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_generate_xrp_sign_request() {
        let tx_json_str = r#"{"TransactionType":"Payment","Amount":"10000000","Destination":"rHSW257ioNLCsyGNjWqk1RetxZmWYjkAFy","Flags":2147483648,"Account":"rEHsDJtuyLguLQdww4UDUfmBHWSd8EUvKg","Fee":"12","Sequence":79991857,"LastLedgerSequence":80032220,"SigningPubKey":"0263e0f578081132fd9e12829c67b9e68185d7f7a8bb37b78f98e976c3d9d163e6"}"#;
        let expect_result = r#"{"result":"5901387b225472616e73616374696f6e54797065223a225061796d656e74222c22416d6f756e74223a223130303030303030222c2244657374696e6174696f6e223a2272485357323537696f4e4c437379474e6a57716b31526574785a6d57596a6b414679222c22466c616773223a323134373438333634382c224163636f756e74223a2272454873444a7475794c67754c5164777734554455666d4248575364384555764b67222c22466565223a223132222c2253657175656e6365223a37393939313835372c224c6173744c656467657253657175656e6365223a38303033323232302c225369676e696e675075624b6579223a22303236336530663537383038313133326664396531323832396336376239653638313835643766376138626233376237386639386539373663336439643136336536227d"}"#;

        assert_eq!(
            expect_result,
            generate_xrp_sign_request(tx_json_str)
        );
    }
}
