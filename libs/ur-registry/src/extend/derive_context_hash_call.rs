use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, DERIVE_CONTEXT_HASH_CALL};
use crate::traits::{MapSize, RegistryItem};
use alloc::string::{String, ToString};
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const APP_NAME: u8 = 1;
const NETWORK: u8 = 2;
const KEY_PATH: u8 = 3;
const CONTEXT: u8 = 4;

impl_template_struct!(DeriveContextHashCall {
    app_name: String,
    network: String,
    key_path: CryptoKeyPath,
    context: String
});

impl RegistryItem for DeriveContextHashCall {
    fn get_registry_type() -> RegistryType<'static> {
        DERIVE_CONTEXT_HASH_CALL
    }
}

impl MapSize for DeriveContextHashCall {
    fn map_size(&self) -> u64 {
        4
    }
}

impl<C> minicbor::Encode<C> for DeriveContextHashCall {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(APP_NAME))?.str(&self.app_name)?;
        e.int(Int::from(NETWORK))?.str(&self.network)?;

        e.int(Int::from(KEY_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        self.key_path.encode(e, ctx)?;

        e.int(Int::from(CONTEXT))?.str(&self.context)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for DeriveContextHashCall {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = DeriveContextHashCall::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                APP_NAME => {
                    obj.set_app_name(d.str()?.to_string());
                }
                NETWORK => {
                    obj.set_network(d.str()?.to_string());
                }
                KEY_PATH => {
                    d.tag()?;
                    obj.set_key_path(CryptoKeyPath::decode(d, ctx)?);
                }
                CONTEXT => {
                    obj.set_context(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}
