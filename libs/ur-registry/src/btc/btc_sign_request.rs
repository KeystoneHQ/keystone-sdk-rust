use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};

use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, BTC_SIGN_REQUEST, UUID};
use crate::traits::{From, RegistryItem, To};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DATA_TYPE: u8 = 3;
const DERIVATION_PATHS: u8 = 4;
const ADDRESSES: u8 = 5;
const ORIGIN: u8 = 6;

#[derive(Clone, Debug)]
#[derive(Default)]
pub enum DataType {
    #[default]
    Message = 1,
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Message),
            x => Err(format!(
                "invalid value for data_type in btc-sign-request, expected (1), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BtcSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    data_type: DataType,
    derivation_paths: Vec<CryptoKeyPath>,
    addresses: Option<Vec<String>>,
    origin: Option<String>,
}

impl BtcSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = id;
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_data_type(&mut self, data_type: DataType) {
        self.data_type = data_type
    }

    pub fn set_derivation_paths(&mut self, derivation_paths: Vec<CryptoKeyPath>) {
        self.derivation_paths = derivation_paths;
    }

    pub fn set_addresses(&mut self, addresses: Vec<String>) {
        self.addresses = Some(addresses)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        request_id: Bytes,
        sign_data: Bytes,
        data_type: DataType,
        derivation_paths: Vec<CryptoKeyPath>,
        addresses: Option<Vec<String>>,
        origin: Option<String>,
    ) -> BtcSignRequest {
        BtcSignRequest {
            request_id,
            sign_data,
            data_type,
            derivation_paths,
            addresses,
            origin,
        }
    }
    pub fn get_request_id(&self) -> Bytes {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_data_type(&self) -> DataType {
        self.data_type.clone()
    }
    pub fn get_derivation_paths(&self) -> Vec<CryptoKeyPath> {
        self.derivation_paths.clone()
    }
    pub fn get_addresses(&self) -> Option<Vec<String>> {
        self.addresses.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }
}

impl RegistryItem for BtcSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        BTC_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for BtcSignRequest {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 4;
        if self.addresses.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        e.map(size)?;
        e.int(
            Int::try_from(REQUEST_ID)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .tag(Tag::Unassigned(UUID.get_tag()))?
        .bytes(&self.get_request_id())?;
        e.int(
            Int::try_from(SIGN_DATA)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .bytes(&self.get_sign_data())?;
        e.int(
            Int::try_from(DATA_TYPE)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .int(
            Int::try_from(self.get_data_type() as u8)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?;

        let derivation_paths = self.get_derivation_paths();
        if derivation_paths.is_empty() {
            return Result::Err(minicbor::encode::Error::message(
                "derivation_paths is invalid",
            ));
        }
        e.int(
            Int::try_from(DERIVATION_PATHS)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .array(derivation_paths.len() as u64)?;
        for path in derivation_paths {
            e.tag(Tag::Unassigned(
                CryptoKeyPath::get_registry_type().get_tag(),
            ))?;
            CryptoKeyPath::encode(&path, e, ctx)?;
        }

        if let Some(addresses) = self.get_addresses() {
            e.int(
                Int::try_from(ADDRESSES)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?
            .array(addresses.len() as u64)?;
            for addr in addresses {
                e.str(&addr)?;
            }
        }
        if let Some(origin) = self.get_origin() {
            e.int(
                Int::try_from(ORIGIN)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?
            .str(&origin)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for BtcSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = BtcSignRequest::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Result::Err(minicbor::decode::Error::message("UUID tag is invalid"));
                    }
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                DATA_TYPE => {
                    obj.data_type = DataType::from_u32(d.u32()?)
                        .map_err(minicbor::decode::Error::message)?;
                }
                DERIVATION_PATHS => {
                    cbor_array(d, &mut obj.derivation_paths, |_key, obj, d| {
                        let tag = d.tag()?;
                        if !tag.eq(&Tag::Unassigned(
                            CryptoKeyPath::get_registry_type().get_tag(),
                        )) {
                            return Result::Err(minicbor::decode::Error::message(
                                "CryptoKeyPath tag is invalid",
                            ));
                        }
                        obj.push(CryptoKeyPath::decode(d, ctx)?);
                        Ok(())
                    })?;
                }
                ADDRESSES => {
                    if obj.addresses.is_none() {
                        obj.addresses = Some(Vec::new())
                    }
                    cbor_array(d, &mut obj.addresses, |_key, obj, d| {
                        match obj {
                            Some(v) => v.push(d.str()?.to_string()),
                            None => {}
                        }
                        Ok(())
                    })?;
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

impl To for BtcSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl From<BtcSignRequest> for BtcSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<BtcSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
