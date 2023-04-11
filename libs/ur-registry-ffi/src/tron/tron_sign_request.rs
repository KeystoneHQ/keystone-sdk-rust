// use ethabi::{Function, StateMutability, Param, ParamType, Token};
use ethabi::{
    Function, Param, ParamType, StateMutability,
};
use hex;
use protobuf::Message;
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::tron::tron_sign_request::TronSignRequest;

use crate::export;
use crate::tron::tron_transfer::LatestBlock;
use crate::tron::types::tron::transaction;
use crate::tron::types::contract;

use super::tron_transfer::{TronTransfer, Override};

fn format_address(address_bytes: Vec<u8>) -> String {
    bs58::encode(address_bytes).with_check().into_string()
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronSignRequest
    fn generate_tron_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
        token_info: &str,
        address: &str,
        origin: &str
    ) -> String {
        let xfp_bytes = match hex::decode(xfp) {
            Ok(v) => v,
            Err(_) => return json!({"error": "xfp is invalid"}).to_string(),
        };
        let xfp_slice: [u8; 4] = match xfp_bytes.as_slice().try_into() {
            Ok(v) => v,
            Err(_) => return json!({"error": "length of xfp must be exactly 8"}).to_string(),
        };
        let derivation_path = match CryptoKeyPath::from_path(path.to_string(), Some(xfp_slice)) {
            Ok(v) => v,
            Err(_) => return json!({"error": "path is invalid"}).to_string(),
        };
        let request_id = match Uuid::parse_str(request_id) {
            Ok(v) => v,
            Err(_) => return json!({"error": "uuid is invalid"}).to_string(),
        }.as_bytes().to_vec();
        let mut sign_data_bytes = match hex::decode(sign_data) {
            Ok(v) => v,
            Err(_) => return json!({"error": "sign data is invalid"}).to_string(),
        };
        let raw_data = match transaction::Raw::parse_from_bytes(&sign_data_bytes) {
            Ok(v) => v,
            Err(err) => {
                println!("{:?}", err);
                return json!({"error": "sign data is invalid"}).to_string();
            },
        };
        let override_info = match serde_json::from_str::<Override>(token_info) {
            Ok(v) => Some(v),
            Err(_) => None
        };

        let mut ref_block_hash: Vec<u8> = vec![0; 8];
        ref_block_hash.extend_from_slice(&raw_data.ref_block_hash);
        ref_block_hash.extend_from_slice(&[0; 16]);

        match raw_data.contract[0].type_.value() {
            // TransferContract
            1 => {
                let contract = contract::TransferContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                match contract {
                    Ok(v) => {
                        let transfer = TronTransfer::new(
                            format_address(v.owner_address),
                            format_address(v.to_address),
                            v.amount.to_string(),
                            raw_data.fee_limit.into(),
                            LatestBlock::new(
                                hex::encode(ref_block_hash),
                                u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                                (raw_data.expiration - 600 * 5 * 1000).into()
                            ),
                            Some("TRX".to_string()),
                            None,
                            override_info
                        );
                        let json = json!(transfer).to_string();
                        sign_data_bytes = json.as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            // TransferAssetContract
            2 => {
                let contract = contract::TransferAssetContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                match contract {
                    Ok(v) => {
                        let transfer = TronTransfer::new(
                            format_address(v.owner_address),
                            format_address(v.to_address),
                            v.amount.to_string(),
                            raw_data.fee_limit.into(),
                            LatestBlock::new(
                                hex::encode(ref_block_hash),
                                u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                                (raw_data.expiration - 600 * 5 * 1000).into()
                            ),
                            Some(String::from_utf8(v.asset_name).unwrap()),
                            None,
                            override_info
                        );
                        let json = json!(transfer).to_string();
                        sign_data_bytes = json.as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            // TriggerSmartContract
            31 => {
                let contract = contract::TriggerSmartContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                match contract {
                    Ok(v) => {
                        let address_params = Param {
                            name: "to".to_owned(),
                            kind: ParamType::Address,
                            internal_type: None,
                        };
                        let value_params = Param {
                            name: "value".to_owned(),
                            kind: ParamType::Uint(256),
                            internal_type: None,
                        };
                        let inputs = vec![address_params, value_params];
                
                        let outputs: Vec<Param> = Vec::new();
                        #[allow(deprecated)]
                        let fun = Function {
                            name: "transfer".to_owned(),
                            inputs,
                            outputs,
                            constant: Some(false),
                            state_mutability: StateMutability::Payable,
                        };
                        let decode_input = fun.decode_input(&v.data[4..]);
                        let inputs = decode_input.unwrap_or_default();
                        let mut to_address_bytes = inputs[0].clone().into_address().unwrap_or_default().to_fixed_bytes().to_vec();
                        to_address_bytes.insert(0, 65);
                        let to_address = format_address(to_address_bytes);
                        let value = inputs[1].clone().into_uint().unwrap_or_default().to_string();
                        let transfer = TronTransfer::new(
                            format_address(v.owner_address),
                            to_address,
                            value,
                            raw_data.fee_limit.into(),
                            LatestBlock::new(
                                hex::encode(ref_block_hash),
                                u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                                raw_data.timestamp.into()
                            ),
                            None,
                            Some(format_address(v.contract_address)),
                            override_info
                        );
                        let json = json!(transfer).to_string();
                        sign_data_bytes = json.as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            _ => {
                return json!({"error": "contract is not supported"}).to_string();
            }
        }

        let address = if address.len() == 0 { None } else { Some(address.as_bytes().to_vec()) };
        let origin = if origin.len() == 0 { None } else { Some(origin.to_string()) };

        let result = TronSignRequest::new(
            Some(request_id),
            sign_data_bytes,
            derivation_path,
            address,
            origin
        );

        let cbor = match result.to_bytes() {
            Ok(v) => v,
            Err(_) => return json!({"error": "cbor serialization failed"}).to_string(),
        };
        let cbor = hex::encode(cbor);
        let ur_type = "tron-sign-request";
        let ur = json!({
            "type": ur_type,
            "cbor": cbor,
        });
        ur.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_tron_sign_request_trc20() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 7, 144, 34, 8, 225, 185, 222, 85, 150, 101, 198, 113, 64, 128, 196, 151,
            137, 187, 44, 90, 174, 1, 8, 31, 18, 169, 1, 10, 49, 116, 121, 112, 101, 46, 103,
            111, 111, 103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111,
            116, 111, 99, 111, 108, 46, 84, 114, 105, 103, 103, 101, 114, 83, 109, 97, 114,
            116, 67, 111, 110, 116, 114, 97, 99, 116, 18, 116, 10, 21, 65, 199, 159, 4, 94, 77,
            72, 173, 141, 174, 0, 230, 166, 113, 77, 174, 30, 0, 10, 223, 205, 18, 21, 65, 13,
            41, 44, 152, 165, 236, 160, 108, 32, 133, 255, 249, 147, 153, 100, 35, 207, 102,
            201, 59, 34, 68, 169, 5, 156, 187, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 155, 188,
            229, 32, 217, 132, 195, 185, 90, 209, 12, 180, 227, 42, 146, 148, 230, 51, 141,
            163, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 15, 66, 64, 112, 192, 182, 224, 135, 187, 44, 144, 1, 128, 148, 235, 220,
            3
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "tokenShortName": "TONE",
            "tokenFullName": "TronOne",
            "decimals": 8
        }"#;

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_trc10() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 46, 30, 34, 8, 84, 62, 84, 106, 67, 173, 78, 95, 64, 248, 187, 213, 157,
            248, 45, 90, 115, 8, 2, 18, 111, 10, 50, 116, 121, 112, 101, 46, 103, 111, 111,
            103, 108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111,
            99, 111, 108, 46, 84, 114, 97, 110, 115, 102, 101, 114, 65, 115, 115, 101, 116, 67,
            111, 110, 116, 114, 97, 99, 116, 18, 57, 10, 7, 49, 48, 48, 49, 48, 57, 48, 18, 21,
            65, 238, 109, 31, 251, 168, 114, 87, 57, 113, 86, 42, 112, 249, 173, 29, 194, 212,
            175, 140, 139, 26, 21, 65, 101, 78, 180, 64, 193, 160, 100, 10, 202, 51, 122, 217,
            235, 243, 161, 34, 151, 106, 145, 5, 32, 1
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "tokenShortName": "TONE",
            "tokenFullName": "TronOne",
            "decimals": 8
        }"#;

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
        ));
    }
    
    #[test]
    fn test_generate_tron_sign_request_tx() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = hex::encode([
            10, 2, 102, 92, 34, 8, 236, 39, 182, 57, 84, 245, 145, 61, 64, 216, 135, 189, 212,
            247, 45, 90, 103, 8, 1, 18, 99, 10, 45, 116, 121, 112, 101, 46, 103, 111, 111, 103,
            108, 101, 97, 112, 105, 115, 46, 99, 111, 109, 47, 112, 114, 111, 116, 111, 99,
            111, 108, 46, 84, 114, 97, 110, 115, 102, 101, 114, 67, 111, 110, 116, 114, 97, 99,
            116, 18, 50, 10, 21, 65, 194, 172, 29, 42, 41, 234, 39, 185, 187, 240, 73, 55, 12,
            53, 5, 19, 156, 124, 157, 144, 18, 21, 65, 238, 109, 31, 251, 168, 114, 87, 57,
            113, 86, 42, 112, 249, 173, 29, 194, 212, 175, 140, 139, 24, 128, 137, 122
        ]);
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "tokenShortName": "TONE",
            "tokenFullName": "TronOne",
            "decimals": 0
        }"#;

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, &sign_data_hex, path, xfp, token_info, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_path_error() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#;
        let path = "";
        let xfp = "1212120";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "tokenShortName": "TONE",
            "tokenFullName": "TronOne",
            "decimals": 0
        }"#;

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_tron_sign_request(
            request_id, sign_data, path, xfp, token_info, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_err_sign_data() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h"}"#;
        let path = "m/44'/501'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";
        let token_info = r#"{
            "tokenShortName": "TONE",
            "tokenFullName": "TronOne",
            "decimals": 0
        }"#;

        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(err_result, generate_tron_sign_request(
            request_id, sign_data, path, xfp, token_info, address, origin
        ));
    }
}
