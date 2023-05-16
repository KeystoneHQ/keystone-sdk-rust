use anyhow::format_err;
use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::arweave::arweave_signature::ArweaveSignature;
use ur_registry::registry_types::ARWEAVE_SIGNATURE;
use ur_registry::traits::From;
use uuid::Uuid;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseArweaveSignature
    fn parse_arweave_signature(ur_type: &str, cbor_hex: &str) -> String {
        if ARWEAVE_SIGNATURE.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let arweave_signature = ArweaveSignature::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let uuid = arweave_signature.get_request_id().ok_or(format_err!(""))?;
            let uuid_hex = hex::encode(uuid);
            let request_id = Uuid::parse_str(&uuid_hex)?.to_string();
            let signature = hex::encode(arweave_signature.get_signature());
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
    fn test_parse_arweave_signature() {
        let arweave_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0259020080337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575";
        let expect_result = "{\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"signature\":\"80337c3a47f1b69a38544c69f379a4aa0ea8ef1f853b718d992c6a73c643e63ca6dff9186cd2f41a45c6405ef6b71353c3b6864c799699964e559afa7aa7f7c345c1966c998193539985e2724831025beadb0a1a269f54ec4a95c69a3bc4295a5c6c5f926dcc84fbf2251b56c841f764b162e062c8db5302090aa1d528d83cf48b53aa0709009f3975d63ea8ff26e80b4f2f01380e100860b304fccbbc0877278efbf72fb045331f76df132a5119bd51590f0502350d3cb31f14daba731893c5834e2e8bfa5bf517ac63693b81041cf7f8ed7293d034b3e54c4d02c66542d3b9648e9ecf912101a20b87f39d75d4f1a02c816f424c8a1fda05a9e7e8ccf064d31c0bf10c661872a7f40c0b1d75dbfae6a95ddcc81eead3f49cfa3803517cf9d79f2541041416c3e8ecfc0292d864f34fe613866e86b7b0bc7abc5b3f84e6ee3b06933c4f82552bb985f6b7fac0a580e94d7a0e8e295dd2e49ece66ead0ee6a46b84553302b94701a9d24b91c085154b7e67a7ac59e3a41ae96c8e1afd1aa778633457005555cff4198820c2aa8ea1ff0f86a9f4ae03d96b215449c63bff7cae9a114c9db05cc4e4d9993a13149393b6a6992b6042bb82d34ffdc7f1aeaf17fa5240ca6ebd9e62fd6c90bce91747af37bf8fc3c72859a1dfec2cf2c49295e1ccdc09b91d9074d204dea74a70002baa05fc86acfcff45fe7f0dd7e5e24c8f69575\"}";

        assert_eq!(
            expect_result,
            parse_arweave_signature("arweave-signature", arweave_signature_cbor)
        );
    }

    #[test]
    fn test_parse_arweave_signature_type_error() {
        let eth_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_arweave_signature("sol-signature", eth_signature_cbor)
        );
    }

    #[test]
    fn test_parse_arweave_signature_error() {
        let eth_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_arweave_signature("arweave-signature", eth_signature_cbor)
        );
    }
}
