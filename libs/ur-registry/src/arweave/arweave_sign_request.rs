use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, UUID, NEAR_SIGN_REQUEST};
use crate::traits::{RegistryItem, To, From as FromCbor};
use crate::types::{Bytes, Fingerprint};

const MASTER_FINGERPRINT: u8 = 1;
const REQUEST_ID: u8 = 2;
const SIGN_DATA: u8 = 3;
const SIGN_TYPE: u8 = 4;
const SALT_LEN: u8 = 5;
const ORIGIN: u8 = 6;
const ACCOUNT: u8 = 7;


#[derive(Clone, Debug, PartialEq)]
pub enum SignType {
    Transaction = 1,
    DataItem = 2,
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
            2 => Ok(SignType::DataItem),
            x => Err(format!(
                "invalid value for sign_type in arweave-sign-request, expected (1, 2), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ArweaveSignRequest {
    master_fingerprint: Fingerprint,
    request_id: Option<Bytes>,
    sign_data: Bytes,
    sign_type: SignType,
    salt_len: u32,
    account: Option<Bytes>,
    origin: Option<String>,
}

impl ArweaveSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_master_fingerprint(&mut self, mfp: Fingerprint) {
        self.master_fingerprint = mfp;
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn set_sign_type(&mut self, sign_type: SignType) {
        self.sign_type = sign_type;
    }

    pub fn set_salt_len(&mut self, salt_len: u32) {
        self.salt_len = salt_len;
    }

    pub fn set_account(&mut self, account: Bytes) {
        self.account = Some(account)
    }

    pub fn set_origin(&mut self, origin: String) {
        self.origin = Some(origin)
    }

    pub fn new(
        master_fingerprint: Fingerprint,
        request_id: Option<Bytes>,
        sign_data: Bytes,
        sign_type: SignType,
        salt_len: u32,
        account: Option<Bytes>,
        origin: Option<String>,
    ) -> ArweaveSignRequest {
        ArweaveSignRequest {
            master_fingerprint,
            request_id,
            sign_data,
            sign_type,
            salt_len,
            account,
            origin,
        }
    }
    pub fn get_master_fingerprint(&self) -> Fingerprint { self.master_fingerprint.clone() }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Bytes {
        self.sign_data.clone()
    }
    pub fn get_sign_type(&self) -> SignType {
        self.sign_type.clone()
    }
    pub fn get_salt_len(&self) -> u32 {
        self.salt_len.clone()
    }
    pub fn get_account(&self) -> Option<Bytes> {
        self.account.clone()
    }
    pub fn get_origin(&self) -> Option<String> {
        self.origin.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 4;
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

impl RegistryItem for ArweaveSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        NEAR_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for ArweaveSignRequest {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        e.int(Int::from(MASTER_FINGERPRINT))?
            .int(Int::try_from(u32::from_be_bytes(self.master_fingerprint))
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?
            .bytes(&self.sign_data)?;

        e.int(Int::from(SIGN_TYPE))?
            .int(Int::from(self.sign_type.clone() as u8))?;

        e.int(Int::from(SALT_LEN))?
            .u32(self.salt_len)?;

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


impl<'b, C> minicbor::Decode<'b, C> for ArweaveSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ArweaveSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    let mfp = u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()));
                    obj.master_fingerprint = u32::to_be_bytes(mfp?);
                }
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                SIGN_TYPE => {
                    obj.sign_type =
                        SignType::from_u32(u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                }
                SALT_LEN => {
                    obj.salt_len = d.u32()?;
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


impl To for ArweaveSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<ArweaveSignRequest> for ArweaveSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<ArweaveSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use hex::FromHex;
    use crate::arweave::arweave_sign_request::{ArweaveSignRequest, SignType};
    use crate::traits::{To, From};

    #[test]
    fn test_encode() {
        let master_fingerprint: [u8; 4] = [233, 24, 28, 243];
        let request_id: Option<Vec<u8>> = Some([155, 29, 235,  77,  59, 125, 75, 173, 155, 221, 43, 13, 123,  61, 203, 109].to_vec());
        let sign_data = hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca").unwrap();
        let sign_type = SignType::Transaction;
        let salt_len = 0;
        let origin = Some("arconnect".to_string());

        let sign_request = ArweaveSignRequest::new(
            master_fingerprint, request_id, sign_data, sign_type, salt_len, None, origin
        );

        assert_eq!(
            "a6011ae9181cf302d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d035820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca0401050006696172636f6e6e656374",
            hex::encode(sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a6011ae9181cf302d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d035820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca0401050006696172636f6e6e656374",
        ).unwrap();

        let sign_request = ArweaveSignRequest::from_cbor(bytes).unwrap();

        let request_id = Some([155, 29, 235,  77,  59, 125, 75, 173, 155, 221, 43, 13, 123,  61, 203, 109].to_vec());
        let sign_data = hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca").unwrap();

        assert_eq!([233, 24, 28, 243], sign_request.get_master_fingerprint());
        assert_eq!(request_id, sign_request.get_request_id());
        assert_eq!(sign_data, sign_request.get_sign_data());
        assert_eq!(SignType::Transaction, sign_request.get_sign_type());
        assert_eq!(0, sign_request.get_salt_len());
        assert_eq!(Some("arconnect".to_string()), sign_request.get_origin());
    }
}
