use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, SOL_SIGN_REQUEST, UUID};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DERIVATION_PATH: u8 = 3;
const ADDRESS: u8 = 4;
const ORIGIN: u8 = 5;
const SIGN_TYPE: u8 = 6;

#[derive(Clone, Debug, PartialEq)]
pub enum SignType {
    Transaction = 1,
    Message,
}

impl Default for SignType {
    fn default() -> Self {
        SignType::Transaction
    }
}

impl SignType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(SignType::Transaction),
            2 => Ok(SignType::Message),
            x => Err(format!(
                "invalid value for sign_type in sol-sign-request, expected 1 or 2, received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct SolSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>,
    sign_type: SignType,
}

impl SolSignRequest {
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

    pub fn set_sign_type(&mut self, sign_type: SignType) {
        self.sign_type = sign_type
    }

    pub fn new(
        request_id: Option<Bytes>,
        sign_data: Bytes,
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
        sign_type: SignType,
    ) -> SolSignRequest {
        SolSignRequest {
            request_id,
            sign_data,
            derivation_path,
            address,
            origin,
            sign_type,
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
    pub fn get_sign_type(&self) -> SignType {
        self.sign_type.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 3;
        if let Some(_) = self.request_id {
            size = size + 1;
        }
        if let Some(_) = self.address {
            size = size + 1;
        }
        if let Some(_) = self.origin {
            size = size + 1;
        }
        size
    }
}

impl RegistryItem for SolSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        SOL_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for SolSignRequest {
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

        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(address) = &self.address {
            e.int(Int::from(ADDRESS))?.bytes(address)?;
        }

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        e.int(Int::from(SIGN_TYPE))?
            .int(Int::from(self.sign_type.clone() as u8))?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for SolSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = SolSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
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
                SIGN_TYPE => {
                    obj.sign_type = SignType::from_u32(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for SolSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<SolSignRequest> for SolSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<SolSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::solana::sol_sign_request::{SignType, SolSignRequest};
    use crate::traits::{From as FromCbor, To};
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(501), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), true).unwrap();

        let source_fingerprint: [u8; 4] = [18, 18, 18, 18];
        let components = vec![path1, path2, path3, path4];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );
        let sign_data = [
            1, 0, 1, 3, 200, 216, 66, 162, 241, 127, 215, 170, 182, 8, 206, 46, 165, 53, 166, 233,
            88, 223, 250, 32, 202, 246, 105, 179, 71, 185, 17, 196, 23, 25, 101, 83, 15, 149, 118,
            32, 178, 40, 186, 226, 185, 76, 130, 221, 212, 192, 147, 152, 58, 103, 54, 85, 85, 183,
            55, 236, 125, 220, 17, 23, 230, 28, 114, 224, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 41, 92, 194, 241, 243, 159,
            54, 4, 113, 132, 150, 234, 0, 103, 109, 106, 114, 236, 102, 173, 9, 217, 38, 227, 236,
            227, 79, 86, 95, 24, 210, 1, 2, 2, 0, 1, 12, 2, 0, 0, 0, 0, 225, 245, 5, 0, 0, 0, 0,
        ]
        .to_vec();
        let sol_sign_request = SolSignRequest::new(
            request_id,
            sign_data,
            crypto_key_path,
            None,
            Some("solflare".to_string()),
            SignType::Transaction,
        );
        assert_eq!(
            "a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02589601000103c8d842a2f17fd7aab608ce2ea535a6e958dffa20caf669b347b911c4171965530f957620b228bae2b94c82ddd4c093983a67365555b737ec7ddc1117e61c72e0000000000000000000000000000000000000000000000000000000000000000010295cc2f1f39f3604718496ea00676d6a72ec66ad09d926e3ece34f565f18d201020200010c0200000000e1f5050000000003d90130a20188182cf51901f5f500f500f5021a121212120568736f6c666c6172650601",
            hex::encode(sol_sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02589601000103c8d842a2f17fd7aab608ce2ea535a6e958dffa20caf669b347b911c4171965530f957620b228bae2b94c82ddd4c093983a67365555b737ec7ddc1117e61c72e0000000000000000000000000000000000000000000000000000000000000000010295cc2f1f39f3604718496ea00676d6a72ec66ad09d926e3ece34f565f18d201020200010c0200000000e1f5050000000003d90130a20188182cf51901f5f500f500f5021a121212120568736f6c666c6172650601",
        )
            .unwrap();
        let sol_sign_request = SolSignRequest::from_cbor(bytes).unwrap();
        assert_eq!(
            "44'/501'/0'/0'",
            sol_sign_request.derivation_path.get_path().unwrap()
        );
        assert_eq!(SignType::Transaction, sol_sign_request.get_sign_type());
    }
}
