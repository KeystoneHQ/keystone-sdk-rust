use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::ethereum::eth_signature::EthSignature;
use ur_registry::traits::From;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneSDK_parseETHSignature
	fn parse_eth_signature(
		cbor_hex: &str
	) -> String {
        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let eth_signature = EthSignature::from_cbor(res).map_err(|_| format_err!(""))?;
            let uuid = eth_signature.get_request_id().ok_or(format_err!(""))?;
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(eth_signature.get_signature());
            Ok((request_id, signature))
        };
        match parse_signature() {
            Ok((request_id, signature)) => json!({
                "request_id": request_id,
                "signature": signature,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_eth_signature() {
        let eth_signature_cbor = "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025841d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f71303686b657973746f6e65";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f713\"}";

        assert_eq!(expect_result, parse_eth_signature(eth_signature_cbor));
    }

    #[test]
    fn test_parse_eth_signature_error() {
        let eth_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(expect_result, parse_eth_signature(eth_signature_cbor));
    }
}
