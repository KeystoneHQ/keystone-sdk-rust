use alloc::{format};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Tag, Int};

use crate::cbor::{cbor_map, cbor_array};
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, UUID, APTOS_SIGN_REQUEST};
use crate::traits::{From, RegistryItem, To};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const AUTHENTICATION_KEY_DERIVATION_PATHS: u8 = 3;
const ACCOUNTS: u8 = 4;
const ORIGIN: u8 = 5;
const SIGN_TYPE: u8 = 6;


#[derive(Clone, Debug)]
pub enum SignType {
    Single = 1,
    Multi = 2,
    Message = 3,
}

impl Default for SignType {
    fn default() -> Self {
        SignType::Single
    }
}

impl SignType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(SignType::Single),
            2 => Ok(SignType::Multi),
            3 => Ok(SignType::Message),
            x => Err(format!(
                "invalid value for sign_type in aptos-sign-request, expected (1, 2, 3), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AptosSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    authentication_key_derivation_paths: Vec<CryptoKeyPath>,
    accounts: Option<Vec<String>>,
    origin: Option<String>,
    sign_type: SignType,
}

impl AptosSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = id;
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_sign_type(&mut self, sign_type: SignType) {
        self.sign_type = sign_type
    }

    pub fn set_authentication_key_derivation_paths(&mut self, authentication_key_derivation_paths: Vec<CryptoKeyPath>) {
        self.authentication_key_derivation_paths = authentication_key_derivation_paths;
    }

    pub fn set_accounts(&mut self, accounts: Vec<String>) {
        self.accounts = Some(accounts)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        request_id: Bytes,
        sign_data: Bytes,
        authentication_key_derivation_paths: Vec<CryptoKeyPath>,
        accounts: Option<Vec<String>>,
        origin: Option<String>,
        sign_type: SignType,
    ) -> AptosSignRequest {
        AptosSignRequest {
            request_id,
            sign_data,
            sign_type,
            authentication_key_derivation_paths,
            accounts,
            origin,
        }
    }
    pub fn get_request_id(&self) -> Bytes {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_sign_type(&self) -> SignType {
        self.sign_type.clone()
    }
    pub fn get_authentication_key_derivation_paths(&self) -> Vec<CryptoKeyPath> {
        self.authentication_key_derivation_paths.clone()
    }
    pub fn get_accounts(&self) -> Option<Vec<String>> {
        self.accounts.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }
}

impl RegistryItem for AptosSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        APTOS_SIGN_REQUEST
    }
}

impl <C> minicbor::Encode<C> for AptosSignRequest {
    fn encode<W: minicbor::encode::Write>(&self, e: &mut minicbor::Encoder<W>, ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 4;
        if self.accounts.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        e.map(size)?;
        e.int(Int::try_from(REQUEST_ID).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.get_request_id())?;
        e.int(Int::try_from(SIGN_DATA).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
            .bytes(&self.get_sign_data())?;

        let authentication_key_derivation_paths = self.get_authentication_key_derivation_paths();
        if authentication_key_derivation_paths.len() == 0 {
            return Err(minicbor::encode::Error::message("authentication key derivation paths is invalid"));
        }
        e.int(Int::try_from(AUTHENTICATION_KEY_DERIVATION_PATHS).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
            .array(authentication_key_derivation_paths.len() as u64)?;
        for path in authentication_key_derivation_paths {
            e.tag(Tag::Unassigned(CryptoKeyPath::get_registry_type().get_tag()))?;
            CryptoKeyPath::encode(&path, e, ctx)?;
        }

        if let Some(accounts) = self.get_accounts() {
            e.int(Int::try_from(ACCOUNTS).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
                .array(accounts.len() as u64)?;
            for addr in accounts {
                e.str(&addr)?;
            }
        }
        if let Some(origin) = self.get_origin() {
            e.int(Int::try_from(ORIGIN).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
                .str(&origin)?;
        }
        e.int(Int::try_from(SIGN_TYPE).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
            .int(Int::try_from(self.get_sign_type() as u8).map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?;
        Ok(())
    }
}

impl <'b, C> minicbor::Decode<'b, C> for AptosSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = AptosSignRequest::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Err(minicbor::decode::Error::message("UUID tag is invalid"))
                    }
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                AUTHENTICATION_KEY_DERIVATION_PATHS => {
                    cbor_array(d, &mut obj.authentication_key_derivation_paths, |_key, obj, d| {
                        let tag = d.tag()?;
                        if !tag.eq(&Tag::Unassigned(CryptoKeyPath::get_registry_type().get_tag())) {
                            return Err(minicbor::decode::Error::message("CryptoKeyPath tag is invalid"))
                        }
                        obj.push(CryptoKeyPath::decode(d, ctx)?);
                        Ok(())
                    })?;
                }
                ACCOUNTS => {
                    if obj.accounts.is_none() {
                        obj.accounts = Some(Vec::new())
                    }
                    cbor_array(d, &mut obj.accounts, |_key, obj, d| {
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
                SIGN_TYPE => {
                    obj.sign_type = SignType::from_u32(d.u32()?).map_err(|err| minicbor::decode::Error::message(err))?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for AptosSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl From<AptosSignRequest> for AptosSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<AptosSignRequest> {
        minicbor::decode(&bytes).map_err(|e| {URError::CborDecodeError(e.to_string())})
    }
}
