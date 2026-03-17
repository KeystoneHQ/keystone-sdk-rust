use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, TRON_SIGN_REQUEST, UUID};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use alloc::format;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DATA_TYPE: u8 = 3;
const DERIVATION_PATH: u8 = 4;
const ADDRESS: u8 = 5;
const ORIGIN: u8 = 6;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DataType {
    #[default]
    Transaction = 1,
    PersonalMessage = 2,
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Transaction),
            2 => Ok(DataType::PersonalMessage),
            x => Err(format!(
                "invalid value for data_type in tron-sign-request, expected (1, 2), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct TronSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    data_type: DataType,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>,
}

impl TronSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = data_type
    }

    pub fn set_derivation_path(&mut self, derivation_path: CryptoKeyPath) {
        self.derivation_path = derivation_path;
    }

    pub fn set_address(&mut self, address: Bytes) {
        self.address = Some(address)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        request_id: Option<Bytes>,
        sign_data: Bytes,
        data_type: DataType,
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
    ) -> TronSignRequest {
        TronSignRequest {
            request_id,
            sign_data,
            data_type,
            derivation_path,
            address,
            origin,
        }
    }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }
    pub fn get_address(&self) -> Option<Bytes> {
        self.address.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 3;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.address.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for TronSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        TRON_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for TronSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;
        e.int(Int::from(DATA_TYPE))?
            .int(Int::from(self.data_type.clone() as u8))?;

        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(address) = &self.address {
            e.int(Int::from(ADDRESS))?.bytes(address)?;
        }

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for TronSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = TronSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Err(minicbor::decode::Error::message("UUID tag is invalid"));
                    }
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                DATA_TYPE => {
                    obj.data_type = DataType::from_u32(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(minicbor::decode::Error::message)?;
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, _ctx)?;
                }
                ADDRESS => {
                    obj.address = Some(d.bytes()?.to_vec());
                }
                ORIGIN => {
                    obj.origin = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for TronSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<TronSignRequest> for TronSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<TronSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl TryFrom<Vec<u8>> for TronSignRequest {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        Self::from_cbor(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::traits::{From as FromCbor, To};
    use crate::tron::tron_sign_request::TronSignRequest;
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;
    extern crate std; // import std
    use std::println;

    #[test]
    fn test_encode_binary() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(195), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(Some(0), false).unwrap();

        let source_fingerprint: [u8; 4] = [18, 18, 18, 18];
        let components = vec![path1, path2, path3, path4, path5];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some(
            Vec::from_hex("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()
        );

        let sign_data = Vec::from_hex("0a0207902208e1b9de559665c6714080c49789bb2c5aae01081f12a9010a31747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e54726967676572536d617274436f6e747261637412740a1541c79f045e4d48ad8dae00e6a6714dae1e000adfcd1215410d292c98a5eca06c2085fff993996423cf66c93b2244a9059cbb0000000000000000000000009bbce520d984c3b95ad10cb4e32a9294e6338da300000000000000000000000000000000000000000000000000000000000f424070c0b6e087bb2c90018094ebdc03").unwrap();

        let tron_sign_request = TronSignRequest::new(
            request_id,
            sign_data,
            DataType::Transaction,
            crypto_key_path,
            None, 
            Some("tron wallet".to_string()),
        );

        let encoded = tron_sign_request.to_bytes().unwrap();
        println!("Encoded CBOR Hex: {}", hex::encode(&encoded));
        
        let decoded = TronSignRequest::from_cbor(encoded).unwrap();
        assert_eq!(decoded.sign_data, tron_sign_request.sign_data);
    }

    #[test]
    fn test_decode_binary() {
        let hex_str = "a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258d40a0207902208e1b9de559665c6714080c49789bb2c5aae01081f12a9010a31747970652e676f6f676c65617069732e636f6d2f70726f746f636f6c2e54726967676572536d617274436f6e747261637412740a1541c79f045e4d48ad8dae00e6a6714dae1e000adfcd1215410d292c98a5eca06c2085fff993996423cf66c93b2244a9059cbb0000000000000000000000009bbce520d984c3b95ad10cb4e32a9294e6338da300000000000000000000000000000000000000000000000000000000000f424070c0b6e087bb2c90018094ebdc03030104d90130a2018a182cf518c3f500f500f400f4021a12121212066b74726f6e2077616c6c6574";
        let bytes = Vec::from_hex(hex_str).unwrap();
        
        let decoded = TronSignRequest::from_cbor(bytes).unwrap();
        
        assert_eq!(decoded.origin, Some("tron wallet".to_string()));
        assert_eq!(decoded.sign_data[0], 0x0A);
        assert_eq!(DataType::Transaction, decoded.get_data_type());
    }
}
