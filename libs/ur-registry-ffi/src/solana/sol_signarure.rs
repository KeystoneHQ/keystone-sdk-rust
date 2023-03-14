use hex;
use serde_json::json;
use ur_registry::solana::sol_signature::SolSignature;
use ur_registry::traits::From;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneSDK_parseSolSignature
	fn parse_sol_signature(
		cbor_hex: &str
	) -> String {
        let cbor = hex::decode(cbor_hex.to_string()).unwrap();
        let res = serde_cbor::from_slice(cbor.as_slice()).unwrap();
        let sol_signature = SolSignature::from_cbor(res).unwrap();

        let uuid_hex = hex::encode(sol_signature.get_request_id().unwrap());
        let request_id = Uuid::parse_str(&uuid_hex).unwrap().to_string();
        let signature = hex::encode(sol_signature.get_signature());

        let sol_signature = json!({
            "request_id": request_id,
            "signature": signature,
        });
        sol_signature.to_string()
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse() {
        let sol_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7\"}";

        assert_eq!(expect_result, parse_sol_signature(sol_signature_cbor));
    }
}
