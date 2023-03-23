use serde_cbor::{Value, from_slice, to_vec};

use crate::{traits::{RegistryItem, To, From}, registry_types::{RegistryType, CRYPTO_PSBT}, cbor_value::CborValue};

#[derive(Clone, Debug, Default)]
pub struct CryptoPSBT {
    psbt: Vec<u8>,
}

impl RegistryItem for CryptoPSBT {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_PSBT
    }
}

impl CryptoPSBT {
    pub fn new(psbt: Vec<u8>) -> Self {
        CryptoPSBT { psbt }
    }

    pub fn get_psbt(&self) -> Vec<u8> {
        self.psbt.clone()
    }
}

impl To for CryptoPSBT {
    fn to_cbor(&self) -> Value {
        Value::Bytes(self.psbt.clone())
    }
    fn to_bytes(&self) -> Vec<u8> {
        let value = self.to_cbor();
        to_vec(&value).unwrap()
    }
}

impl From<CryptoPSBT> for CryptoPSBT {
    fn from_cbor(cbor: Value) -> Result<CryptoPSBT, String> {
        Ok(CryptoPSBT {
            psbt: CborValue::new(cbor).get_bytes().unwrap()
        })
    }

    fn from_bytes(bytes: Vec<u8>) -> Result<CryptoPSBT, String> {
        let value: Value = from_slice(bytes.as_slice()).unwrap();
        CryptoPSBT::from_cbor(value)
    }
}

#[cfg(test)]
mod tests {
    use hex::FromHex;
    use crate::traits::{To, From};

    use super::CryptoPSBT;

    #[test]
    fn test_encode() {
        let data = Vec::from_hex("70736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000").unwrap();
        let psbt = CryptoPSBT { psbt: data };
        let cbor = psbt.to_cbor();
        let cbor_bytes = serde_cbor::to_vec(&cbor).unwrap();
        assert_eq!(
            "58A770736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000",
            hex::encode(cbor_bytes).to_uppercase()
        );
    }

    #[test]
    fn test_decode() {
        let cbor_bytes = Vec::from_hex("58A770736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000").unwrap();
        let psbt = CryptoPSBT::from_bytes(cbor_bytes).unwrap();
        let data = psbt.get_psbt();
        assert_eq!(
            "70736274FF01009A020000000258E87A21B56DAF0C23BE8E7070456C336F7CBAA5C8757924F545887BB2ABDD750000000000FFFFFFFF838D0427D0EC650A68AA46BB0B098AEA4422C071B2CA78352A077959D07CEA1D0100000000FFFFFFFF0270AAF00800000000160014D85C2B71D0060B09C9886AEB815E50991DDA124D00E1F5050000000016001400AEA9A2E5F0F876A588DF5546E8742D1D87008F000000000000000000",
            hex::encode(data).to_uppercase()
        )
    }
}
