use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::ergo::ergo_sign_request::{DataType, ErgoSignRequest};
use ur_registry::ergo::ergo_unspent_box::{ErgoAsset, ErgoUnspentBox};
use ur_registry::registry_types::ERGO_SIGN_REQUEST;
use crate::export;
use crate::util_internal::string_helper::remove_prefix_0x;

#[derive(Deserialize)]
struct Box {
    box_id: String,
    value: u64,
    ergo_tree: String,
    assets: Option<Vec<Asset>>
}

#[derive(Deserialize)]
struct Asset {
    token_id: String,
    amount: u64
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateErgoSignRequest
    fn generate_ergo_sign_request(
         request_id: &str,
         sign_data: &str,
         boxes: &str,
         derivation_paths: &str,
         origin: &str
    ) -> String {
        let request_id = match Uuid::parse_str(request_id) {
            Ok(id) => id,
            Err(_) => return json!({"error" : "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();

        let boxes: Vec<ErgoUnspentBox> = match serde_json::from_str::<Vec<Box>>(boxes) {
            Ok(v) => v,
            Err(_) => return json!({"error": "box is invalid"}).to_string(),
        }.iter().map(|ergo_box| {
            let assets: Option<Vec<ErgoAsset>> = match &ergo_box.assets {
                Some(a) => Some(
                    a.iter().map(|asset| {
                        ErgoAsset::new(
                            asset.token_id.clone(),
                            asset.amount,
                        )
                    }).collect()),
                None => None,
            };

            ErgoUnspentBox::new(
                ergo_box.box_id.clone(),
                ergo_box.value,
                ergo_box.ergo_tree.clone(),
                assets
            )
        }).collect();

        let derivation_paths: Vec<CryptoKeyPath> = match serde_json::from_str::<Vec<String>>(derivation_paths) {
            Ok(paths) => paths,
            Err(_) => return json!({"error" : "derivation paths are invalid"}).to_string(),
        }.iter().map(|path| CryptoKeyPath::from_path(path.to_string(), None))
        .flatten()
        .collect();

        if derivation_paths.len() == 0 {
            return json!({"error" : "derivation paths is invalid"}).to_string();
        }

        let sign_date_bytes = match hex::decode(remove_prefix_0x(sign_data)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };

         let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let cbor_bytes: Vec<u8> = match ErgoSignRequest::new(
            request_id,
            sign_date_bytes,
            DataType::Transaction,
            derivation_paths,
            boxes,
            origin,
        ).try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error" : "sign data is invalid"}).to_string(),
        };
        let cbor_hex = hex::encode(cbor_bytes);
        let ur = json!({
            "type": ERGO_SIGN_REQUEST.get_type(),
            "cbor": cbor_hex,
        });
        ur.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_ergo_sign_request() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = "9402011a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c00000001fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e4003c0843d0008cd02dc5b9d9d2081889ef00e6452fb5ad1730df42444ceccb9ea02258256d2fbd262e4f25601006400c0843d1005040004000e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a701730073011001020402d19683030193a38cc7b2a57300000193c2b2a57301007473027303830108cdeeac93b1a57304e4f2560000809bee020008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aae4f25601006400cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa0000";
        let derivation_paths = r#"[
           "m/44'/429'/0'/0/6"
        ]"#;

        let unspent_boxes = r#"[
            {
               "box_id": "1a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c",
               "value": 8000000,
               "ergo_tree": "0008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa",
               "assets": [
                  {
                   "token_id": "fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e40",
                   "amount": 200
                  }
               ]
            }
        ]"#;

        let origin = "ergo-wallet";

        let expect_result =  "{\"cbor\":\"a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0259013a9402011a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c00000001fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e4003c0843d0008cd02dc5b9d9d2081889ef00e6452fb5ad1730df42444ceccb9ea02258256d2fbd262e4f25601006400c0843d1005040004000e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a701730073011001020402d19683030193a38cc7b2a57300000193c2b2a57301007473027303830108cdeeac93b1a57304e4f2560000809bee020008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aae4f25601006400cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa000003010481d90130a1018a182cf51901adf500f500f406f40581d920d3a401784031613966313562666163393337396338383266653062376563623232383831353363653466326465663466323732323134666238306638653236333066303463021a007a12000378483030303863643033383866613534333338313437333731303233616163623834366339366335376537326364636437336263383564323032353034363765356237396466613261610481d920d4a2017840666262616163373333376430353163313066633364613063636238363466346433326434303032373535316531633365613363653336316633396239316534300218c8066b6572676f2d77616c6c6574\",\"type\":\"ergo-sign-request\"}";

        assert_eq!(
            expect_result,
            generate_ergo_sign_request(request_id, sign_data, unspent_boxes, derivation_paths, origin)
        );
    }
}

