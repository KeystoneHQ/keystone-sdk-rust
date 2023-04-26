use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{CRYPTO_KEYPATH, RegistryType, UUID, NEAR_SIGN_REQUEST};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DERIVATION_PATH: u8 = 3;
const ACCOUNT: u8 = 4;
const ORIGIN: u8 = 5;

#[derive(Clone, Debug, Default)]
pub struct NearSignRequest {
    request_id: Option<Bytes>,
    sign_data: Vec<Bytes>,
    derivation_path: CryptoKeyPath,
    account: Option<Bytes>,
    origin: Option<String>,
}

impl NearSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Vec<Bytes>) {
        self.sign_data = data;
    }

    pub fn set_derivation_path(&mut self, derivation_path: CryptoKeyPath) {
        self.derivation_path = derivation_path;
    }

    pub fn set_account(&mut self, account: Bytes) {
        self.account = Some(account)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        request_id: Option<Bytes>,
        sign_data: Vec<Bytes>,
        derivation_path: CryptoKeyPath,
        account: Option<Bytes>,
        origin: Option<String>,
    ) -> NearSignRequest {
        NearSignRequest {
            request_id,
            sign_data,
            derivation_path,
            account,
            origin,
        }
    }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Vec<Bytes> {
        self.sign_data.clone()
    }
    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }
    pub fn get_account(&self) -> Option<Bytes> {
        self.account.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 2;
        if self.request_id.is_some() {
            size = size + 1;
        }
        if self.account.is_some() {
            size = size + 1;
        }
        if self.origin.is_some() {
            size = size + 1;
        }
        size
    }
}

impl RegistryItem for NearSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        NEAR_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for NearSignRequest {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?;
        let sign_data_len = self.sign_data.len().try_into().unwrap();
        e.array(sign_data_len)?;
        for ele in &self.sign_data {
            e.bytes(&ele)?;
        }

        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(account) = &self.account {
            e.int(Int::from(ACCOUNT))?
                .bytes(account)?;
        }

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?
                .str(origin)?;
        }

        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for NearSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = NearSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGN_DATA => {
                    let sign_data_len = d.array()?;
                    obj.sign_data = Vec::new();
                    if sign_data_len.is_some() {
                        for _ in 0..sign_data_len.unwrap() {
                            obj.sign_data.push(d.bytes()?.to_vec());
                        }
                    }
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, _ctx)?;
                }
                ACCOUNT => {
                    obj.account = Some(d.bytes()?.to_vec());
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


impl To for NearSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<NearSignRequest> for NearSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<NearSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::near::near_sign_request::NearSignRequest;
    use crate::traits::{To, From};

    #[test]
    fn test_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(397), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();

        let source_fingerprint: [u8; 4] = [242, 63, 159, 210];
        let components = vec![path1, path2, path3];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap());
        let sign_data = vec![
            hex::decode("4000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009FCC0720A016D3C1E849D86B16D7139E043EFC48ADD1C78F39C3D2F00EE98C07823E0CA1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037F0787E1CB1C22A1C63C24A37E4C6C656DD3CB049E6B7C17F75D01F0859EFB7D80100000003000000A1EDCCCE1BC2D3000000000000").unwrap(),
        ];
        let sign_request = NearSignRequest::new(
            request_id, sign_data, crypto_key_path, None, Some("nearwallet".to_string())
        );

        assert_eq!(
            "a401d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d028158e64000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009fcc0720a016d3c1e849d86b16d7139e043efc48add1c78f39c3d2f00ee98c07823e0ca1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037f0787e1cb1c22a1c63c24a37e4c6c656dd3cb049e6b7c17f75d01f0859efb7d80100000003000000a1edccce1bc2d300000000000003d90130a20186182cf519018df500f5021af23f9fd2056a6e65617277616c6c6574",
            hex::encode(sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a401d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d028158e64000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009fcc0720a016d3c1e849d86b16d7139e043efc48add1c78f39c3d2f00ee98c07823e0ca1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037f0787e1cb1c22a1c63c24a37e4c6c656dd3cb049e6b7c17f75d01f0859efb7d80100000003000000a1edccce1bc2d300000000000003d90130a20186182cf519018df500f5021af23f9fd2056a6e65617277616c6c6574",
        )
            .unwrap();
        let sign_request = NearSignRequest::from_cbor(bytes).unwrap();
        let request_id = Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap());
        let sign_data = vec![
            hex::decode("4000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037009FCC0720A016D3C1E849D86B16D7139E043EFC48ADD1C78F39C3D2F00EE98C07823E0CA1957100004000000039666363303732306130313664336331653834396438366231366437313339653034336566633438616464316337386633396333643266303065653938633037F0787E1CB1C22A1C63C24A37E4C6C656DD3CB049E6B7C17F75D01F0859EFB7D80100000003000000A1EDCCCE1BC2D3000000000000").unwrap(),
        ];

        assert_eq!("44'/397'/0'", sign_request.derivation_path.get_path().unwrap());
        assert_eq!(request_id, sign_request.get_request_id());
        assert_eq!(sign_data, sign_request.get_sign_data());
    }
}
