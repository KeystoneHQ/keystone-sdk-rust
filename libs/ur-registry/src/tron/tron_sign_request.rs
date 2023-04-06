use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{CRYPTO_KEYPATH, RegistryType, TRON_SIGN_REQUEST, UUID};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DERIVATION_PATH: u8 = 3;
const ADDRESS: u8 = 4;
const ORIGIN: u8 = 5;


#[derive(Clone, Debug, Default)]
pub struct TronSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>
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
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
    ) -> TronSignRequest {
        TronSignRequest {
            request_id,
            sign_data,
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
        if let Some(_) = self.address {
            size = size + 1;
        }
        if let Some(_) = self.origin {
            size = size + 1;
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
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?
            .bytes(&self.sign_data)?;

        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(address) = &self.address {
            e.int(Int::from(ADDRESS))?
                .bytes(address)?;
        }

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?
                .str(origin)?;
        }

        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for TronSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = TronSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
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

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, To};
    use hex::FromHex;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::tron::tron_sign_request::{TronSignRequest};

    #[test]
    fn test_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(195), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), true).unwrap();

        let source_fingerprint: [u8; 4] = [18, 18, 18, 18];
        let components = vec![path1, path2, path3, path4];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some([155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109].to_vec());
        let sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#.as_bytes().to_vec();
        let tron_sign_request = TronSignRequest::new(request_id, sign_data, crypto_key_path, None, Some("keystone".to_string()));
        assert_eq!(
            "a401d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a1212121205686b657973746f6e65",
            hex::encode(tron_sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a401d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025901557b22746f223a22544b43735874664b6648326436614561514363747962444339756141334d536a3268222c2266726f6d223a22545868745972386e6d6769537033645933635366694b426a6564337a4e3874654853222c2276616c7565223a2231222c22666565223a3130303030302c226c6174657374426c6f636b223a7b2268617368223a2236383836613736666361653637376533353433653534366134336164346535666336393230363533623536623731333534326530626636346530666638356365222c226e756d626572223a31363036383132362c2274696d657374616d70223a313537383435393639393030307d2c22746f6b656e223a2231303031303930222c226f76657272696465223a7b22746f6b656e53686f72744e616d65223a22544f4e45222c22746f6b656e46756c6c4e616d65223a2254726f6e4f6e65222c22646563696d616c73223a31387d7d03d90130a20188182cf518c3f500f500f5021a1212121205686b657973746f6e65",
        ).unwrap();
        let tron_sign_request = TronSignRequest::from_cbor(bytes).unwrap();

        let expected_sign_data = r#"{"to":"TKCsXtfKfH2d6aEaQCctybDC9uaA3MSj2h","from":"TXhtYr8nmgiSp3dY3cSfiKBjed3zN8teHS","value":"1","fee":100000,"latestBlock":{"hash":"6886a76fcae677e3543e546a43ad4e5fc6920653b56b713542e0bf64e0ff85ce","number":16068126,"timestamp":1578459699000},"token":"1001090","override":{"tokenShortName":"TONE","tokenFullName":"TronOne","decimals":18}}"#;
        assert_eq!("44'/195'/0'/0'", tron_sign_request.derivation_path.get_path().unwrap());
        assert_eq!(expected_sign_data, core::str::from_utf8(tron_sign_request.sign_data.as_slice()).unwrap_or_default());
    }
}