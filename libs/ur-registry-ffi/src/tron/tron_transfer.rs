use ethabi::{
    Function, Param, ParamType, StateMutability,
};
use protobuf::Message;
use serde::{Serialize, Deserialize};
use serde_json::Number;
use ur_registry::pb::protoc::{TronTx, LatestBlock, Override};

use crate::tron::types::contract;

use super::types::tron::transaction;

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    name: String,
    symbol: String,
    decimals: Number,
}

fn format_address(address_bytes: Vec<u8>) -> String {
    bs58::encode(address_bytes).with_check().into_string()
}

pub fn raw_to_tx(sign_data_bytes: Vec<u8>, token_info: &str) -> Result<TronTx, &str> {
    let raw_data = match transaction::Raw::parse_from_bytes(&sign_data_bytes) {
        Ok(v) => v,
        Err(_) => {
            return Err("sign data is invalid");
        },
    };
    let token_info = match serde_json::from_str::<TokenInfo>(token_info) {
        Ok(v) => Some(v),
        Err(_) => None
    };
    let override_info = if token_info.is_none() { None } else {
        let info = token_info.unwrap();
        Some(Override {
            token_full_name: info.name,
            token_short_name: info.symbol,
            decimals: info.decimals.as_i64().unwrap() as i32
        })
    };

    let mut ref_block_hash: Vec<u8> = vec![0; 8];
    ref_block_hash.extend_from_slice(&raw_data.ref_block_hash);
    ref_block_hash.extend_from_slice(&[0; 16]);

    let fee = 990000;

    match raw_data.contract[0].type_.value() {
        // TransferContract
        1 => {
            let contract = contract::TransferContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
            match contract {
                Ok(v) => {
                    let tx = TronTx {
                        token: "TRX".to_string(),
                        contract_address: "".to_string(),
                        from: format_address(v.owner_address),
                        to: format_address(v.to_address),
                        memo: "".to_string(),
                        value: v.amount.to_string(),
                        latest_block: Some(LatestBlock {
                            hash: hex::encode(ref_block_hash),
                            number: u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                            timestamp: (raw_data.expiration - 600 * 5 * 1000).into(),
                        }),
                        r#override: override_info,
                        fee,
                    };
                    Ok(tx)
                },
                Err(_) => {
                    return Err("sign data is invalid");
                }
            }
        }
        // TransferAssetContract
        2 => {
            let contract = contract::TransferAssetContract::parse_from_bytes(&raw_data.contract[0].parameter.get_or_default().value);
            match contract {
                Ok(v) => {
                    let tx = TronTx {
                        from: format_address(v.owner_address),
                        to: format_address(v.to_address),
                        value: v.amount.to_string(),
                        fee,
                        latest_block: Some(LatestBlock {
                            hash: hex::encode(ref_block_hash),
                            number: u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                            timestamp: (raw_data.expiration - 600 * 5 * 1000).into()
                        }),
                        token: String::from_utf8(v.asset_name).unwrap(),
                        contract_address: "".to_string(),
                        memo: "".to_string(),
                        r#override: override_info
                    };
                    Ok(tx)
                },
                Err(_) => {
                    return Err("sign data is invalid");
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
                    let tx = TronTx {
                        token: "".to_string(),
                        contract_address: format_address(v.contract_address),
                        from: format_address(v.owner_address),
                        to: to_address,
                        memo: "".to_string(),
                        value,
                        latest_block: Some(LatestBlock {
                            hash: hex::encode(ref_block_hash),
                            number: u16::from_be_bytes([raw_data.ref_block_bytes[0], raw_data.ref_block_bytes[1]]).into(),
                            timestamp: raw_data.timestamp.into(),
                        }),
                        r#override: override_info,
                        fee,
                    };
                    Ok(tx)
                },
                Err(_) => {
                    return Err("sign data is invalid");
                }
            }
        }
        _ => {
            return Err("contract is not supported");
        }
    }
}
