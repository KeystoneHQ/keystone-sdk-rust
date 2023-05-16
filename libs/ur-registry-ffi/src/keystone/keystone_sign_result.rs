use anyhow::format_err;
use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::keystone::keystone_sign_result::KeystoneSignResult;
use ur_registry::pb::protobuf_parser::{parse_protobuf, unzip};
use ur_registry::pb::protoc::payload::Content;
use ur_registry::pb::protoc::{Base, SignTransactionResult};
use ur_registry::registry_types::KEYSTONE_SIGN_RESULT;
use ur_registry::traits::From;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseKeystoneSignResult
    fn parse_keystone_sign_result(ur_type: &str, cbor_hex: &str) -> String {
        if KEYSTONE_SIGN_RESULT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_sign_result = || -> Result<SignTransactionResult, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let keystone_sign_result = KeystoneSignResult::from_cbor(cbor).map_err(|_| format_err!(""))?;
            let ziped_sign_result = keystone_sign_result.get_sign_result();
            let sign_result = unzip(ziped_sign_result).map_err(|_| format_err!(""))?;
            let sign_result_base = parse_protobuf::<Base>(sign_result).map_err(|_| format_err!(""))?;
            let payload = sign_result_base.data.unwrap_or_default();
            let content = payload.content.unwrap();
            match content {
                Content::SignTxResult(sign_result) => Ok(sign_result),
                _ => Err(format_err!(""))
            }
        };

        match parse_sign_result() {
            Ok(sign_result) => json!({
                "request_id": sign_result.sign_id,
                "raw_data": sign_result.raw_tx,
            }).to_string(),
            Err(_) => json!({"error": "sign result is invalid"}).to_string()
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_keystone_sign_result() {
        let keystone_sign_result_cbor = "a1015901b11f8b08000000000000004d923d8e14300c85b5628b0121214db9150505cd48fe8b635351a0a9b942123b0d122376b7e12c9c8c1b50517002cca241b88822c7fe64bf97c3cdf1d5a7fcfaf078f99cafbfdcaf4be4dd8fdbc3f3e3e14c7cf6f3077af7fdf6c59bb55c74269d2c659d84669e06139e9a8ec6c3d66ea8c7f7b2713919e0748bec113ef7f018c8a2446d933832c2de9b87efb0ee0a163b8ad17af77df7f319105c030195b70e9bb46016cf8c53862e6c0da9b1b7e4241113a69014666b3e07e2c831d6ac6b2bc65f5447ad53960866065ab22f81ec93338d3036e54a27eca0dcc176eca70042a00eff05f6e128c9c164620bb7f49a0d427dda469ecdc97b20470930b7f55caa38ae735cfbfb54d6ec69211a634ccf05630cf134fcb31fc2ea5533ad03496710012290dda2d74b099292c05a45e62bb74cf766a3483a143266ae51f6706d365a69b3cb26c9f9c468e808d6c0a66bf26ab01741ccca4ccba0020e3441dccaa1adaa2672b9db5bb9d4a9142d49b8a94994538930178eb261468fe6595340dfb4d173bb5a3524962e59a84466d7da5554af6abcfdf6eb865efefb7f1fef2fbf01cb596bb490020000";
        let expect_result = "{\"raw_data\":\"0200000000010163f6a8b2c0bde7883e4a6c155125395e3e2448432d4e433859ba11aeaacb9ba50100000017160014c441eed18e39c40e7b3ee821df2ece9217063708fdffffff02102700000000000017a914e3d32848c1f470bd0d69b8f13b59297d13d8debf87ec661a010000000017a9147b636e7e8d46daab9ec0aaa49e816c1510c77b6b870247304402204f5d70c78b2e4e036c7789cef4b9958adaa6a60edbecaa323821a52d4f56a4eb02204519108508b96e3c50fc20db850b8ed2789a18411f63d658b9b13f5175da57250121035684d200e10bc1a3e2bd7d59e58a07f2f19ef968725e18f1ed65e13396ab946600000000\",\"request_id\":\"cc946be2-8e4c-42be-a321-56a53a8cf516\"}";

        assert_eq!(
            expect_result,
            parse_keystone_sign_result("keystone-sign-result", keystone_sign_result_cbor)
        );
    }

    #[test]
    fn test_parse_keystone_sign_result_type_error() {
        let keystone_sign_result_cbor = "a1015901b11f8b08000000000000004d923d8e14300c85b5628b0121214db9150505cd48fe8b635351a0a9b942123b0d122376b7e12c9c8c1b50517002cca241b88822c7fe64bf97c3cdf1d5a7fcfaf078f99cafbfdcaf4be4dd8fdbc3f3e3e14c7cf6f3077af7fdf6c59bb55c74269d2c659d84669e06139e9a8ec6c3d66ea8c7f7b2713919e0748bec113ef7f018c8a2446d933832c2de9b87efb0ee0a163b8ad17af77df7f319105c030195b70e9bb46016cf8c53862e6c0da9b1b7e4241113a69014666b3e07e2c831d6ac6b2bc65f5447ad53960866065ab22f81ec93338d3036e54a27eca0dcc176eca70042a00eff05f6e128c9c164620bb7f49a0d427dda469ecdc97b20470930b7f55caa38ae735cfbfb54d6ec69211a634ccf05630cf134fcb31fc2ea5533ad03496710012290dda2d74b099292c05a45e62bb74cf766a3483a143266ae51f6706d365a69b3cb26c9f9c468e808d6c0a66bf26ab01741ccca4ccba0020e3441dccaa1adaa2672b9db5bb9d4a9142d49b8a94994538930178eb261468fe6595340dfb4d173bb5a3524962e59a84466d7da5554af6abcfdf6eb865efefb7f1fef2fbf01cb596bb490020000";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_keystone_sign_result("eth-signature", keystone_sign_result_cbor)
        );
    }

    #[test]
    fn test_parse_keystone_sign_result_error() {
        let keystone_sign_result_cbor =
            "a1015901b11f8b08000000000000004d923d8e14300c85b5628b0121214db";
        let expect_result = "{\"error\":\"sign result is invalid\"}";

        assert_eq!(
            expect_result,
            parse_keystone_sign_result("keystone-sign-result", keystone_sign_result_cbor)
        );
    }
}
