use crate::export;
use anyhow::format_err;
use anyhow::Error;
use hex;
use serde::{Deserialize, Serialize};
use serde_json::json;
use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::registry_types::{CRYPTO_HDKEY};
use ur_registry::traits::From;
use crate::util_internal::chain::map_coin_type;

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    chain: String,
    path: String,
    public_key: String,
    name: String,
    chain_code: String,
    extended_public_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    note: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    xfp: Option<String>
}

impl core::convert::From<&CryptoHDKey> for Account {
    fn from(value: &CryptoHDKey) -> Account {
        let hd_path = value
            .get_origin()
            .unwrap()
            .get_components()
            .iter()
            .map(|path| {
                format!(
                    "{}{}",
                    if path.is_wildcard() {
                        "*".to_string()
                    } else {
                        path.get_index().unwrap_or_default().to_string()
                    },
                    if path.is_hardened() { "'" } else { "" }
                )
            })
            .collect::<Vec<String>>()
            .join("/");
        let coin_type = value
            .get_origin()
            .unwrap_or_default()
            .get_components()
            .to_vec()[1]
            .get_index()
            .unwrap_or_default();
        let chain_code = hex::encode(value.get_chain_code().unwrap_or_default());
        let mut xpub = "".to_string();
        if !chain_code.is_empty()
            && value.get_parent_fingerprint().is_some()
            && value.get_origin().is_some()
        {
            xpub = value.get_bip32_key();
        }

        let source_fingerprint = value.get_origin().unwrap_or_default().get_source_fingerprint();
        let xfp = if source_fingerprint.is_some() { Some(hex::encode(source_fingerprint.unwrap())) } else { None };

        Account {
            chain: map_coin_type(coin_type),
            path: format!("m/{}", hd_path),
            public_key: hex::encode(value.get_key()),
            name: value.get_name().unwrap_or_default(),
            chain_code,
            extended_public_key: xpub,
            note: value.get_note(),
            xfp,
        }
    }
}


export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseCryptoHDKey
    fn parse_crypto_hd_key(ur_type: &str, cbor_hex: &str) -> String {
        if CRYPTO_HDKEY.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse_signature = || -> Result<Account, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let crypto_hd_key = CryptoHDKey::from_cbor(cbor).map_err(|_| format_err!(""))?;
            Ok(Account::from(&crypto_hd_key))
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
    fn test_parse_crypto_hd_key() {
        let hd_key_cbor = "a902f403582102cc6d7834204653ff10e0047a2395343cc6df081e76c88d5eee83f346f0b21cb7045820712a9187e5c60c573a5acce855445376e1b74c240e417fe8cb2a8fdfd78d2d9d05d90131a201183c020006d90130a30186182cf5183cf500f5021af23f9fd2030307d90130a2018400f480f40300081a483c932809684b657973746f6e650a706163636f756e742e7374616e64617264";
        let expect_result = "{\"chain\":\"ETH\",\"chain_code\":\"712a9187e5c60c573a5acce855445376e1b74c240e417fe8cb2a8fdfd78d2d9d\",\"extended_public_key\":\"xpub6CBZfsQuZgVnvTcScAAXSxtX5jdMHtX5LdRuygnTScMBbKyjsxznd8XMEqDntdY1jigmjunwRwHsQs3xusYQBVFbvLdN4YLzH8caLSSiAoV\",\"name\":\"Keystone\",\"note\":\"account.standard\",\"path\":\"m/44'/60'/0'\",\"public_key\":\"02cc6d7834204653ff10e0047a2395343cc6df081e76c88d5eee83f346f0b21cb7\",\"xfp\":\"f23f9fd2\"}";

        assert_eq!(
            expect_result,
            parse_crypto_hd_key("crypto-hdkey", hd_key_cbor)
        );
    }

    #[test]
    fn test_parse_crypto_hd_key_wrong_type() {
        let hd_keys_cbor = "A2011A52006EA0028AD9012FA502F403582102FEF03A2BD3DE113F1DC1CDB1E69AA4D935DC3458D542D796F5827ABBB1A58B5E06D90130A3018A182CF5183CF500F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F4035821033F1EDDF1D1BB2762FCFA67FBC35E12DC9968CD2587ADA055210E84F780C1109A06D90130A3018A182CF5183CF501F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102C5FCF766AD77A0C254834D57CE3E6120A2BE5C266E9BABE8A047D1A53CB34F9E06D90130A3018A182CF5183CF502F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102CD0B648CF944CBA7E6BE97BF1F17F0EAB7B9E600D181C421B3BCE6E7F6D941F006D90130A3018A182CF5183CF503F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F40358210351F72104E737E94C7CC66E33307C74D5BBF19216800157AD34EBFE232F23C75106D90130A3018A182CF5183CF504F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102037F8C5FC1074E654FF11619A8BF28DCC3DB5D037191F08EB5722252AF57A4A606D90130A3018A182CF5183CF505F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582103A441895DFBE9C7B3BF8EBA0CE461465A14350D902DF163A0B3F06E4F4843E54F06D90130A3018A182CF5183CF506F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102A8DCDF480733A5B7FB331C9464B7E0EDF5206D8581FE3E26BFD6DE38C8063D4C06D90130A3018A182CF5183CF507F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F4035821037A1A6A48B09D4E3A01223B37C9D1212D8DA20746302009956168E1EA3BD3E0C806D90130A3018A182CF5183CF508F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582103C6F04A813F23799940B6FA44C6CA48ABE04DE9FBB8133B7342DBABC95B0EA48106D90130A3018A182CF5183CF509F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_crypto_hd_key("crypto-account", hd_keys_cbor)
        );
    }
}
