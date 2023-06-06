use alloc::string::ToString;
use alloc::vec;
use crate::extend::key_derivation_schema::KeyDerivationSchema;
use crate::impl_template_struct;
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::data::Tag;
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};
use crate::cbor::{cbor_array, cbor_map};
use crate::registry_types::{KEY_DERIVATION_CALL, KEY_DERIVATION_SCHEMA, RegistryType};
use crate::traits::{MapSize, RegistryItem};

const SCHEMAS: u8 = 1;

impl_template_struct!(KeyDerivationCall {
    schemas: Vec<KeyDerivationSchema>
});

impl RegistryItem for KeyDerivationCall {
    fn get_registry_type() -> RegistryType<'static> {
        KEY_DERIVATION_CALL
    }
}

impl MapSize for KeyDerivationCall {
    fn map_size(&self) -> u64 {
        1
    }
}

impl<C> minicbor::Encode<C> for KeyDerivationCall {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(SCHEMAS))?
            .array(self.get_schemas().len() as u64)?;
        for x in self.get_schemas() {
            e.tag(Tag::Unassigned(KEY_DERIVATION_SCHEMA.get_tag()))?;
            x.encode(e, ctx)?
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for KeyDerivationCall {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = KeyDerivationCall::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SCHEMAS => {
                    let mut schemas = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        schemas.push(KeyDerivationSchema::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.set_schemas(schemas)
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}