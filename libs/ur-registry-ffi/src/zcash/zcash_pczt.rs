use crate::{export, util_internal::string_helper::remove_prefix_0x};
use anyhow::{format_err, Error};
use serde_json::json;
use ur_registry::{registry_types::ZCASH_PCZT, zcash::zcash_pczt::ZcashPczt};

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateZcashPczt
    fn generate_zcash_pczt(
        data: &str
    ) -> String {
        let data = match data {
            "" => return json!({"error": "data is required"}).to_string(),
            _x => _x.to_string()
        };

        let bytes = match hex::decode(data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "data is invalid"}).to_string(),
        };

        let cbor_bytes: Vec<u8> = match ZcashPczt::new(
            bytes,
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur_type = ZCASH_PCZT.get_type();
        let ur = json!({
            "type": ur_type,
            "cbor": cbor_hex,
        });
        ur.to_string()
    }

    @Java_com_keystone_sdk_KeystoneNativeSDK_parseZcashPczt
    fn parse_zcash_pczt(ur_type: &str, cbor_hex: &str) -> String {
        if ZCASH_PCZT.get_type() != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let parse = || -> Result<String, Error> {
            let cbor = hex::decode(remove_prefix_0x(cbor_hex).to_string())?;
            let pczt = ZcashPczt::try_from(cbor).map_err(|_| format_err!(""))?;
            let pczt_hex = hex::encode(pczt.get_data());
            Ok(pczt_hex)
        };
        match parse() {
            Ok(v) => json!({
                "pczt": v,
            }).to_string(),
            Err(_) => json!({"error": "PCZT is invalid"}).to_string(),
        }
    }
}
