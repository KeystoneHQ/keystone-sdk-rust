use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};

use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_key_path::CryptoKeyPath;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, SUI_SIGN_REQUEST, UUID};
use crate::traits::{MapSize, RegistryItem};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const INTENT_MESSAGE: u8 = 2;
const DERIVATION_PATHS: u8 = 3;
const ADDRESSES: u8 = 4;
const ORIGIN: u8 = 5;

impl_template_struct!(SuiSignRequest {
    request_id: Option<Bytes>,
    intent_message: Bytes,
    derivation_paths: Vec<CryptoKeyPath>,
    addresses: Option<Vec<Bytes>>,
    origin: Option<String>
});

impl RegistryItem for SuiSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        SUI_SIGN_REQUEST
    }
}

impl MapSize for SuiSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 2;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.addresses.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for SuiSignRequest {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = self.get_request_id() {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(&request_id)?;
        }
        e.int(Int::from(INTENT_MESSAGE))?
            .bytes(&self.get_intent_message())?;

        let derivation_paths = self.get_derivation_paths();
        if derivation_paths.is_empty() {
            return Err(minicbor::encode::Error::message(
                "derivation paths is invalid",
            ));
        }
        e.int(Int::from(DERIVATION_PATHS))?
            .array(derivation_paths.len() as u64)?;
        for path in derivation_paths {
            e.tag(Tag::Unassigned(
                CryptoKeyPath::get_registry_type().get_tag(),
            ))?;
            CryptoKeyPath::encode(&path, e, ctx)?;
        }

        if let Some(addresses) = self.get_addresses() {
            e.int(Int::from(ADDRESSES))?.array(addresses.len() as u64)?;
            for addr in addresses {
                e.bytes(&addr)?;
            }
        }

        if let Some(origin) = self.get_origin() {
            e.int(Int::from(ORIGIN))?.str(&origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for SuiSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = SuiSignRequest::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Err(minicbor::decode::Error::message("UUID tag is invalid"));
                    }
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                INTENT_MESSAGE => {
                    obj.intent_message = d.bytes()?.to_vec();
                }
                DERIVATION_PATHS => {
                    cbor_array(d, &mut obj.derivation_paths, |_key, obj, d| {
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
                    })?;
                }
                ADDRESSES => {
                    if obj.addresses.is_none() {
                        obj.addresses = Some(Vec::new())
                    }
                    cbor_array(d, &mut obj.addresses, |_key, obj, d| {
                        match obj {
                            Some(v) => v.push(d.bytes()?.to_vec()),
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

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;

    use crate::crypto_key_path::PathComponent;

    use super::*;

    #[test]
    fn test_encode() {
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(784), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
        ];
        let source_fingerprint = hex::decode("78230804").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        let sig = SuiSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            intent_message: hex::decode("00000000000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e803000000000000640000000000000000").unwrap(),
            derivation_paths: vec![crypto_key_path],
            addresses: Some(vec![hex::decode("ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869").unwrap()]),
            origin: Some("Sui Wallet".to_string())
        };
        let result: Vec<u8> = sig.try_into().unwrap();
        let expect_result = hex::decode("a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258dc00000000000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e8030000000000006400000000000000000381d90130a2018a182cf5190310f500f500f500f5021a7823080404815820ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869056a5375692057616c6c6574").unwrap();

        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_decode() {
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(784), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
        ];
        let source_fingerprint = hex::decode("78230804").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        let expect_result = SuiSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            intent_message: hex::decode("00000000000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e803000000000000640000000000000000").unwrap(),
            derivation_paths: vec![crypto_key_path],
            addresses: Some(vec![hex::decode("ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869").unwrap()]),
            origin: Some("Sui Wallet".to_string())
        };
        let result = SuiSignRequest::try_from(hex::decode("a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258dc00000000000200201ff915a5e9e32fdbe0135535b6c69a00a9809aaf7f7c0275d3239ca79db20d6400081027000000000000020200010101000101020000010000ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec3944093886901a2e3e42930675d9571a467eb5d4b22553c93ccb84e9097972e02c490b4e7a22ab73200000000000020176c4727433105da34209f04ac3f22e192a2573d7948cb2fabde7d13a7f4f149ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869e8030000000000006400000000000000000381d90130a2018a182cf5190310f500f500f500f5021a7823080404815820ebe623e33b7307f1350f8934beb3fb16baef0fc1b3f1b92868eec39440938869056a5375692057616c6c6574").unwrap()).unwrap();

        assert_eq!(expect_result.request_id, result.request_id);
        assert_eq!(expect_result.intent_message, result.intent_message);
        assert_eq!(
            expect_result.derivation_paths[0].get_path(),
            result.derivation_paths[0].get_path()
        );
        assert_eq!(expect_result.addresses, result.addresses);
        assert_eq!(expect_result.origin, result.origin);
    }
}
