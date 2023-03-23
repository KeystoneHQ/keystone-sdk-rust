use anyhow::Error;
use anyhow::format_err;
use hex;
use serde_json::json;
use ur_registry::crypto_psbt::CryptoPSBT;
use ur_registry::traits::From;
use ur_registry::traits::To;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneSDK_parseCryptoPSBT
	fn parse_crypto_psbt(
		cbor_hex: &str
	) -> String {
        let parse = || -> Result<String, Error> {
            let cbor = hex::decode(cbor_hex.to_string())?;
            let res = serde_cbor::from_slice(cbor.as_slice())?;
            let psbt = CryptoPSBT::from_cbor(res).map_err(|_| format_err!(""))?;
            let psbt_hex = hex::encode(psbt.get_psbt());
            Ok(psbt_hex)
        };
        match parse() {
            Ok(v) => json!({
                "psbt": v,
            }).to_string(),
            Err(_) => json!({"error": "PSBT is invalid"}).to_string(),
        }
    }

    @Java_com_keystone_sdk_KeystoneSDK_generateCryptoPSBT
    fn generate_crypto_psbt(psbt_hex: &str) -> String {
        let gen = || -> Result<String, Error> {
            let psbt = hex::decode(psbt_hex.to_string())?;
            let crypto_psbt = CryptoPSBT::new(psbt);
            let cbor_hex = hex::encode(crypto_psbt.to_bytes());
            Ok(cbor_hex)
        };
        match gen() {
            Ok(v) => json!({
                "type": "crypto-psbt",
                "cbor": v,
            }).to_string(),
            Err(_) => json!({"error": "PSBT is invalid"}).to_string(),
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_crypto_psbt() {
        let cbor_hex = "58A770736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000";
        let expect_result = "{\"psbt\":\"70736274ff01009a020000000258e87a21b56daf0c23be8e7070456c336f7cbaa5c8757924f545887bb2abdd750000000000ffffffff838d0427d0ec650a68aa46bb0b098aea4422c071b2ca78352a077959d07cea1d0100000000ffffffff0270aaf00800000000160014d85c2b71d0060b09c9886aeb815e50991dda124d00e1f5050000000016001400aea9a2e5f0f876a588df5546e8742d1d87008f000000000000000000\"}";

        assert_eq!(expect_result, parse_crypto_psbt(cbor_hex));
    }

    #[test]
    fn test_parse_crypto_psbt_error() {
        let cbor_hex = "a201";
        let expect_result = "{\"error\":\"PSBT is invalid\"}";

        assert_eq!(expect_result, parse_crypto_psbt(cbor_hex));
    }

    #[test]
    fn test_generate_crypto_psbt() {
        let psbt_hex = "70736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000";
        let expect_result = "{\"cbor\":\"58a770736274ff01009a020000000258e87a21b56daf0c23be8e7070456c336f7cbaa5c8757924f545887bb2abdd750000000000ffffffff838d0427d0ec650a68aa46bb0b098aea4422c071b2ca78352a077959d07cea1d0100000000ffffffff0270aaf00800000000160014d85c2b71d0060b09c9886aeb815e50991dda124d00e1f5050000000016001400aea9a2e5f0f876a588df5546e8742d1d87008f000000000000000000\",\"type\":\"crypto-psbt\"}";

        assert_eq!(expect_result, generate_crypto_psbt(psbt_hex))
    }

    #[test]
    fn test_generate_crypto_psbt_error() {
        let psbt_hex = "707";
        let expect_result = "{\"error\":\"PSBT is invalid\"}";

        assert_eq!(expect_result, generate_crypto_psbt(psbt_hex))
    }
}
