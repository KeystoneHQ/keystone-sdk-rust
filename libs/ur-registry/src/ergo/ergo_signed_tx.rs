use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, ERGO_SIGNED_TX, UUID};
use crate::traits::{MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use minicbor::data::{Int, Tag};

const REQUEST_ID: u8 = 1;
const SIGNED_TX: u8 = 2;

impl_template_struct!(ErgoSignedTx {
    request_id: Bytes,
    signed_tx: Bytes
});

impl MapSize for ErgoSignedTx {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for ErgoSignedTx {
    fn get_registry_type() -> RegistryType<'static> {
        ERGO_SIGNED_TX
    }
}

impl<C> minicbor::Encode<C> for ErgoSignedTx {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(
            Int::try_from(REQUEST_ID)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.get_request_id())?;
        e.int(
            Int::try_from(SIGNED_TX)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .bytes(&self.get_signed_tx())?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ErgoSignedTx {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = ErgoSignedTx::default();

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
                SIGNED_TX => {
                    obj.signed_tx = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}
