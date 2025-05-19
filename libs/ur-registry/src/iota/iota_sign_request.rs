use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};

use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_key_path::CryptoKeyPath;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, IOTA_SIGN_REQUEST, UUID};
use crate::traits::{MapSize, RegistryItem};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const INTENT_MESSAGE: u8 = 2;
const DERIVATION_PATHS: u8 = 3;
const ADDRESSES: u8 = 4;
const ORIGIN: u8 = 5;

impl_template_struct!(IotaSignRequest {
    request_id: Option<Bytes>,
    intent_message: Bytes,
    derivation_paths: Vec<CryptoKeyPath>,
    addresses: Option<Vec<Bytes>>,
    origin: Option<String>
});

impl RegistryItem for IotaSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        IOTA_SIGN_REQUEST
    }
}

impl MapSize for IotaSignRequest {
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

impl<C> minicbor::Encode<C> for IotaSignRequest {
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

impl<'b, C> minicbor::Decode<'b, C> for IotaSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = IotaSignRequest::default();

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
    extern crate std;
    use std::println;

    use crate::crypto_key_path::PathComponent;

    use super::*;

    #[test]
    fn test_iota_sign_request_encode() {
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(4218), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
        ];
        let source_fingerprint = hex::decode("E57D9654").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        // stake
        let intent_message = "01000000000300080094357700000000010100000000000000000000000000000000000000000000000000000000000000050100000000000000010020be550ed781f1c2fb015343b25f247de46b0b1e54a75091b8b10fbb379fd9a058020200010100000000000000000000000000000000000000000000000000000000000000000000030b696f74615f73797374656d11726571756573745f6164645f7374616b650003010100020000010200193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04010266c887c1357c50c0b5e6fe7073a22f89deb9c3dfba9edf937edf6f2ca32cbe504e630d0000000020dcf06d5def0011c98b13c5eaa346c444ca8e3dd879cb51a00062d3654ba7c347193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04e803000000000000601341000000000000";

        // transaction intent message
        let intent_message = "0100000000020008005ed0b2000000000020ae03b45942086752b470ce7806a6d50bd05ae0085a052108e4444f94d92c2535020200010100000101020000010100193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04030266c887c1357c50c0b5e6fe7073a22f89deb9c3dfba9edf937edf6f2ca32cbebe0021110000000020b2f9dd61ef4a38625d69b97484e5d832057aaea1e7f8b4f9040575da62cacad330849e93e113b34c8b167f784a4a791d17930464518176eb76989aa8e7d763a1b951630d0000000020479e005468b4a9479eecf9bf0823ac7bc1f2f718b746757ec9467aefdd423f93f567740212875ab10edfcbec293bee7b8852bb7433996c01c12d554fd1a256cbb851630d000000002054c4924cd98469979987aaab38099b79cd044c4a48e3edc3475670ac8a415512193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04e803000000000000e06f3c000000000000";
        let sig = IotaSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            intent_message: hex::decode(intent_message).unwrap(),
            derivation_paths: vec![crypto_key_path],
            addresses: Some(vec![hex::decode("193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04").unwrap()]),
            origin: Some("Nightly Wallet".to_string())
        };
        let result: Vec<u8> = sig.try_into().unwrap();
        assert_eq!(hex::encode(&result).to_lowercase(), "a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258db000002000800e40b54020000000020193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04020200010100000101030000000001010032bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa201b9ee296780e0b8e23456c605bacfaad200e468985e5e9a898c7b31919225066eef48630d0000000020b1d6709ed59ff892f9a86cd38493b45b7cbb96cf5a459fdc49cbdbc6e79921f832bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa2e80300000000000000e40b5402000000000381d90130a2018a182cf519107af500f500f500f5021ae57d965404815820193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04056e4e696768746c792057616c6c6574");
    }

    #[test]
    fn test_iota_sign_request_decode() {
        let components = vec![
            PathComponent::new(Some(44), true).unwrap(),
            PathComponent::new(Some(4218), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
            PathComponent::new(Some(0), true).unwrap(),
        ];
        let source_fingerprint = hex::decode("E57D9654").unwrap().try_into().unwrap();
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);
        let expect_result = IotaSignRequest {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            intent_message: hex::decode("000000000002000800e40b54020000000020193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04020200010100000101030000000001010032bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa201b9ee296780e0b8e23456c605bacfaad200e468985e5e9a898c7b31919225066eef48630d0000000020b1d6709ed59ff892f9a86cd38493b45b7cbb96cf5a459fdc49cbdbc6e79921f832bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa2e80300000000000000e40b540200000000").unwrap(),
            derivation_paths: vec![crypto_key_path],
            addresses: Some(vec![hex::decode("193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04").unwrap()]),
            origin: Some("Nightly Wallet".to_string())
        };
        let result = IotaSignRequest::try_from(hex::decode("a501d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0258db000002000800e40b54020000000020193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04020200010100000101030000000001010032bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa201b9ee296780e0b8e23456c605bacfaad200e468985e5e9a898c7b31919225066eef48630d0000000020b1d6709ed59ff892f9a86cd38493b45b7cbb96cf5a459fdc49cbdbc6e79921f832bc9471570ca24fcd1fe5b201ea6894748aa0ddd44d20c68f1a4f99db513aa2e80300000000000000e40b5402000000000381d90130a2018a182cf519107af500f500f500f5021ae57d965404815820193a4811b7207ac7a861f840552f9c718172400f4c46bdef5935008a7977fb04056e4e696768746c792057616c6c6574").unwrap()).unwrap();
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
