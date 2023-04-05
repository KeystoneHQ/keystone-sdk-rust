use ur_registry::crypto_hd_key::CryptoHDKey;
use ur_registry::crypto_account::CryptoAccount;
use ur_registry::crypto_output::CryptoOutput;
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

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct MultiHDKeys {
    hd_keys: Vec<HDKey>,
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

impl Into<MultiHDKeys> for CryptoAccount {
    fn into(self) -> MultiHDKeys {
        let hd_keys = self.get_output_descriptors().iter()
            .map(|output| CryptoOutput::get_crypto_key(output))
            .map(|crypto_hd_key| crypto_hd_key.into())
            .collect();
        MultiHDKeys {
            hd_keys: hd_keys,
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

    @Java_com_keystone_sdk_KeystoneNativeSDK_getHDKeys
    fn get_hd_keys(
        cbor_hex: &str
    ) -> String {
        let parse_signature = || -> Result<MultiHDKeys, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let crypto_account = CryptoAccount::from_cbor(res).map_err(|_| format_err!(""))?;
            let multi_hd_keys = crypto_account.into();
            Ok(multi_hd_keys)
        };
        match parse_signature() {
            Ok(multi_hd_keys) => json!(multi_hd_keys).to_string(),
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

    #[test]
    fn test_get_hd_keys() {
        let hd_keys_cbor = "A2011A52006EA0028AD9012FA502F403582102FEF03A2BD3DE113F1DC1CDB1E69AA4D935DC3458D542D796F5827ABBB1A58B5E06D90130A3018A182CF5183CF500F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F4035821033F1EDDF1D1BB2762FCFA67FBC35E12DC9968CD2587ADA055210E84F780C1109A06D90130A3018A182CF5183CF501F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102C5FCF766AD77A0C254834D57CE3E6120A2BE5C266E9BABE8A047D1A53CB34F9E06D90130A3018A182CF5183CF502F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102CD0B648CF944CBA7E6BE97BF1F17F0EAB7B9E600D181C421B3BCE6E7F6D941F006D90130A3018A182CF5183CF503F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F40358210351F72104E737E94C7CC66E33307C74D5BBF19216800157AD34EBFE232F23C75106D90130A3018A182CF5183CF504F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102037F8C5FC1074E654FF11619A8BF28DCC3DB5D037191F08EB5722252AF57A4A606D90130A3018A182CF5183CF505F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582103A441895DFBE9C7B3BF8EBA0CE461465A14350D902DF163A0B3F06E4F4843E54F06D90130A3018A182CF5183CF506F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582102A8DCDF480733A5B7FB331C9464B7E0EDF5206D8581FE3E26BFD6DE38C8063D4C06D90130A3018A182CF5183CF507F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F4035821037A1A6A48B09D4E3A01223B37C9D1212D8DA20746302009956168E1EA3BD3E0C806D90130A3018A182CF5183CF508F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665D9012FA502F403582103C6F04A813F23799940B6FA44C6CA48ABE04DE9FBB8133B7342DBABC95B0EA48106D90130A3018A182CF5183CF509F500F400F4021A52006EA0030509684B657973746F6E650A736163636F756E742E6C65646765725F6C697665";
        let expect_result = "{\"hd_keys\":[{\"chain_code\":\"\",\"key\":\"02fef03a2bd3de113f1dc1cdb1e69aa4d935dc3458d542d796f5827abbb1a58b5e\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"033f1eddf1d1bb2762fcfa67fbc35e12dc9968cd2587ada055210e84f780c1109a\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"02c5fcf766ad77a0c254834d57ce3e6120a2be5c266e9babe8a047d1a53cb34f9e\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"02cd0b648cf944cba7e6be97bf1f17f0eab7b9e600d181c421b3bce6e7f6d941f0\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"0351f72104e737e94c7cc66e33307c74d5bbf19216800157ad34ebfe232f23c751\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"02037f8c5fc1074e654ff11619a8bf28dcc3db5d037191f08eb5722252af57a4a6\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"03a441895dfbe9c7b3bf8eba0ce461465a14350d902df163a0b3f06e4f4843e54f\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"02a8dcdf480733a5b7fb331c9464b7e0edf5206d8581fe3e26bfd6de38c8063d4c\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"037a1a6a48b09d4e3a01223b37c9d1212d8da20746302009956168e1ea3bd3e0c8\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"},{\"chain_code\":\"\",\"key\":\"03c6f04a813f23799940b6fa44c6ca48abe04de9fbb8133b7342dbabc95b0ea481\",\"note\":\"account.ledger_live\",\"source_fingerprint\":\"52006ea0\"}]}";

        assert_eq!(expect_result, get_hd_keys(hd_keys_cbor));
    }
}