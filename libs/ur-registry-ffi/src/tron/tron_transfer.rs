use serde::{Serialize, Deserialize};
use serde_json::Number;

#[derive(Serialize, Deserialize)]
struct LatestBlock {
    hash: String,
    number: Number,
    timestamp: Number,
}

#[derive(Serialize, Deserialize)]
struct Override {
    #[serde(rename = "tokenShortName")]
    token_short_name: String,
    #[serde(rename = "tokenFullName")]
    token_full_name: String,
    decimals: Number,
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
