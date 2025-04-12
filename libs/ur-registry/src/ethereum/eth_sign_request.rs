use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, ETH_SIGN_REQUEST, UUID};
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
const DATA_TYPE: u8 = 3;
const CHAIN_ID: u8 = 4;
const DERIVATION_PATH: u8 = 5;
const ADDRESS: u8 = 6;
const ORIGIN: u8 = 7;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DataType {
    #[default]
    Transaction = 1,
    TypedData = 2,
    PersonalMessage = 3,
    TypedTransaction = 4,
    Auth7702 = 5
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Transaction),
            2 => Ok(DataType::TypedData),
            3 => Ok(DataType::PersonalMessage),
            4 => Ok(DataType::TypedTransaction),
            5 => Ok(DataType::Auth7702),
            x => Err(format!(
                "invalid value for data_type in eth-sign-request, expected (1, 2, 3, 4, 5), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct EthSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    data_type: DataType,
    chain_id: Option<i128>,
    derivation_path: CryptoKeyPath,
    address: Option<Bytes>,
    origin: Option<String>,
}

impl EthSignRequest {
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

    pub fn set_chain_id(&mut self, chain_id: i128) {
        self.chain_id = Some(chain_id)
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
        chain_id: Option<i128>,
        derivation_path: CryptoKeyPath,
        address: Option<Bytes>,
        origin: Option<String>,
    ) -> EthSignRequest {
        EthSignRequest {
            request_id,
            sign_data,
            data_type,
            chain_id,
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
    pub fn get_chain_id(&self) -> Option<i128> {
        self.chain_id
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
        if self.chain_id.is_some() {
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

impl RegistryItem for EthSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for EthSignRequest {
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

        if let Some(chain_id) = self.chain_id {
            e.int(Int::from(CHAIN_ID))?.int(
                Int::try_from(chain_id)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;
        }

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

impl<'b, C> minicbor::Decode<'b, C> for EthSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = EthSignRequest::default();
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
                DATA_TYPE => {
                    obj.data_type = DataType::from_u32(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(minicbor::decode::Error::message)?;
                }
                CHAIN_ID => {
                    obj.chain_id = Some(i128::from(d.int()?));
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

impl To for EthSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<EthSignRequest> for EthSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<EthSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::ethereum::eth_sign_request::{DataType, EthSignRequest};
    use crate::traits::RegistryItem;
    use crate::traits::{From as FromCbor, To};
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(1), true).unwrap();
        let path3 = PathComponent::new(Some(1), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(Some(1), false).unwrap();

        let source_fingerprint: [u8; 4] = [18, 52, 86, 120];
        let components = vec![path1, path2, path3, path4, path5];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );
        let sign_data = [
            248, 73, 128, 134, 9, 24, 78, 114, 160, 0, 130, 39, 16, 148, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 128, 164, 127, 116, 101, 115, 116, 50, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 96, 0, 87, 128, 128,
            128,
        ]
        .to_vec();
        let eth_sign_request = EthSignRequest::new(
            request_id,
            sign_data,
            DataType::Transaction,
            Some(1),
            crypto_key_path,
            None,
            Some("metamask".to_string()),
        );
        assert_eq!(
            "a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584bf849808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000578080800301040105d90130a2018a182cf501f501f500f401f4021a1234567807686d6574616d61736b",
            hex::encode(eth_sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584bf849808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000578080800301040105d90130a2018a182cf501f501f500f401f4021a1234567807686d6574616d61736b",
        )
            .unwrap();
        let eth_sign_request = EthSignRequest::from_cbor(bytes).unwrap();
        assert_eq!(
            "44'/1'/1'/0/1",
            eth_sign_request.get_derivation_path().get_path().unwrap()
        );
        assert_eq!(DataType::Transaction, eth_sign_request.get_data_type());
    }

    #[test]
    fn test_avax_c_chain_encode() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(60), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(Some(6), false).unwrap();

        let source_fingerprint: [u8; 4] = [0xbd, 0xee, 0xe7, 0x82];
        let components = vec![path1, path2, path3, path4, path5];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );

        let sign_data = hex::decode("02f87482a86901841dcd6500849502f9008252089446a836a6d5800dd3ab9a6b914c904ef8017b48c8880dcac353ec227a0080c001a03cebc64b4bd58567b7205897f1f68922c3f142366b3236fba169bea5ab875284a05291dae91b105ac2c0dc5479ecf1ed7890d93c2ab1e12695f1e8ecbc92a42e5a").unwrap();
        let eth_sign_request = EthSignRequest::new(
            request_id,
            sign_data,
            DataType::TypedTransaction,
            Some(43113),
            crypto_key_path,
            None,
            Some("core wallet".to_string()),
        );
        let data = hex::decode(hex::encode(eth_sign_request.to_bytes().unwrap())).unwrap();
        let ur = ur::encode(&data, EthSignRequest::get_registry_type().get_type());
        assert_eq!(ur, "ur:eth-sign-request/oladtpdagdndcawmgtfrkigrpmndutdnbtkgfssbjnaohdktaoyajylfpdinadlrcasnihaelrmdaoytaelfgmaymwfgpdenoltllabttepynyjemegsmhglyaadkgfdsplobtsgsrguwpcpknaelartadnbfnwmswgrgrtllpiorlcxhdmswnynldcpsrwnfwenjeeyenzooyinrnonpyltgmlrnbgmmetnwlcwbehtsartuoghkkwpwnweksmhtafndrpavydsmdwnvswprfmooxdmhtaxaaaacfpdinahtaaddyoeadlecsdwykcsfnykaeykaewkamwkaocyrywyvdlfatjeiajljpihcxkthsjzjzihjyfwkouyfp");
    }
}
