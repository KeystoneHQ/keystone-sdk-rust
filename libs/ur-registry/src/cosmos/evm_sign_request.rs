use alloc::format;
use alloc::string::{String, ToString};
use core::convert::From;
use minicbor::data::{Int, Tag};

use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, UUID, EVM_SIGN_REQUEST, CRYPTO_KEYPATH};
use crate::types::Bytes;
use crate::traits::{MapSize, RegistryItem};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DATA_TYPE: u8 = 3;
const CUSTOM_CHAIN_IDENTIFIER: u8 = 4;
const DERIVATION_PATH: u8 = 5;
const ADDRESS: u8 = 6;
const ORIGIN: u8 = 7;

impl_template_struct!(EvmSignRequest {request_id: Bytes, sign_data: Bytes, data_type: SignDataType, custom_chain_identifier: u32, derivation_path: CryptoKeyPath, address: Option<Bytes>, origin: Option<String>});


#[derive(Clone, Debug, Default)]
pub enum SignDataType {
    #[default]
    Arbitrary = 1,
    CosmosAmino = 2,
    CosmosDirect = 3,
}

impl SignDataType {
    pub fn from_u8(value: u8) -> Result<Self, String> {
        match value {
            1 => Ok(SignDataType::Arbitrary),
            2 => Ok(SignDataType::CosmosAmino),
            3 => Ok(SignDataType::CosmosDirect),
            x => Err(format!(
                "invalid value for data_type in evm-sign-request, expected (1, 2, 3), received {:?}",
                x
            )),
        }
    }
}

impl RegistryItem for EvmSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        EVM_SIGN_REQUEST
    }
}

impl MapSize for EvmSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 5;
        if self.address.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for EvmSignRequest {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(REQUEST_ID))?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.request_id)?;

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;
        e.int(Int::from(DATA_TYPE))?
        .int(Int::from( self.data_type.clone() as u8))?;

        e.int(Int::from(CUSTOM_CHAIN_IDENTIFIER))?.u32(self.custom_chain_identifier)?;

        e.int(Int::from(DERIVATION_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, ctx)?;

        if let Some(address) = &self.address {
            e.int(Int::from(ADDRESS))?.bytes(address)?;
        }

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for EvmSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = EvmSignRequest::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                DATA_TYPE => {
                    obj.data_type = SignDataType::from_u8(
                        d.u8()
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                        .map_err(minicbor::decode::Error::message)?;
                }
                CUSTOM_CHAIN_IDENTIFIER => {
                    obj.custom_chain_identifier = d.u32()?
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, ctx)?;
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

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use hex::FromHex;
    use crate::crypto_key_path::PathComponent;
    use super::*;

    #[test]
    fn test_encode_evm_sign_request() {
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(9000), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();
        let path4 = PathComponent::new(Some(0), false).unwrap();
        let path5 = PathComponent::new(Some(0), false).unwrap();

        let source_fingerprint: [u8; 4] = [120, 35, 8, 4];
        let components = vec![path1, path2, path3, path4, path5];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let request_id = hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap();
        let sign_data = hex::decode("8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e").unwrap();
        let address = "evmos13nmjt4hru5ag0c6q3msk0srs55qd3dtme8wgep".as_bytes();
        let sign_request = EvmSignRequest::new(
            request_id,
            sign_data,
            SignDataType::CosmosAmino,
            9000,
            crypto_key_path,
            Some(address.to_vec()),
            Some("evm wallet".to_string()),
        );
        let result: Vec<u8> = sign_request.try_into().unwrap();
        assert_eq!(
            "a701d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258208e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e03020419232805d90130a2018a182cf5192328f500f500f400f4021a7823080406582c65766d6f7331336e6d6a743468727535616730633671336d736b30737273353571643364746d653877676570076a65766d2077616c6c6574",
            hex::encode(result)
        );
    }

    #[test]
    fn test_decode_evm_sign_request() {
        let bytes = Vec::from_hex(
            "a701d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258208e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e03020419232805d90130a2018a182cf5192328f500f500f400f4021a7823080406582c65766d6f7331336e6d6a743468727535616730633671336d736b30737273353571643364746d653877676570076a65766d2077616c6c6574",
        )
            .unwrap();
        let sign_request = EvmSignRequest::try_from(bytes).unwrap();
        let request_id = hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap();

        let sign_data = hex::decode("8e53e7b10656816de70824e3016fc1a277e77825e12825dc4f239f418ab2e04e").unwrap();
        let address = "evmos13nmjt4hru5ag0c6q3msk0srs55qd3dtme8wgep".as_bytes();
        assert_eq!(
            "44'/9000'/0'/0/0",
            sign_request.derivation_path.get_path().unwrap()
        );
        assert_eq!(request_id, sign_request.get_request_id());
        assert_eq!(9000, sign_request.get_custom_chain_identifier());
        assert_eq!(Some("evm wallet".to_string()), sign_request.get_origin());
        assert_eq!(sign_data, sign_request.get_sign_data());
        assert_eq!(Some(address.to_vec()), sign_request.get_address());
    }
}
