use alloc::string::{String, ToString};
use minicbor::data::{Int, Tag};

use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, TON_SIGNATURE, UUID};
use crate::traits::{MapSize, RegistryItem};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;
const ORIGIN: u8 = 3;

impl_template_struct!(TonSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
    origin: Option<String>
});

impl RegistryItem for TonSignature {
    fn get_registry_type() -> RegistryType<'static> {
        TON_SIGNATURE
    }
}

impl MapSize for TonSignature {
    fn map_size(&self) -> u64 {
        let mut size = 1;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for TonSignature {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = self.get_request_id() {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(&request_id)?;
        }
        e.int(Int::from(SIGNATURE))?.bytes(&self.get_signature())?;
        if let Some(origin) = self.get_origin() {
            e.int(Int::from(ORIGIN))?.str(&origin)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for TonSignature {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = TonSignature::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Result::Err(minicbor::decode::Error::message(
                            "UUID tag is invalid",
                        ));
                    }
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGNATURE => {
                    obj.signature = d.bytes()?.to_vec();
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
    use alloc::vec::Vec;

    use super::*;
    extern crate std;
    use std::println;

    #[test]
    fn test_encode() {
        let sig = TonSignature {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            signature: hex::decode("f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505").unwrap(),
            origin: Some("Keystone".to_string()),
        };
        let result: Vec<u8> = sig.try_into().unwrap();
        let expect_result = hex::decode("a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da650503684b657973746f6e65").unwrap();

        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_decode() {
        let result = TonSignature::try_from(hex::decode("a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d025840f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da650503684b657973746f6e65").unwrap()).unwrap();
        let expect_result = TonSignature {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            signature: hex::decode("f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505").unwrap(),
            origin: Some("Keystone".to_string()),
        };

        assert_eq!(expect_result.request_id, result.request_id);
        assert_eq!(expect_result.signature, result.signature);
        assert_eq!(expect_result.origin, result.origin);
    }
}
