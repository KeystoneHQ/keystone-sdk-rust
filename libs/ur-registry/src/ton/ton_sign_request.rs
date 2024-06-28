use alloc::{
    format,
    string::{String, ToString},
};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::cbor_map,
    crypto_key_path::CryptoKeyPath,
    impl_template_struct,
    registry_types::{RegistryType, CRYPTO_KEYPATH, TON_SIGN_REQUEST, UUID},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DATA_TYPE: u8 = 3;
const DERIVATION_PATH: u8 = 4;
const ADDRESS: u8 = 5;
const ORIGIN: u8 = 6;

impl_template_struct!(TonSignRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    data_type: DataType,
    derivation_path: Option<CryptoKeyPath>,
    address: String,
    origin: Option<String>
});

#[derive(Clone, Debug, PartialEq, Default)]
pub enum DataType {
    #[default]
    Transaction = 1,
    SignProof = 2,
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Transaction),
            2 => Ok(DataType::SignProof),
            x => Err(format!(
                "invalid value for data_type in eth-sign-request, expected (1, 2, 3, 4), received {:?}",
                x
            )),
        }
    }
}

impl MapSize for TonSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 3;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.derivation_path.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for TonSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        TON_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for TonSignRequest {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        e.int(Int::from(DATA_TYPE))?
            .int(Int::from(self.data_type.clone() as u8))?;

        if let Some(derivation_path) = &self.derivation_path {
            e.int(Int::from(DERIVATION_PATH))?
                .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
            CryptoKeyPath::encode(derivation_path, e, _ctx)?;
        }

        e.int(Int::from(ADDRESS))?.str(&self.address)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for TonSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = TonSignRequest::default();
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
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = Some(CryptoKeyPath::decode(d, ctx)?);
                }
                ADDRESS => {
                    obj.address = d.str()?.to_string();
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
    use base64::Engine;

    use crate::crypto_key_path::PathComponent;

    use super::*;
    extern crate std;
    use std::println;

    #[test]
    fn test_encode() {
        let tx = "te6cckEBAgEARwABHCmpoxdmOz6lAAAACAADAQBoQgArFnMvHAX9tOjTp4/RDd3vP2Bn8xG+U5MTuKRKUE1NoqHc1lAAAAAAAAAAAAAAAAAAAHBy4G8=";
        let payload = base64::prelude::BASE64_STANDARD.decode(tx).unwrap();
        let sig = TonSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            sign_data: payload,
            data_type: DataType::Transaction,
            derivation_path: None,
            address: "UQC1IywyQwixSOU8pezOZDC9rv2xCV4CGJzOWH6RX8BTsGJx".to_string(),
            origin: Some("TonKeeper".to_string()),
        };
        let result: Vec<u8> = sig.try_into().unwrap();
        let expect_result = hex::decode("a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025856b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f0301057830555143314979777951776978534f553870657a4f5a4443397276327843563443474a7a4f574836525838425473474a780669546f6e4b6565706572").unwrap();

        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_decode() {
        let tx = "te6cckEBAgEARwABHCmpoxdmOz6lAAAACAADAQBoQgArFnMvHAX9tOjTp4/RDd3vP2Bn8xG+U5MTuKRKUE1NoqHc1lAAAAAAAAAAAAAAAAAAAHBy4G8=";
        let payload = base64::prelude::BASE64_STANDARD.decode(tx).unwrap();
        println!("{}", hex::encode(&payload));
        let expect_result = TonSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            sign_data: payload,
            data_type: DataType::Transaction,
            derivation_path: None,
            address: "UQC1IywyQwixSOU8pezOZDC9rv2xCV4CGJzOWH6RX8BTsGJx".to_string(),
            origin: Some("TonKeeper".to_string()),
        };
        let result = TonSignRequest::try_from(hex::decode("a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025856b5ee9c7241010201004700011c29a9a317663b3ea500000008000301006842002b16732f1c05fdb4e8d3a78fd10dddef3f6067f311be539313b8a44a504d4da2a1dcd65000000000000000000000000000007072e06f0301057830555143314979777951776978534f553870657a4f5a4443397276327843563443474a7a4f574836525838425473474a780669546f6e4b6565706572").unwrap()).unwrap();

        assert_eq!(expect_result.request_id, result.request_id);
        assert_eq!(expect_result.sign_data, result.sign_data);
        assert!(result.derivation_path.is_none());
        assert_eq!(expect_result.address, result.address);
        assert_eq!(expect_result.origin, result.origin);
    }
}
