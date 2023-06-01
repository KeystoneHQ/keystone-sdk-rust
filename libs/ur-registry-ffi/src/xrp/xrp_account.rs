use hex;
use serde_json::json;
use serde::{Deserialize};
use ur_registry::bytes::Bytes;
use ur_registry::traits::From;

use crate::export;

#[derive(Deserialize)]
struct XrpAccount {
    pubkey: String,
    address: String
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseXrpAccount
    fn parse_xrp_account(ur_type: &str, cbor_hex: &str) -> String {
        if "bytes" != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let account_info = Bytes::from_cbor(hex::decode(cbor_hex).unwrap_or_default()).unwrap().get_bytes();
        match serde_json::from_slice::<XrpAccount>(&account_info) {
            Ok(account) => return json!({
                "pubkey": account.pubkey,
                "address": account.address,
            }).to_string(),
            Err(_) => return json!({"error": "signature is invalid"}).to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_xrp_account() {
        let xrp_account_cbor = "587e7b2261646472657373223a2272454873444a7475794c67754c5164777734554455666d4248575364384555764b67222c227075626b6579223a22303236336530663537383038313133326664396531323832396336376239653638313835643766376138626233376237386639386539373663336439643136336536227d";
        let expect_result = "{\"address\":\"rEHsDJtuyLguLQdww4UDUfmBHWSd8EUvKg\",\"pubkey\":\"0263e0f578081132fd9e12829c67b9e68185d7f7a8bb37b78f98e976c3d9d163e6\"}";

        assert_eq!(
            expect_result,
            parse_xrp_account("bytes", xrp_account_cbor)
        );
    }
}
