use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::traits::From;
use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use crate::export;
use serde::{Serialize, Deserialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct HDKey {
    key: String,
    chain_code: String,
    source_fingerprint: String,
    note: Option<String>,
}

impl Into<HDKey> for CryptoHDKey {
    fn into(self) -> HDKey {
        HDKey {
            key: hex::encode(self.get_key()),
            chain_code: hex::encode(self.get_chain_code().unwrap_or_default()),
            source_fingerprint: hex::encode(self.get_origin().unwrap_or_default().get_source_fingerprint().unwrap_or_default()),
            note: self.get_note(),
        }
    }
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_getSourceHDKey
    fn get_source_hd_key(
        cbor_hex: &str
    ) -> String {
        let parse_signature = || -> Result<HDKey, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let crypto_hd_key = CryptoHDKey::from_cbor(res).map_err(|_| format_err!(""))?;
            let hd_key = crypto_hd_key.into();
            Ok(hd_key)
        };
        match parse_signature() {
            Ok(hd_key) => json!(hd_key).to_string(),
            Err(_) => json!({"error": "crypto hd key is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_source_hd_key() {
        let hd_key_cbor = "A902F4035821032F547FD525B6D83CC2C44F939CC1425FA1E98D97D26B00F9E2D04952933C5128045820B92B17B393612FC8E945E5C5389439CA0C0A28C3076C060B15C3F9F6523A9D1905D90131A201183C020006D90130A30186182CF5183CF500F5021A52006EA0030307D90130A2018400F480F40300081AEA156CD409684B657973746F6E650A706163636F756E742E7374616E64617264";
        let expect_result = "{\"chain_code\":\"b92b17b393612fc8e945e5c5389439ca0c0a28c3076c060b15c3f9f6523a9d19\",\"key\":\"032f547fd525b6d83cc2c44f939cc1425fa1e98d97d26b00f9e2d04952933c5128\",\"note\":\"account.standard\",\"source_fingerprint\":\"52006ea0\"}";

        assert_eq!(expect_result, get_source_hd_key(hd_key_cbor));
    }
}