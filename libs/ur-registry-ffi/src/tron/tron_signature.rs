use anyhow::{ensure, anyhow, Result};
use no_std_io2::io::Read;
use prost::Message as ProstMessage;
use protobuf::Message;
use serde_json::json;
use sha2::{Digest, Sha256};
use ur_registry::keystone::keystone_sign_result::KeystoneSignResult;
use ur_registry::pb::protoc::{Base, payload::Content};
use ur_registry::traits::{From, To};
use crate::export;
use crate::tron::types::{tron::Transaction, contract::{TransferContract, TransferAssetContract, TriggerSmartContract}};

const MAX_RESULT_BYTES: usize = 128 * 1024;
const MAX_TRANSACTION_BYTES: usize = 32 * 1024;

fn known<T: Message>(message: &T) -> Result<()> {
    ensure!(message.special_fields().unknown_fields().iter().next().is_none(), "invalid transaction");
    Ok(())
}
fn canonical<T: Message>(message: &T, bytes: &[u8]) -> Result<()> {
    known(message)?;
    ensure!(message.write_to_bytes()? == bytes, "invalid transaction");
    Ok(())
}
fn address(bytes: &[u8]) -> Result<()> {
    ensure!(bytes.len() == 21 && bytes[0] == 0x41, "invalid address");
    Ok(())
}
fn validated(cbor_hex: &str) -> Result<serde_json::Value> {
    ensure!(!cbor_hex.is_empty() && cbor_hex.len() <= MAX_RESULT_BYTES * 2, "invalid result");
    let cbor = hex::decode(cbor_hex)?;
    let wrapped = KeystoneSignResult::from_cbor(cbor.clone())?;
    ensure!(wrapped.to_bytes()? == cbor, "invalid result");
    let compressed = wrapped.get_sign_result();
    let decoder = libflate::gzip::Decoder::new(compressed.as_slice())?;
    let mut decoded = Vec::new();
    decoder.take((MAX_RESULT_BYTES + 1) as u64).read_to_end(&mut decoded)?;
    ensure!(!decoded.is_empty() && decoded.len() <= MAX_RESULT_BYTES, "invalid result");
    let base = Base::decode(decoded.as_slice())?;
    ensure!(base.encode_to_vec() == decoded, "invalid result");
    let payload = base.data.ok_or_else(|| anyhow!("invalid result"))?;
    ensure!(payload.r#type == 9, "invalid result");
    let Some(Content::SignTxResult(result)) = payload.content else { return Err(anyhow!("invalid result")); };
    ensure!(result.sign_id.len() == 36 && uuid::Uuid::parse_str(&result.sign_id)?.hyphenated().to_string().eq_ignore_ascii_case(&result.sign_id), "invalid result");
    ensure!(result.tx_id.len() == 64 && hex::decode(&result.tx_id)?.len() == 32, "invalid result");
    ensure!(!result.raw_tx.is_empty() && result.raw_tx.len() <= MAX_TRANSACTION_BYTES * 2, "invalid result");
    let bytes = hex::decode(&result.raw_tx)?;
    let tx = Transaction::parse_from_bytes(&bytes)?;
    canonical(&tx, &bytes)?;
    ensure!(tx.signature.len() == 1 && tx.signature[0].len() == 65, "invalid signature");
    let raw = tx.raw_data.as_ref().ok_or_else(|| anyhow!("invalid transaction"))?;
    known(raw)?;
    ensure!(raw.ref_block_bytes.len() == 2 && raw.ref_block_hash.len() == 8 && raw.timestamp > 0 && raw.expiration > raw.timestamp && raw.fee_limit >= 0 && raw.auths.is_empty() && raw.scripts.is_empty() && raw.contract.len() == 1, "invalid transaction");
    let contract = &raw.contract[0];
    known(contract)?;
    ensure!(contract.provider.is_empty() && contract.ContractName.is_empty() && contract.Permission_id >= 0, "invalid contract");
    let parameter = contract.parameter.as_ref().ok_or_else(|| anyhow!("invalid contract"))?;
    known(parameter)?;
    match contract.type_.value() {
        1 => {
            ensure!(parameter.type_url == "type.googleapis.com/protocol.TransferContract", "invalid contract");
            let value = TransferContract::parse_from_bytes(&parameter.value)?;
            canonical(&value, &parameter.value)?;
            address(&value.owner_address)?; address(&value.to_address)?;
            ensure!(value.amount > 0, "invalid amount");
        }
        2 => {
            ensure!(parameter.type_url == "type.googleapis.com/protocol.TransferAssetContract", "invalid contract");
            let value = TransferAssetContract::parse_from_bytes(&parameter.value)?;
            canonical(&value, &parameter.value)?;
            address(&value.owner_address)?; address(&value.to_address)?;
            ensure!(value.amount > 0 && !value.asset_name.is_empty(), "invalid amount");
        }
        31 => {
            ensure!(parameter.type_url == "type.googleapis.com/protocol.TriggerSmartContract", "invalid contract");
            let value = TriggerSmartContract::parse_from_bytes(&parameter.value)?;
            canonical(&value, &parameter.value)?;
            address(&value.owner_address)?; address(&value.contract_address)?;
            ensure!(!value.data.is_empty() && value.call_value >= 0 && value.call_token_value >= 0 && value.token_id >= 0, "invalid contract");
        }
        _ => return Err(anyhow!("invalid contract")),
    }
    let tx_id = hex::encode(Sha256::digest(raw.write_to_bytes()?));
    ensure!(result.tx_id.eq_ignore_ascii_case(&tx_id), "invalid transaction id");
    Ok(json!({"request_id": result.sign_id, "raw": hex::encode(tx.write_to_bytes()?), "tx_id": tx_id}))
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseTronSignature
    fn parse_tron_signature(ur_type: &str, cbor_hex: &str) -> String {
        if ur_type != "keystone-sign-result" { return json!({"error":"type not match"}).to_string(); }
        match validated(cbor_hex) {
            Ok(value) => value.to_string(),
            Err(_) => json!({"error":"signature is invalid"}).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_tron_signature() {
        let tron_signature_cbor = "a1015901bd1f8b08000000000000008d523d8e94310cd54aac342024a429b7a2a0a019c9761c3ba6a24053738538761a24462c349c85821b5071324e80bf866e259c2acf3fc97bcfa7bbf3ab4ff9fdebb7dbe77cfde571dd221f7edd9f9e9f4f576a57bb7ea0773fef5fbc31c748e7b834d7b8b0cfb898475cc821d45b2c9738bf4708696e0b1612766308304cd71d996da5ae4d2e991464440b779f16da1c3a590eebfcf0fb19cc60409840a006443012dd227b3791be449161c062d361eeb4fa9c59f50337d2b4a3b3a1b29a82744a51d975549674c1c24c5ba1adb0a0ada0245bb96e07b2283b176255afd2957a93a82e52e6239f7a64519a321606133be3889d0b5724ee9469b9da98284b436a7acb563c46c70587205cfa14f36563f65c13eac54aeebdcd9a9930b5b545963527622e36dd963b3c112533a96df498b6577da746a1f7b61a88000de1de5a6a7baaff3f623331282c28d760e8a1b701e000e3f458d09018990edad967efe527378d12aa1c09d8f51b825d245bcf42dc04dd8753a9422327e8e251aa90501fd29a61b0899197db0344b725c0613cd7aa64d916633a1869bd85735755772df7dffef873472fff6df1c7c7db5f8509bb54d6020000";
        let expect_result = "{\"raw\":\"0ad4010a0207902208e1b9de559665c6714080c49789bb2c5aae01081f12a9010a31747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e54726967676572536d617274436f6e747261637412740a15418dfec1cde1fe6a9ec38a16c7d67073e3020851c01215410d292c98a5eca06c2085fff993996423cf66c93b2244a9059cbb0000000000000000000000009c0279f1bda9fc40a85f1b53c306602864533e7300000000000000000000000000000000000000000000000000000000000f424070c0b6e087bb2c90018094ebdc03124142a9ece5a555a9437de74108d0fb5320f20835e108b961bb8b230228ea07c485412625863391d49692be558067f9e00559641f5ee63d8ab09275a51afe555b7e01\",\"request_id\":\"9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d\",\"tx_id\":\"10d63b9c0c1215940d091eb7fdee3ce7cf2b6ee2d2922c1f5a9d73b0529e8954\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("keystone-sign-result", tron_signature_cbor)
        );
    }

    #[test]
    fn test_parse_tron_signature_type_error() {
        let tron_signature_cbor = "a1015901bd1f8b08000000000000008d523d8e94310cd54aac342024a429b7a2a0a019c9761c3ba6a24053738538761a24462c349c85821b5071324e80bf866e259c2acf3fc97bcfa7bbf3ab4ff9fdebb7dbe77cfde571dd221f7edd9f9e9f4f576a57bb7ea0773fef5fbc31c748e7b834d7b8b0cfb898475cc821d45b2c9738bf4708696e0b1612766308304cd71d996da5ae4d2e991464440b779f16da1c3a590eebfcf0fb19cc60409840a006443012dd227b3791be449161c062d361eeb4fa9c59f50337d2b4a3b3a1b29a82744a51d975549674c1c24c5ba1adb0a0ada0245bb96e07b2283b176255afd2957a93a82e52e6239f7a64519a321606133be3889d0b5724ee9469b9da98284b436a7acb563c46c70587205cfa14f36563f65c13eac54aeebdcd9a9930b5b545963527622e36dd963b3c112533a96df498b6577da746a1f7b61a88000de1de5a6a7baaff3f623331282c28d760e8a1b701e000e3f458d09018990edad967efe527378d12aa1c09d8f51b825d245bcf42dc04dd8753a9422327e8e251aa90501fd29a61b0899197db0344b725c0613cd7aa64d916633a1869bd85735755772df7dffef873472fff6df1c7c7db5f8509bb54d6020000";
        let expect_result = "{\"error\":\"type not match\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("eth-signature", tron_signature_cbor)
        );
    }

    #[test]
    fn test_parse_tron_signature_error() {
        let tron_signature_cbor = "a201";
        let expect_result = "{\"error\":\"signature is invalid\"}";

        assert_eq!(
            expect_result,
            parse_tron_signature("keystone-sign-result", tron_signature_cbor)
        );
    }
}
