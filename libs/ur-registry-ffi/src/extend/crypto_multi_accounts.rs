use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::extend::crypto_multi_accounts::CryptoMultiAccounts;
use ur_registry::traits::From;
use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneSDK_parseCryptoMultiAccounts
	fn parse_crypto_multi_accounts(
		cbor_hex: &str
	) -> String {
        let parse_signature = || -> Result<CryptoMultiAccounts, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let multi_accounts = CryptoMultiAccounts::from_cbor(res).map_err(|_| format_err!(""))?;
            Ok(multi_accounts)
        };
        match parse_signature() {
            Ok(multi) => json!(multi).to_string(),
            Err(_) => json!({"error": "crypto multi accounts is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_crypto_multi_accounts() {
        let multi_accounts_cbor = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b06d90130a10188182cf51901f5f500f500f503686b657973746f6e65";
        let expect_result = "{\"device\":\"keystone\",\"keys\":[{\"chain_code\":null,\"children\":null,\"is_master\":false,\"is_private_key\":null,\"key\":\"02eae4b876a8696134b868f88cc2f51f715f2dbedb7446b8e6edf3d4541c4eb67b\",\"name\":null,\"note\":null,\"origin\":\"m/44'/501'/0'/0'\",\"parent_fingerprint\":null,\"use_info\":null}],\"master_fingerprint\":\"e9181cf3\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts(multi_accounts_cbor));
    }

    #[test]
    fn test_parse_crypto_multi_accounts_error() {
        let multi_accounts_cbor = "a3011ae9181cf30281d9012fa203582102eae4b876a8696134b868f88cc2f51f";
        let expect_result = "{\"error\":\"crypto multi accounts is invalid\"}";

        assert_eq!(expect_result, parse_crypto_multi_accounts(multi_accounts_cbor));
    }
}
