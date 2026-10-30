use anyhow::{anyhow, ensure, Result};
use serde_json::json;
use ur_registry::crypto_key_path::{CryptoKeyPath, PathComponent};
use ur_registry::traits::{From, To};
use ur_registry::tron::{tron_sign_request::{TronSignRequest, DataType}, tron_signature::TronSignature};
use crate::export;

fn request(request_id: &str, sign_data: &str, path: &str, xfp: &str, data_type: i32, address: &str, origin: &str) -> Result<Vec<u8>> {
    let sign_data = sign_data.strip_prefix("0x").or_else(|| sign_data.strip_prefix("0X")).unwrap_or(sign_data);
    ensure!(!sign_data.is_empty() && sign_data.len() <= 65536 && xfp.len() == 8 && !path.is_empty() && path.len() <= 256 && origin.len() <= 128, "invalid request");
    let id = uuid::Uuid::parse_str(request_id)?.as_bytes().to_vec();
    let data = hex::decode(sign_data)?;
    let fingerprint: [u8; 4] = hex::decode(xfp)?.try_into().map_err(|_| anyhow!("invalid fingerprint"))?;
    let path = path.strip_prefix("m/").unwrap_or(path);
    let mut components = Vec::new();
    for part in path.split('/') {
        let hardened = part.ends_with("'");
        let number = part.strip_suffix("'").unwrap_or(part);
        ensure!(!number.is_empty() && number.bytes().all(|b| b.is_ascii_digit()), "invalid path");
        components.push(PathComponent::new(Some(number.parse::<u32>()?), hardened).map_err(|_| anyhow!("invalid path"))?);
    }
    let address = if address.is_empty() { None } else {
        let bytes = bs58::decode(address).with_check(None).into_vec()?;
        ensure!(bytes.len() == 21 && bytes[0] == 0x41, "invalid address");
        Some(bytes)
    };
    let kind = DataType::from_u32(data_type as u32).map_err(|_| anyhow!("invalid data type"))?;
    Ok(TronSignRequest::new(Some(id), data, kind, CryptoKeyPath::new(components, Some(fingerprint), None), address, if origin.is_empty() { None } else { Some(origin.to_owned()) }).to_bytes()?)
}

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_generateTronNativeSignRequest
    fn generate_tron_native_sign_request(request_id: &str, sign_data: &str, path: &str, xfp: &str, data_type: i32, address: &str, origin: &str) -> String {
        match request(request_id, sign_data, path, xfp, data_type, address, origin) {
            Ok(cbor) => json!({"type":"tron-sign-request", "cbor":hex::encode(cbor)}).to_string(),
            Err(_) => json!({"error":"Invalid TRON native sign request"}).to_string(),
        }
    }
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseTronNativeSignature
    fn parse_tron_native_signature(ur_type: &str, cbor_hex: &str) -> String {
        let parse = || -> Result<serde_json::Value> {
            ensure!(ur_type == "tron-signature" && !cbor_hex.is_empty() && cbor_hex.len() <= 256, "invalid response");
            let bytes = hex::decode(cbor_hex)?;
            let signature = TronSignature::from_cbor(bytes.clone())?;
            let id = signature.get_request_id().ok_or_else(|| anyhow!("missing request id"))?;
            ensure!(id.len() == 16 && signature.get_signature().len() == 65 && signature.to_bytes()? == bytes, "invalid response");
            Ok(json!({"request_id":uuid::Uuid::from_slice(&id)?.to_string(), "signature":hex::encode(signature.get_signature())}))
        };
        match parse() {
            Ok(value) => value.to_string(),
            Err(_) => json!({"error":"Invalid TRON native signature"}).to_string(),
        }
    }
}
