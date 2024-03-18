use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, BTC_SIGNATURE, UUID};
use crate::traits::RegistryItem;
use crate::types::Bytes;
use alloc::string::ToString;
use minicbor::data::{Int, Tag};

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;
const PUBLIC_KEY: u8 = 3;

impl_template_struct!(BtcSignature {
    request_id: Bytes,
    signature: Bytes,
    public_key: Bytes
});

impl RegistryItem for BtcSignature {
    fn get_registry_type() -> RegistryType<'static> {
        BTC_SIGNATURE
    }
}

impl<C> minicbor::Encode<C> for BtcSignature {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(3)?;
        e.int(
            Int::try_from(REQUEST_ID)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .tag(Tag::Unassigned(UUID.get_tag()))?
        .bytes(&self.get_request_id())?;
        e.int(
            Int::try_from(SIGNATURE)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .bytes(&self.get_signature())?;
        e.int(
            Int::try_from(PUBLIC_KEY)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .bytes(&self.get_public_key())?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for BtcSignature {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = BtcSignature::default();

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
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGNATURE => {
                    obj.signature = d.bytes()?.to_vec();
                }
                PUBLIC_KEY => {
                    obj.public_key = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(result)
    }
}
