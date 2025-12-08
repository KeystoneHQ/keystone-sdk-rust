use crate::cbor::{cbor_array, cbor_map};
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, AVAX_SIGN_REQUEST, UUID, CRYPTO_KEYPATH};
use crate::crypto_key_path::CryptoKeyPath;
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DERIVATION_PATH: u8 = 3;

#[derive(Debug, Clone, Default)]
pub struct AvaxSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    derivation_path: CryptoKeyPath,
}

impl AvaxSignRequest {
    pub fn new(
        request_id: Bytes,
        sign_data: Bytes,
        derivation_path: CryptoKeyPath,
    ) -> Self {
        AvaxSignRequest {
            request_id,
            sign_data,
            derivation_path,
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

    pub fn get_derivation_path(&self) -> CryptoKeyPath {
        self.derivation_path.clone()
    }

    pub fn set_derivation_path(&mut self, derivation_path: CryptoKeyPath) {
        self.derivation_path = derivation_path;
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
        e.map(3)?;
        e.int(Int::from(REQUEST_ID))?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.request_id)?;
        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;
        e.int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

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
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, _ctx)?;
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
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use alloc::vec;
    extern crate std;
    use std::println;

    #[test]
    fn test_avax_encode() {
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(9000), true).unwrap(),
            PathComponent::new(Some(1), true).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
        ];
        let unsigned_data = AvaxSignRequest {
            request_id: [12, 34, 56, 78].to_vec(),
            sign_data: Vec::from_hex("000000000022000000050000000000000000000000000000000000000000000000000000000000000000000000023d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa0000000700000000000f42400000000000000000000000010000000132336f8715dd313a426155cccc15ba27c3033dae3d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000007000000004d58ade90000000000000000000000010000000132336f8715dd313a426155cccc15ba27c3033dae00000001410b47f7c7aa13f88122be58735c5e985edc65d86fb0baf0b016359c22253d75000000013d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000005000000004d680464000000010000000000000000")
                .unwrap(),
            derivation_path: CryptoKeyPath::new(
                components,
                Some([45,11,218,188]),
                None,
            ),
        };
        let result: Vec<u8> = unsigned_data.try_into().unwrap();
        println!("result = {:?}", hex::encode(&result));
        let ur = ur::encode(&result, AvaxSignRequest::get_registry_type().get_type());
        assert_eq!(ur, "ur:avax-sign-request/otadtpdafybncpetglaohkaddmaeaeaeaeaecpaeaeaeahaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaofsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaeaebsfwfzaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadeyeojlltbzutehftfwhsgosfsfbzrddisraxfsplfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaegthdpmwlaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadeyeojlltbzutehftfwhsgosfsfbzrddisraxfsplaeaeaeadfpbdflylstpkbwyalycprnhdjkhhhymkhyuoihtpjlpfrdwtpfcmecnscpdafskpaeaeaeadfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeahaeaeaeaegtisaaieaeaeaeadaeaeaeaeaeaeaeaeaxtaaddyoeadlecsdwykcfcndeykaeykaewkaewkaocybggdrprfknlulrbk");
    }

    #[test]
    fn test_avax_decode() {
        let ur_string = "ur:avax-sign-request/otadtpdafybncpetglaohkaddmaeaeaeaeaecpaeaeaeahaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaofsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaeaebsfwfzaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadeyeojlltbzutehftfwhsgosfsfbzrddisraxfsplfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaegthdpmwlaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadeyeojlltbzutehftfwhsgosfsfbzrddisraxfsplaeaeaeadfpbdflylstpkbwyalycprnhdjkhhhymkhyuoihtpjlpfrdwtpfcmecnscpdafskpaeaeaeadfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeahaeaeaeaegtisaaieaeaeaeadaeaeaeaeaeaeaeaeaxtaaddyoeadlecsdwykcfcndeykaeykaewkaewkaocybggdrprfknlulrbk";
        
        let bytes =
            Vec::from_hex("a501d825440c22384e0258de00000000000000000001ed5f38341e436e5d46e2bb00b45d62ae97d1b050c64bc634ae10626739e35c4b0000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000089544000000000000000000000000100000001512e7191685398f00663e12197a3d8f6012d9ea300000001db720ad6707915cc4751fb7e5491a3af74e127a1d81817abe9438590c0833fe10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000050000000000989680000000010000000000000000031a0102030406786f7870756236445872797a384b6437586368745876446e6b6a61726138337368474a4838756275374b5a684868506670344c3173687644455969465a6d3332454b486e796f34627661346778586a61624647715937664e7338476764346b68597a326f4e73324b594c663536613947580706")
                .unwrap();
        let data = AvaxSignRequest::try_from(bytes).unwrap();
        assert_eq!(
            data.get_tx_data(),
            Vec::from_hex("00000000000000000001ed5f38341e436e5d46e2bb00b45d62ae97d1b050c64bc634ae10626739e35c4b0000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000089544000000000000000000000000100000001512e7191685398f00663e12197a3d8f6012d9ea300000001db720ad6707915cc4751fb7e5491a3af74e127a1d81817abe9438590c0833fe10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000050000000000989680000000010000000000000000")
                .unwrap()
        );
    }
}
