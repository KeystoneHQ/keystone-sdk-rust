use alloc::string::ToString;
use minicbor::data::{Int, Tag};

use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, SUI_SIGNATURE, UUID};
use crate::traits::{RegistryItem, MapSize};
use crate::types::Bytes;

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;
const PUBLIC_KEY: u8 = 3;

impl_template_struct!(SuiSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
    public_key: Option<Bytes>
});

impl RegistryItem for SuiSignature {
    fn get_registry_type() -> RegistryType<'static> {
        SUI_SIGNATURE
    }
}

impl MapSize for SuiSignature {
    fn map_size(&self) -> u64 {
        let mut size = 1;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.public_key.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for SuiSignature {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = self.get_request_id() {
            e.int(Int::from(REQUEST_ID))?.tag(Tag::Unassigned(UUID.get_tag()))?.bytes(&request_id)?;
        }
        e.int(Int::from(SIGNATURE))?.bytes(&self.get_signature())?;
        if let Some(public_key) = self.get_public_key() {
            e.int(Int::from(PUBLIC_KEY))?.bytes(&public_key)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for SuiSignature {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = SuiSignature::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Result::Err(minicbor::decode::Error::message("UUID tag is invalid"));
                    }
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGNATURE => {
                    obj.signature = d.bytes()?.to_vec();
                }
                PUBLIC_KEY => {
                    obj.public_key = Some(d.bytes()?.to_vec());
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

    #[test]
    fn test_encode() {
        let sig = SuiSignature {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            signature: hex::decode("f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505").unwrap(),
            public_key: Some(hex::decode("aeb28ecace5c664c080e71b9efd3d071b3dac119a26f4e830dd6bd06712ed93f").unwrap())
        };
        let result: Vec<u8> = sig.try_into().unwrap();
        let expect_result = hex::decode("A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840F4B79835417490958C72492723409289B444F3AF18274BA484A9EEACA9E760520E453776E5975DF058B537476932A45239685F694FC6362FE5AF6BA714DA6505035820AEB28ECACE5C664C080E71B9EFD3D071B3DAC119A26F4E830DD6BD06712ED93F").unwrap();

        assert_eq!(
            expect_result,
            result
        );
    }

    #[test]
    fn test_decode() {
        let result = SuiSignature::try_from(hex::decode("A301D825509B1DEB4D3B7D4BAD9BDD2B0D7B3DCB6D025840F4B79835417490958C72492723409289B444F3AF18274BA484A9EEACA9E760520E453776E5975DF058B537476932A45239685F694FC6362FE5AF6BA714DA6505035820AEB28ECACE5C664C080E71B9EFD3D071B3DAC119A26F4E830DD6BD06712ED93F").unwrap()).unwrap();
        let expect_result = SuiSignature {
            request_id: Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()),
            signature: hex::decode("f4b79835417490958c72492723409289b444f3af18274ba484a9eeaca9e760520e453776e5975df058b537476932a45239685f694fc6362fe5af6ba714da6505").unwrap(),
            public_key: Some(hex::decode("aeb28ecace5c664c080e71b9efd3d071b3dac119a26f4e830dd6bd06712ed93f").unwrap())
        };

        assert_eq!(expect_result.request_id, result.request_id);
        assert_eq!(expect_result.signature, result.signature);
        assert_eq!(expect_result.public_key, result.public_key);
    }
}
