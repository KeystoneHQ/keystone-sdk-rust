// use ethabi::{Function, StateMutability, Param, ParamType, Token};
use ethabi::{
    ethereum_types::{H160, U256},
    Function, Param, ParamType, StateMutability, Token,
};
use hex;
use protobuf::{Message, Enum};
use serde_json::json;
use ur_registry::traits::To;
use uuid::Uuid;
use ur_registry::crypto_key_path::CryptoKeyPath;
use ur_registry::tron::tron_sign_request::TronSignRequest;

use crate::export;
use crate::tron::tron_transfer::{LatestBlock, Override};
use crate::tron::types::tron::{transaction};
use crate::tron::types::contract;

use super::tron_transfer::TronTransfer;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronSignRequest
    fn generate_tron_sign_request(
        request_id: &str,
        sign_data: &str,
        path: &str,
        xfp: &str,
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
        // let sign_date_bytes = match serde_json::from_str::<TronTransfer>(sign_data) {
        //     Ok(_) => sign_data.as_bytes().to_vec(),
        //     Err(err) => return json!({"error": "sign data is invalid"}).to_string(),
        // };
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
        println!("--------------------------------");
        println!("raw_data: {:?}", raw_data);
        println!("--------------------------------");
        match raw_data.contract[0].type_.value() {
            // transaction::contract::ContractType::TransferContract => {
            1 => {
                let contract = contract::TransferContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                println!("{:?}", contract);
                match contract {
                    Ok(v) => {
                        let transfer = TronTransfer::new(
                            hex::encode(v.owner_address),
                            hex::encode(v.to_address),
                            v.amount.to_string(),
                            raw_data.fee_limit.into(),
                            LatestBlock::new(hex::encode(raw_data.ref_block_hash), raw_data.ref_block_num.into(), raw_data.timestamp.into()),
                            Some("TRX".to_string()),
                            None,
                            None
                        );
                        sign_data_bytes = json!(transfer).to_string().as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            2 => {
                let contract = contract::TransferAssetContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                println!("{:?}", contract);
                match contract {
                    Ok(v) => {
                        let transfer = TronTransfer::new(
                            hex::encode(v.owner_address),
                            hex::encode(v.to_address),
                            v.amount.to_string(),
                            raw_data.fee_limit.into(),
                            LatestBlock::new(hex::encode(raw_data.ref_block_hash), raw_data.ref_block_num.into(), raw_data.timestamp.into()),
                            Some(hex::encode(v.asset_name)),
                            None,
                            None
                        );
                        sign_data_bytes = json!(transfer).to_string().as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            31 => {
                let contract = contract::TriggerSmartContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
                println!("{:?}", contract);
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
                        let fun = Function {
                            name: "transfer".to_owned(),
                            inputs,
                            outputs,
                            constant: false,
                            state_mutability: StateMutability::Payable,
                        };
                        let decode_input = fun.decode_input(&v.data);
                        println!("decode_input: {:?}", decode_input);

                        // decode_input
                        // println!("decode_input  222: {:?}", decode_input.unwrap()[1].to_owned());
                        let token: Token = decode_input.unwrap()[1].to_owned();

                        let transfer = TronTransfer::new(
                            hex::encode(v.owner_address),
                            "".to_string(),
                            "".to_string(),
                            raw_data.fee_limit.into(),
                            LatestBlock::new(hex::encode(raw_data.ref_block_hash), raw_data.ref_block_num.into(), raw_data.timestamp.into()),
                            None,
                            Some(hex::encode(v.contract_address)),
                            None
                        );
                        sign_data_bytes = json!(transfer).to_string().as_bytes().to_vec();
                    },
                    Err(_) => {
                        return json!({"error": "sign data is invalid"}).to_string();
                    }
                }
            }
            _ => {}
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
        let sign_data_hex = "0a0207902208e1b9de559665c6714080c49789bb2c5aae01081f12a9010a31747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e54726967676572536d617274436f6e747261637412740a1541c79f045e4d48ad8dae00e6a6714dae1e000adfcd1215410d292c98a5eca06c2085fff993996423cf66c93b2244a9059cbb0000000000000000000000009bbce520d984c3b95ad10cb4e32a9294e6338da300000000000000000000000000000000000000000000000000000000000f424070c0b6e087bb2c90018094ebdc03";
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, sign_data_hex, path, xfp, address, origin
        ));
    }

    #[test]
    fn test_generate_tron_sign_request_trc10() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = "0a022e1e2208543e546a43ad4e5f40f8bbd59df82d5a730802126f0a32747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e736665724173736574436f6e747261637412390a0731303031303930121541ee6d1ffba872573971562a70f9ad1dc2d4af8c8b1a1541654eb440c1a0640aca337ad9ebf3a122976a91052001";
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, sign_data_hex, path, xfp, address, origin
        ));
    }
    
    #[test]
    fn test_generate_tron_sign_request_tx() {
        let request_id = "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d";
        let sign_data_hex = "0a02639422088ad966a9b0b6a5d140c0d7d7cf812e5a65080112610a2d747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e5472616e73666572436f6e747261637412300a15417946f66d0fc67924da0ac9936183ab3b07c811261215418cb2ab880d4fa7b33c9645a2276dc9b192902e2d186470818ed4cf812e";
        let path = "m/44'/195'/0'/0'";
        let xfp = "12121212";
        let address = "";
        let origin = "";

        let expect_result = "{\"cbor\":\"a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a12121212\",\"type\":\"tron-sign-request\"}";

        assert_eq!(expect_result, generate_tron_sign_request(
            request_id, sign_data_hex, path, xfp, address, origin
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

        let err_result_derivation_path = "{\"error\":\"xfp is invalid\"}";

        assert_eq!(err_result_derivation_path, generate_tron_sign_request(
            request_id, sign_data, path, xfp, address, origin
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
        
        let err_result = "{\"error\":\"sign data is invalid\"}";

        assert_eq!(err_result, generate_tron_sign_request(
            request_id, sign_data, path, xfp, address, origin
        ));
    }
}
