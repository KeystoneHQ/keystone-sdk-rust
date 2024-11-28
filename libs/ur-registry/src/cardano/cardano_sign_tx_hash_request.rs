use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_SIGN_TX_HASH_REQUEST, UUID};
use crate::traits::{MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const TX_HASH: u8 = 2;
const PATHS: u8 = 3;
const ORIGIN: u8 = 4;

impl_template_struct!(
    CardanoSignTxHashRequest {
        request_id: Option<Bytes>,
        tx_hash: String,
        paths: Vec<CryptoKeyPath>,
        origin: Option<String>
    }
);

impl MapSize for CardanoSignTxHashRequest {
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

impl RegistryItem for CardanoSignTxHashRequest {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_SIGN_TX_HASH_REQUEST
    }
}

impl To for CardanoSignTxHashRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl<C> minicbor::Encode<C> for CardanoSignTxHashRequest {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }
        e.int(Int::from(TX_HASH))?.str(&self.tx_hash)?;
        e.int(Int::from(PATHS))?.array(self.paths.len() as u64)?;
        for x in &self.paths {
            e.tag(Tag::Unassigned(
                CryptoKeyPath::get_registry_type().get_tag(),
            ))?;
            x.encode(e, _ctx)?;
        }
        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoSignTxHashRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_sign_request = CardanoSignTxHashRequest::default();
        cbor_map(d, &mut cardano_sign_request, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.set_request_id(Some(d.bytes()?.to_vec()));
                }
                TX_HASH => {
                    obj.set_tx_hash(d.str()?.to_string());
                }
                PATHS => {
                    cbor_array(d, &mut obj.paths, |_index, array, d| {
                        d.tag()?;
                        array.push(CryptoKeyPath::decode(d, _ctx)?);
                        Ok(())
                    })?;
                }
                ORIGIN => {
                    obj.set_origin(Some(d.str()?.to_string()));
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(cardano_sign_request)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_key_path::CryptoKeyPath;
    use crate::crypto_key_path::PathComponent;
    use alloc::vec;
    extern crate std;

    #[test]
    fn test_cardano_sign_tx_hash_request() {
        let origin = "eternl".to_string();
        let request_id = hex::decode("52090a1c29394842a9adba0bc021a58b").unwrap();
        let tx_hash = "52a1f5596f31358030f0d9d3a2db2b119b8f766386071684d26d0d37439c144e";
        let mut paths = vec![];
        let components = vec![
            PathComponent::new(Some(1852), true).unwrap(),
            PathComponent::new(Some(1815), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
        ];
        let source_fingerprint = hex::decode("1250b6bc").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        paths.push(crypto_key_path);
        let components = vec![
            PathComponent::new(Some(1852), true).unwrap(),
            PathComponent::new(Some(1815), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(2), false).unwrap(),
            PathComponent::new(Some(0), false).unwrap(),
        ];
        let source_fingerprint = hex::decode("1250b6bc").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        paths.push(crypto_key_path);
        let request = CardanoSignTxHashRequest {
            request_id: Some(request_id),
            tx_hash: tx_hash.to_string(),
            paths,
            origin: Some(origin),
        };
        let expect_result = CardanoSignTxHashRequest::try_from(hex::decode("a401d8255052090a1c29394842a9adba0bc021a58b027840353261316635353936663331333538303330663064396433613264623262313139623866373636333836303731363834643236643064333734333963313434650382d90130a2018a19073cf5190717f500f500f400f4021a1250b6bcd90130a2018a19073cf5190717f500f502f400f4021a1250b6bc0466657465726e6c").unwrap()).unwrap();
        assert_eq!(expect_result.request_id, request.request_id);
        assert_eq!(expect_result.tx_hash, request.tx_hash);
        assert_eq!(expect_result.paths, request.paths);
        assert_eq!(expect_result.origin, request.origin);
    }
}
