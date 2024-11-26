use crate::export;
use serde_json::json;
use ur_registry::{registry_types::ZCASH_PCZT, zcash::zcash_pczt::ZcashPczt};

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateZcashPcZt
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
}
