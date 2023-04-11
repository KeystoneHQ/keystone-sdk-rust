use ethabi::{
    Function, Param, ParamType, StateMutability,
};
use protobuf::Message;
use serde::{Serialize, Deserialize};
use serde_json::{Number, json};

use crate::tron::types::contract;

use super::types::tron::transaction;

#[derive(Serialize, Deserialize)]
pub struct LatestBlock {
    hash: String,
    number: Number,
    timestamp: Number,
}

impl LatestBlock {
    pub fn new(hash: String, number: Number, timestamp: Number) -> Self { Self { hash, number, timestamp } }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Override {
    #[serde(rename = "tokenShortName")]
    token_short_name: String,
    #[serde(rename = "tokenFullName")]
    token_full_name: String,
    decimals: Number,
}

impl Override {
    pub fn new(token_short_name: String, token_full_name: String, decimals: Number) -> Self { Self { token_short_name, token_full_name, decimals } }
}

#[derive(Serialize, Deserialize)]
pub struct TronTransfer {
    from: String,
    to: String,
    value: String,
    fee: Number,
    #[serde(rename = "latestBlock")]
    latest_block: LatestBlock,
    token: Option<String>,
    #[serde(rename = "contractAddress")]
    contract_address: Option<String>,
    r#override: Option<Override>,
}

impl TronTransfer {
    pub fn new(from: String, to: String, value: String, fee: Number, latest_block: LatestBlock, token: Option<String>, contract_address: Option<String>, r#override: Option<Override>) -> Self { Self { from, to, value, fee, latest_block, token, contract_address, r#override } }
}

fn format_address(address_bytes: Vec<u8>) -> String {
    bs58::encode(address_bytes).with_check().into_string()
}

pub fn raw_to_json(sign_data_bytes: Vec<u8>, token_info: &str) -> Result<Vec<u8>, &str> {
    let raw_data = match transaction::Raw::parse_from_bytes(&sign_data_bytes) {
        Ok(v) => v,
        Err(_) => {
            return Err("sign data is invalid");
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
                    Ok(json.as_bytes().to_vec())
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
                    Ok(json.as_bytes().to_vec())
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
                    Ok(json.as_bytes().to_vec())
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
