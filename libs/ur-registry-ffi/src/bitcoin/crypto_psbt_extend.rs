use anyhow::{Error, format_err};
use serde_json::json;
use ur_registry::crypto_psbt_extend::{CryptoPSBTExtend, SupportedPsbtCoin};
use ur_registry::registry_types::CRYPTO_PSBT_EXTEND;
use ur_registry::traits::{From, To};

use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateCryptoPSBTExtend
    fn generate_crypto_psbt_extend(psbt_hex: &str, coin_id: i32) -> String {
        let coin_id = if coin_id == 0 { None } else { Some(i128::from(coin_id)) };
        if coin_id.is_some_and(|id| !SupportedPsbtCoin::is_valid_coin_id(id)) {
            return json!({"error": "invalid coin_id"}).to_string();
        }
        let encode = || -> Result<Vec<u8>, Error> {
            let psbt = hex::decode(remove_prefix_0x(psbt_hex))?;
            let crypto = CryptoPSBTExtend::new(psbt, coin_id)?;
            Ok(crypto.to_bytes()?)
        };
        match encode() {
            Ok(cbor) => json!({"type": CRYPTO_PSBT_EXTEND.get_type(), "cbor": hex::encode(cbor)}).to_string(),
            Err(_) => json!({"error": "PSBT is invalid"}).to_string(),
        }
    }

    @Java_com_keystone_sdk_KeystoneNativeSDK_parseCryptoPSBTExtend
    fn parse_crypto_psbt_extend(ur_type: &str, cbor_hex: &str) -> String {
        if ur_type != CRYPTO_PSBT_EXTEND.get_type() {
            return json!({"error": "type not match"}).to_string();
        }
        let parse = || -> Result<CryptoPSBTExtend, Error> {
            let cbor = hex::decode(remove_prefix_0x(cbor_hex))?;
            let crypto = CryptoPSBTExtend::from_cbor(cbor)?;
            if crypto.get_coin_id().is_some_and(|id| !SupportedPsbtCoin::is_valid_coin_id(id)) {
                return Err(format_err!("invalid coin_id"));
            }
            Ok(crypto)
        };
        match parse() {
            Ok(crypto) => json!({"psbt": hex::encode(crypto.get_psbt()), "coin_id": crypto.get_coin_id()}).to_string(),
            Err(_) => json!({"error": "PSBT is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn supported_coins_round_trip() {
        for coin_id in [0, 2, 3, 5, 145] {
            let generated: Value =
                serde_json::from_str(&generate_crypto_psbt_extend("0x70736274ff", coin_id))
                    .unwrap();
            assert_eq!(generated["type"], "crypto-psbt-extend");
            let parsed: Value = serde_json::from_str(&parse_crypto_psbt_extend(
                "crypto-psbt-extend",
                generated["cbor"].as_str().unwrap(),
            ))
            .unwrap();
            assert_eq!(
                parsed,
                json!({"psbt": "70736274ff", "coin_id": if coin_id == 0 { None } else { Some(coin_id) }})
            );
        }
    }

    #[test]
    fn known_cbor_vectors() {
        assert_eq!(
            generate_crypto_psbt_extend("aabb", 2),
            json!({"type": "crypto-psbt-extend", "cbor": "a20142aabb0202"}).to_string()
        );
        assert_eq!(
            generate_crypto_psbt_extend("aabb", 0),
            json!({"type": "crypto-psbt-extend", "cbor": "a10142aabb"}).to_string()
        );
        assert_eq!(
            parse_crypto_psbt_extend("crypto-psbt-extend", "0xA20142AABB021891"),
            json!({"psbt": "aabb", "coin_id": 145}).to_string()
        );
    }

    #[test]
    fn invalid_inputs() {
        for coin_id in [-1, 1, 4, i32::MAX] {
            assert_eq!(
                generate_crypto_psbt_extend("aabb", coin_id),
                json!({"error": "invalid coin_id"}).to_string()
            );
        }
        assert_eq!(
            generate_crypto_psbt_extend("707", 2),
            json!({"error": "PSBT is invalid"}).to_string()
        );
        assert_eq!(
            parse_crypto_psbt_extend("crypto-psbt", "a10142aabb"),
            json!({"error": "type not match"}).to_string()
        );
        for cbor in ["zz", "a201", "42aabb", "a20142aabb0201", "a20142aabb0204"] {
            assert_eq!(
                parse_crypto_psbt_extend("crypto-psbt-extend", cbor),
                json!({"error": "PSBT is invalid"}).to_string()
            );
        }
    }
}
