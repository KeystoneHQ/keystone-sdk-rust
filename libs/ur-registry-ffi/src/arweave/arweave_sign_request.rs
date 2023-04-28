use hex;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::arweave::arweave_sign_request::{ArweaveSignRequest, SaltLen, SignType};

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateArweaveSignRequest
    fn generate_arweave_sign_request(
        request_id: &str,
        sign_data: &str,
        sign_type: u32,
        salt_len: u32,
        master_fingerprint: &str,
        account: &str,
        origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();

        let xfp_bytes = match hex::decode(master_fingerprint) {
            Ok(v) => v,
            Err(_) => return json!({"error": "master fingerprint is invalid"}).to_string(),
        };
        let xfp_slice: [u8; 4] = match xfp_bytes.as_slice().try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "length of master fingerprint must be exactly 8"}).to_string(),
        };

        let sign_date_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign_data is invalid"}).to_string(),
        };
        let sign_type = match SignType::from_u32(sign_type) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign type is invalid"}).to_string(),
        };
        let salt_len = match SaltLen::from_u32(salt_len) {
            Ok(v) => v,
            Err(_) => return json!({"error": "salt length must be 0 or 32"}).to_string(),
        };

        let account = if account.len() == 0 { None } else { Some(account.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = ArweaveSignRequest::new(
            xfp_slice,
            Some(request_id),
            sign_date_bytes,
            sign_type,
            salt_len,
            account,
            origin
        );
        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "arweave-sign-request";
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
    fn test_generate_arweave_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "7b22666f726d6174223a322c226964223a22222c226c6173745f7478223a22675448344631615059587639314a704b6b6e39495336684877515a3141597949654352793251694f654145547749454d4d5878786e466a30656b42466c713939222c226f776e6572223a22222c2274616773223a5b7b226e616d65223a2256486c775a51222c2276616c7565223a2256484a68626e4e6d5a5849227d2c7b226e616d65223a22513278705a573530222c2276616c7565223a2251584a44623235755a574e30227d2c7b226e616d65223a22513278705a5735304c565a6c636e4e70623234222c2276616c7565223a224d5334774c6a49227d5d2c22746172676574223a226b796977315934796c7279475652777454617473472d494e3965773838474d6c592d795f4c473346784741222c227175616e74697479223a2231303030303030303030222c2264617461223a22222c22646174615f73697a65223a2230222c22646174615f726f6f74223a22222c22726577617264223a2239313037353734333836222c227369676e6174757265223a22227d";
        let sign_type = 1;
        let salt_len = 0;
        let xfp = "e9181cf3";
        let origin = "arconnect";

        let expect_result = "{\"cbor\":\"a6011ae9181cf302d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d035901967b22666f726d6174223a322c226964223a22222c226c6173745f7478223a22675448344631615059587639314a704b6b6e39495336684877515a3141597949654352793251694f654145547749454d4d5878786e466a30656b42466c713939222c226f776e6572223a22222c2274616773223a5b7b226e616d65223a2256486c775a51222c2276616c7565223a2256484a68626e4e6d5a5849227d2c7b226e616d65223a22513278705a573530222c2276616c7565223a2251584a44623235755a574e30227d2c7b226e616d65223a22513278705a5735304c565a6c636e4e70623234222c2276616c7565223a224d5334774c6a49227d5d2c22746172676574223a226b796977315934796c7279475652777454617473472d494e3965773838474d6c592d795f4c473346784741222c227175616e74697479223a2231303030303030303030222c2264617461223a22222c22646174615f73697a65223a2230222c22646174615f726f6f74223a22222c22726577617264223a2239313037353734333836222c227369676e6174757265223a22227d0401050006696172636f6e6e656374\",\"type\":\"arweave-sign-request\"}";

        assert_eq!(expect_result, generate_arweave_sign_request(
            request_id, sign_data, sign_type, salt_len, xfp, "", origin
        ));
    }
    
    #[test]
    fn test_generate_arweave_sign_request_xfp_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "7b22666f726d6174223a322c226964223a22222c226c6173745f7478223a22675448344631615059587639314a704b6b6e39495336684877515a3141597949654352793251694f654145547749454d4d5878786e466a30656b42466c713939222c226f776e6572223a22222c2274616773223a5b7b226e616d65223a2256486c775a51222c2276616c7565223a2256484a68626e4e6d5a5849227d2c7b226e616d65223a22513278705a573530222c2276616c7565223a2251584a44623235755a574e30227d2c7b226e616d65223a22513278705a5735304c565a6c636e4e70623234222c2276616c7565223a224d5334774c6a49227d5d2c22746172676574223a226b796977315934796c7279475652777454617473472d494e3965773838474d6c592d795f4c473346784741222c227175616e74697479223a2231303030303030303030222c2264617461223a22222c22646174615f73697a65223a2230222c22646174615f726f6f74223a22222c22726577617264223a2239313037353734333836222c227369676e6174757265223a22227d";
        let sign_type = 1;
        let salt_len = 0;
        let xfp = "e9181cf";
        let origin = "arconnect";

        let expect_result = "{\"error\":\"master fingerprint is invalid\"}";

        assert_eq!(expect_result, generate_arweave_sign_request(
            request_id, sign_data, sign_type, salt_len, xfp, "", origin
        ));
    }

    #[test]
    fn test_generate_arweave_sign_request_salt_len_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "7b22666f726d6174223a322c226964223a22222c226c6173745f7478223a22675448344631615059587639314a704b6b6e39495336684877515a3141597949654352793251694f654145547749454d4d5878786e466a30656b42466c713939222c226f776e6572223a22222c2274616773223a5b7b226e616d65223a2256486c775a51222c2276616c7565223a2256484a68626e4e6d5a5849227d2c7b226e616d65223a22513278705a573530222c2276616c7565223a2251584a44623235755a574e30227d2c7b226e616d65223a22513278705a5735304c565a6c636e4e70623234222c2276616c7565223a224d5334774c6a49227d5d2c22746172676574223a226b796977315934796c7279475652777454617473472d494e3965773838474d6c592d795f4c473346784741222c227175616e74697479223a2231303030303030303030222c2264617461223a22222c22646174615f73697a65223a2230222c22646174615f726f6f74223a22222c22726577617264223a2239313037353734333836222c227369676e6174757265223a22227d";
        let sign_type = 1;
        let salt_len = 20;
        let xfp = "e9181cf3";
        let origin = "arconnect";

        let expect_result = "{\"error\":\"salt length must be 0 or 32\"}";

        assert_eq!(expect_result, generate_arweave_sign_request(
            request_id, sign_data, sign_type, salt_len, xfp, "", origin
        ));
    }
}
