use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_SIGN_CIP8_DATA_REQUEST, CRYPTO_KEYPATH, UUID};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
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
const ORIGIN: u8 = 4;
const XPUB: u8 = 6;
const HASH_PAYLOAD: u8 = 7;

const ADDRESS_BENCH32: u8 = 8;
const ADDRESS_TYPE: u8 = 9;
// https://github.com/LedgerHQ/app-cardano/blob/develop/src/signMsg.c#L175-L189

#[derive(Debug, Clone, Copy, Default)]
pub enum Cip8AddressType {
    #[default]
    Address,
    KeyHash,
}

impl Cip8AddressType {
    pub fn as_str(&self) -> &'static str {
        match self {
            Cip8AddressType::Address => "ADDRESS",
            Cip8AddressType::KeyHash => "KEY_HASH",
        }
    }
}

impl From<&str> for Cip8AddressType {
    fn from(s: &str) -> Self {
        match s {
            "ADDRESS" => Cip8AddressType::Address,
            "KEY_HASH" => Cip8AddressType::KeyHash,
            _ => panic!("Invalid AddressType string"),
        }
    }
}

impl_template_struct!(CardanoSignCip8DataRequest {
    request_id: Option<Bytes>,
    sign_data: Bytes,
    derivation_path: CryptoKeyPath,
    origin: Option<String>,
    xpub: Bytes,
    hash_payload: bool,
    address_bench32: Option<String>,
    address_type: Cip8AddressType
});

impl MapSize for CardanoSignCip8DataRequest {
    fn map_size(&self) -> u64 {
        let mut size = 4;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        if self.address_bench32.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for CardanoSignCip8DataRequest {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_SIGN_CIP8_DATA_REQUEST
    }
}

impl<C> minicbor::Encode<C> for CardanoSignCip8DataRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        e.int(Int::from(XPUB))?.bytes(&self.xpub)?;
        e.int(Int::from(HASH_PAYLOAD))?.bool(self.hash_payload)?;

        if let Some(address_bench32) = &self.address_bench32 {
            e.int(Int::from(ADDRESS_BENCH32))?.str(address_bench32)?;
        }

        e.int(Int::from(ADDRESS_TYPE))?
            .str(&self.address_type.as_str())?;

        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoSignCip8DataRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result: CardanoSignCip8DataRequest = CardanoSignCip8DataRequest::default();
        cbor_map(d, &mut result, |key, obj, d: &mut Decoder| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.set_request_id(Some(d.bytes()?.to_vec()));
                }
                SIGN_DATA => {
                    obj.set_sign_data(d.bytes()?.to_vec());
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, _ctx)?;
                }
                XPUB => {
                    obj.set_xpub(d.bytes()?.to_vec());
                }
                ORIGIN => {
                    obj.origin = Some(d.str()?.to_string());
                }
                HASH_PAYLOAD => {
                    obj.hash_payload = d.bool()?;
                }
                ADDRESS_BENCH32 => {
                    obj.address_bench32 = Some(d.str()?.to_string());
                }
                ADDRESS_TYPE => {
                    obj.address_type = Cip8AddressType::from(d.str()?);
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CardanoSignCip8DataRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CardanoSignCip8DataRequest> for CardanoSignCip8DataRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoSignCip8DataRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
