use anyhow::format_err;
use anyhow::Error;
use hex;
use serde_json::json;
use ur_registry::arweave::arweave_crypto_account::ArweaveCryptoAccount;
use ur_registry::registry_types::ARWEAVE_CRYPTO_ACCOUNT;
use ur_registry::traits::From;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseArweaveAccount
    fn parse_arweave_account(ur_type: &str, cbor_hex: &str) -> String {
        if ARWEAVE_CRYPTO_ACCOUNT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<(String, String, String), Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let arweave_account = ArweaveCryptoAccount::from_cbor(cbor).map_err(|_| format_err!(""))?;

            let master_fingerprint = hex::encode(arweave_account.get_master_fingerprint());
            let key_data = hex::encode(arweave_account.get_key_data());
            let device = arweave_account.get_device().unwrap_or_default();

            Ok((master_fingerprint, key_data, device))
        };
        match parse_signature() {
            Ok((master_fingerprint, key_data, device)) => json!({
                "master_fingerprint": master_fingerprint,
                "key_data": key_data,
                "device": device,
            }).to_string(),
            Err(_) => json!({"error": "signature is invalid"}).to_string(),
        }
    }
}

use crate::export;

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_arweave_account() {
        let eth_signature_cbor = "a3011ae9181cf302590200c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c9503686b657973746f6e65";
        let expect_result = "{\"device\":\"keystone\",\"key_data\":\"c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c95\",\"master_fingerprint\":\"e9181cf3\"}";

        assert_eq!(
            expect_result,
            parse_arweave_account("arweave-crypto-account", eth_signature_cbor)
        );
    }

    #[test]
    fn test_parse_arweave_account_type_error() {
        let eth_signature_cbor = "a201d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840d4f0a7bcd95bba1fbb1051885054730e3f47064288575aacc102fbbf6a9a14daa066991e360d3e3406c20c00a40973eff37c7d641e5b351ec4a99bfe86f335f7";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_arweave_account("sol-signature", eth_signature_cbor)
        );
    }

    #[test]
    fn test_parse_arweave_account_error() {
        let eth_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_arweave_account("arweave-crypto-account", eth_signature_cbor)
        );
    }
}
