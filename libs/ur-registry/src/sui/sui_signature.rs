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
        let mut size = 3;
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
