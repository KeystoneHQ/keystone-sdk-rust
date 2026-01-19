use crate::cbor::{cbor_array, cbor_map};
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, AVAX_SIGN_REQUEST, UUID, CRYPTO_KEYPATH, AVAX_UTXO};
use crate::crypto_key_path::CryptoKeyPath;
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use super::avax_utxo::AvaxUtxo;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DERIVATION_PATH: u8 = 3;
const UTXOS: u8 = 4;

#[derive(Debug, Clone, Default)]
pub struct AvaxSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    derivation_path: Vec<CryptoKeyPath>,
    utxos: Vec<AvaxUtxo>,
}

impl AvaxSignRequest {
    pub fn new(
        request_id: Bytes,
        sign_data: Bytes,
        derivation_path: Vec<CryptoKeyPath>,
        utxos: Vec<AvaxUtxo>,
    ) -> Self {
        AvaxSignRequest {
            request_id,
            sign_data,
            derivation_path,
            utxos: utxos,
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

    pub fn get_derivation_path(&self) -> Vec<CryptoKeyPath> {
        self.derivation_path.clone()
    }

    pub fn set_derivation_path(&mut self, derivation_path: Vec<CryptoKeyPath>) {
        self.derivation_path = derivation_path;
    }

    pub fn get_utxos(&self) -> Vec<AvaxUtxo> {
        self.utxos.clone()
    }

    pub fn set_utxos(&mut self, utxos: Vec<AvaxUtxo>) {
        self.utxos = utxos;
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
        let utxos = self.get_utxos();
        let map_size = if utxos.is_empty() { 3 } else { 4 };
        
        e.map(map_size)?;
        e.int(Int::from(REQUEST_ID))?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.request_id)?;
        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        let key_derivation_paths = self.get_derivation_path();
        if key_derivation_paths.is_empty() {
            return Err(minicbor::encode::Error::message(
                "key derivation paths is invalid",
            ));
        }

        e.int(Int::from(DERIVATION_PATH))?.array(key_derivation_paths.len() as u64)?;
        for path in key_derivation_paths {
            e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
            CryptoKeyPath::encode(&path, e, _ctx)?;
        }

        if !utxos.is_empty() {
            e.int(Int::from(UTXOS))?
                .array(utxos.len() as u64)?;
            for utxo in utxos {
                e.tag(Tag::Unassigned(AvaxUtxo::get_registry_type().get_tag()))?;
                AvaxUtxo::encode(&utxo, e, _ctx)?;
            }
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for AvaxSignRequest {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
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
                    cbor_array(
                        d,
                        &mut obj.derivation_path,
                        |_key, obj, d| {
                            let tag = d.tag()?;
                            if !tag.eq(&Tag::Unassigned(
                                CryptoKeyPath::get_registry_type().get_tag(),
                            )) {
                                return Err(minicbor::decode::Error::message(
                                    "CryptoKeyPath tag is invalid",
                                ));
                            }
                            obj.push(CryptoKeyPath::decode(d, ctx)?);
                            Ok(())
                        },
                    )?;
                }
                UTXOS => {
                    cbor_array(d, &mut obj.utxos, |_key, obj, d| {
                        let tag = d.tag()?;
                        if !tag.eq(&Tag::Unassigned(
                            AvaxUtxo::get_registry_type().get_tag(),
                        )) {
                            return Err(minicbor::decode::Error::message(
                                "AvaxUtxo tag is invalid",
                            ));
                        }
                        obj.push(AvaxUtxo::decode(d, ctx)?);
                        Ok(())
                    })?;
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
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
        ];
        let utxos = vec![];
        let unsigned_data = AvaxSignRequest {
            request_id: [12, 34, 56, 78].to_vec(),
            sign_data: Vec::from_hex("0000000000220000000100000000000000000000000000000000000000000000000000000000000000000000000221e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000070000000000007d4c00000000000000000000000100000001b5e66be5c7093d1114d74940333c0c45f81092c521e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000070000000002ed658f00000000000000000000000100000001b5e66be5c7093d1114d74940333c0c45f81092c500000001918cf421e834d4d7031175ac9605ba292ee04a17beb4fb81f8557969b4651b860000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000050000000002edf716000000010000000000000000")
                .unwrap(),
            derivation_path: vec![CryptoKeyPath::new(
                components,
                Some([45,11,218,188]),
                None,
            )],
            utxos
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
            Vec::from_hex("a401d82550d797b45aef4b483cb106506e288b2c77025903960000000000220000000100000000000000000000000000000000000000000000000000000000000000000000000221e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000031cb3a00000000000000000000000100000001b5e66be5c7093d1114d74940333c0c45f81092c521e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000070000000005dabac900000000000000000000000100000001b5e66be5c7093d1114d74940333c0c45f81092c500000008120d0def706b8b759935b8ea9727662aafa5381e598a074daddc82492549cd760000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000046239d0000000100000000174d1a9b28e1d4d518f1999d4f8ac422b8a3a4755001f5965e8d05c93359feb10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000131021c0000000100000000174d1a9b28e1d4d518f1999d4f8ac422b8a3a4755001f5965e8d05c93359feb10000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000176b6a5000000010000000065a3b1de10620296debfa01aa953e45ddd19d2c39e3dacb9a92e6a85ca8a309c0000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000098968000000001000000006f6522ae52b0231076dc63ff95f7ea22e2fd80943e37235302c7ee32afce4cd60000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff0000000500000000009896800000000100000000845649c3d1a630d8b466f7b727f6577cb4a17864699e6de756e484b81d84cd2a0000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff0000000500000000006c2b440000000100000000d1e6480c1825197e2ec293a60bacdc7f60bfba2f3cc5383855180b45d595a7030000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff0000000500000000007a12000000000100000000f59b9a175ebe4ccd8de5dcfc6a26870414f30c696cce19283f30145624b445b70000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000005000000000107a4930000000100000000000000000382d90130a2018a182cf5192328f500f500f400f4021a2d0bdabcd90130a2018a182cf5192328f500f500f401f4021a2d0bdabc0480")
                .unwrap();
        let data = AvaxSignRequest::try_from(bytes).unwrap();
        assert_eq!(
            data.get_tx_data(),
            Vec::from_hex("00000000000000000001ed5f38341e436e5d46e2bb00b45d62ae97d1b050c64bc634ae10626739e35c4b0000000121e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff00000007000000000089544000000000000000000000000100000001512e7191685398f00663e12197a3d8f6012d9ea300000001db720ad6707915cc4751fb7e5491a3af74e127a1d81817abe9438590c0833fe10000000021e67317cbc4be2aeb00677ad6462778a8f52274b9d605df2591b23027a87dff000000050000000000989680000000010000000000000000")
                .unwrap()
        );
    }
}
