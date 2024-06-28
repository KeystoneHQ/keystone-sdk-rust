use crate::export;
use anyhow::{format_err, Error};
use bip32::{DerivationPath, XPub};
use hex;
use secp256k1::{Parity, XOnlyPublicKey};
use serde_json::json;
use std::str::FromStr;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_getUncompressedKey
    fn get_uncompressed_key(
        compressed_key: &str
    ) -> String {
        let slice = &compressed_key.clone()[2..];
        let decoded_slice = hex::decode(slice).unwrap();

        let result = match XOnlyPublicKey::from_slice(&decoded_slice) {
            Ok(res) => res,
            Err(_) => return json!({"error": "compressed key is invalid"}).to_string(),
        };

        let prefix = &compressed_key[..2];
        let parity = if prefix.eq("02") {
            Parity::Even
        } else {
            Parity::Odd
        };

        let uncompressed_key = result.public_key(parity).serialize_uncompressed();
        json!({"result": hex::encode(uncompressed_key)}).to_string()
    }

    @Java_com_keystone_sdk_KeystoneNativeSDK_derivePublicKey
    fn derive_public_key(
        xpub: &str,
        path: &str
    ) -> String {
        let derived_public_key = || -> Result<bip32::PublicKeyBytes, Error> {
            let extended_pubkey = XPub::from_str(xpub).map_err(|_| format_err!(""))?;
            let derivation_path = DerivationPath::from_str(path).map_err(|_| format_err!(""))?;
            let derived_key = derivation_path.iter().fold(Ok(extended_pubkey), |acc: Result<XPub, Error>, cur| {
                acc.and_then(|v| v.derive_child(cur).map_err(|_|format_err!("")))
            })?;
            let pubkey_bytes = derived_key.to_bytes();
            Ok(pubkey_bytes)
        };
        match derived_public_key() {
            Ok(derived_public_key) => json!({"result": hex::encode(derived_public_key)}).to_string(),
            Err(_) => json!({"error": "can not derive public key"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_uncompressed_key_from_even_y() {
        let compressed_key = "02fef03a2bd3de113f1dc1cdb1e69aa4d935dc3458d542d796f5827abbb1a58b5e";
        let expect_result = r#"{"result":"04fef03a2bd3de113f1dc1cdb1e69aa4d935dc3458d542d796f5827abbb1a58b5ebdffecfa6587da3216d50114700e5e314650cc2268e9fcb6ac31593bcc71d178"}"#;

        assert_eq!(expect_result, get_uncompressed_key(compressed_key));
    }

    #[test]
    fn test_get_uncompressed_key_from_odd_y() {
        let compressed_key = "03b7db1c60fed9f333a5afb0f945c4fafc7739775bc4bda24ac6979362eca0f1f2";
        let expect_result = r#"{"result":"04b7db1c60fed9f333a5afb0f945c4fafc7739775bc4bda24ac6979362eca0f1f2ae4a073a1eb2f8bad6ddb7bcee0c475456d0c490eec0913c7bc30826fff3193d"}"#;

        assert_eq!(expect_result, get_uncompressed_key(compressed_key));
    }

    #[test]
    fn test_derive_public_key() {
        let xpub = "xpub6DXryz8Kd7XchtXvDnkjara83shGJH8ubu7KZhHhPfp4L1shvDEYiFZm32EKHnyo4bva4gxXjabFGqY7fNs8Ggd4khYz2oNs2KYLf56a9GX";
        let expect_result =
            r#"{"result":"02bac0f67b40d388965912461c2e508b67ed57b88835ee519e5ce8daac0d468573"}"#;

        assert_eq!(expect_result, derive_public_key(xpub, "m/0/0"))
    }
}
