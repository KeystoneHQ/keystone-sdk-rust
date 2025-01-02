use crate::cbor::{cbor_array, cbor_map};
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, AVAX_SIGN_REQUEST, UUID};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const XPUB: u8 = 6;
const WALLET_INDEX: u8 = 7;

#[derive(Debug, Clone, Default)]
pub struct AvaxSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    xpub: String,
    wallet_index: u64,
}

impl AvaxSignRequest {
    pub fn new(request_id: Bytes, sign_data: Bytes, xpub: String, wallet_index: u64) -> Self {
        AvaxSignRequest {
            request_id,
            sign_data,
            xpub,
            wallet_index,
        }
    }

    pub fn get_request_id(&self) -> Bytes {
        self.request_id.clone()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = id;
    }

    pub fn get_tx_data(&self) -> Bytes {
        self.sign_data.clone()
    }

    pub fn set_tx_data(&mut self, data: Bytes) {
        self.sign_data = data;
    }

    pub fn get_xpub(&self) -> String {
        self.xpub.clone()
    }

    pub fn set_xpub(&mut self, xpub: String) {
        self.xpub = xpub;
    }

    pub fn get_wallet_index(&self) -> u64 {
        self.wallet_index
    }

    pub fn set_wallet_index(&mut self, index: u64) {
        self.wallet_index = index;
    }
}

impl RegistryItem for AvaxSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        AVAX_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for AvaxSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(4)?;
        e.int(Int::from(REQUEST_ID))?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.request_id)?;
        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;
        e.int(Int::from(XPUB))?.str(&self.xpub)?;
        e.int(Int::from(WALLET_INDEX))?.u64(self.wallet_index)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for AvaxSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = AvaxSignRequest::default();

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
                XPUB => {
                    obj.xpub = d.str()?.to_string();
                }
                WALLET_INDEX => {
                    obj.wallet_index = d.u64()?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for AvaxSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<AvaxSignRequest> for AvaxSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<AvaxSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::RegistryItem;
    use alloc::vec::Vec;
    use hex::FromHex;
    extern crate std;
    use std::println;

    #[test]
    fn test_avax_encode() {
        let unsigned_data = AvaxSignRequest {
            request_id: [12, 34, 56, 78].to_vec(),
            sign_data: Vec::from_hex("00000000000000000001ed5f38341e436e5d46e2bb00b45d62ae97d1b050c64bc634ae10626739e35c4b0000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000089544000000000000000000000000100000001512e7191685398f00663e12197a3d8f6012d9ea300000001db720ad6707915cc4751fb7e5491a3af74e127a1d81817abe9438590c0833fe10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000050000000000989680000000010000000000000000")
                .unwrap(),
            xpub: "xpub6DXryz8Kd7XchtXvDnkjara83shGJH8ubu7KZhHhPfp4L1shvDEYiFZm32EKHnyo4bva4gxXjabFGqY7fNs8Ggd4khYz2oNs2KYLf56a9GX".to_string(),
            wallet_index: 6,
            };
        let result: Vec<u8> = unsigned_data.try_into().unwrap();
        let ur = ur::encode(&result, AvaxSignRequest::get_registry_type().get_type());
        assert_eq!(ur, "ur:avax-sign-request/hkaddmaeaeaeaeaecpaeaeaeahaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaofsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaeahykvyaeaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadltjsmobwadtlrszmykmotnvsiymdolbzryqzoxfpfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaechstjstdaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadasinwdidvorkdyvajnlfvsdlvoioweynltckonylaeaeaeadhgtlvofmdmctfgbdhslurdcwgomefhwftoqdbzwtttpsssctvafzmnuogtwlzssnaeaeaeaefsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeahaeaeaeaecaryiobtaeaeaeadaeaeaeaeaeaeaeaeecfhesrp");
    }

    #[test]
    fn test_avax_decode() {
        let bytes =
            Vec::from_hex("a401d82544123456780258de00000000000000000001ed5f38341e436e5d46e2bb00b45d62ae97d1b050c64bc634ae10626739e35c4b0000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000089544000000000000000000000000100000001512e7191685398f00663e12197a3d8f6012d9ea300000001db720ad6707915cc4751fb7e5491a3af74e127a1d81817abe9438590c0833fe10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000098968000000001000000000000000006786f7870756236445872797a384b6437586368745876446e6b6a61726138337368474a4838756275374b5a684868506670344c3173687644455969465a6d3332454b486e796f34627661346778586a61624647715937664e7338476764346b68597a326f4e73324b594c663536613947580706")
                .unwrap();
        let data = AvaxSignRequest::try_from(bytes).unwrap();
        assert_eq!(
            data.get_tx_data(),
            Vec::from_hex("000000000022000000050000000000000000000000000000000000000000000000000000000000000000000000023d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa000000070000000005f5e100000000000000000000000001000000018771921301d5bffff592dae86695a615bdb4a4413d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa000000070000000017c771d2000000000000000000000001000000010969ea62e2bb30e66d82e82fe267edf6871ea5f70000000157d5e23e2e1f460b618bba1b55913ff3ceb315f0d1acc41fe6408edc4de9facd000000003d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000005000000001dbd670d000000010000000000000000")
                .unwrap()
        );
    }
}
