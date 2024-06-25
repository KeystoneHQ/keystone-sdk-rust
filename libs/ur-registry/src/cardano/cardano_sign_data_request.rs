use crate::cbor::cbor_map;

use crate::impl_template_struct;
use crate::error::{URError, URResult};
use crate::registry_types::{
    RegistryType, CARDANO_SIGN_DATA_REQUEST, UUID,
};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const PAYLOAD: u8 = 2;
const PATH: u8 = 3;
const ORIGIN: u8 = 4;

impl_template_struct!(CardanoSignDataRequest {request_id: Option<Bytes>, payload: Bytes, path: Bytes, origin: Option<String>});

impl MapSize for CardanoSignDataRequest {
    fn map_size(&self) -> u64 {
        let mut size = 2;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for CardanoSignDataRequest {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_SIGN_DATA_REQUEST
    }
}

impl<C> minicbor::Encode<C> for CardanoSignDataRequest {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(PAYLOAD))?.bytes(&self.payload)?;

        e.int(Int::from(PATH))?.bytes(&self.payload)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoSignDataRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_sign_data_request = CardanoSignDataRequest::default();
        cbor_map(d, &mut cardano_sign_data_request, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.set_request_id(Some(d.bytes()?.to_vec()));
                }
                PAYLOAD => {
                    obj.set_payload(d.bytes()?.to_vec());
                }
                PATH => obj.set_path(d.bytes()?.to_vec()),
                ORIGIN => obj.set_origin(Some(d.str()?.to_string())),
                _ => {}
            }
            Ok(())
        })?;
        Ok(cardano_sign_data_request)
    }
}

impl To for CardanoSignDataRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CardanoSignDataRequest> for CardanoSignDataRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoSignDataRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
