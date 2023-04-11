use serde::{Serialize, Deserialize};
use serde_json::Number;

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
