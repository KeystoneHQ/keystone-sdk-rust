use crate::cbor::cbor_map;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_SIGN_CIP8_DATA_SIGNATURE, UUID};
use crate::traits::{MapSize, RegistryItem};
use crate::types::Bytes;
use alloc::string::ToString;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGNATURE: u8 = 2;
const PUBLIC_KEY: u8 = 3;

impl_template_struct!(CardanoSignCip8DataSignature {
    request_id: Option<Bytes>,
    signature: Bytes,
    public_key: Bytes
});

impl RegistryItem for CardanoSignCip8DataSignature {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_SIGN_CIP8_DATA_SIGNATURE
    }
}

impl MapSize for CardanoSignCip8DataSignature {
    fn map_size(&self) -> u64 {
        let mut size = 2;
        if self.request_id.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for CardanoSignCip8DataSignature {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        if let Some(id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(id)?;
        }

        e.int(Int::from(SIGNATURE))?.bytes(&self.signature)?;

        e.int(Int::from(PUBLIC_KEY))?.bytes(&self.public_key)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoSignCip8DataSignature {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_sign_data_signature = CardanoSignCip8DataSignature::default();
        cbor_map(d, &mut cardano_sign_data_signature, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.set_request_id(Some(d.bytes()?.to_vec()));
                }
                SIGNATURE => {
                    obj.set_signature(d.bytes()?.to_vec());
                }
                PUBLIC_KEY => {
                    obj.set_public_key(d.bytes()?.to_vec());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(cardano_sign_data_signature)
    }
}
