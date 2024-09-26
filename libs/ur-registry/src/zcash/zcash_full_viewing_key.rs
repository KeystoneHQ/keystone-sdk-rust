use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::cbor_map,
    crypto_key_path::CryptoKeyPath,
    impl_template_struct,
    registry_types::{RegistryType, CRYPTO_KEYPATH, UUID, ZCASH_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

const KEY_PATH: u8 = 1;
const KEY_DATA: u8 = 2;

impl_template_struct!(ZcashFullViewingKey {
    key_path: CryptoKeyPath,
    key_data: Bytes
});

impl MapSize for ZcashFullViewingKey {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for ZcashFullViewingKey {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_FULL_VIEWING_KEY
    }
}

impl<C> minicbor::Encode<C> for ZcashFullViewingKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(KEY_DATA))?.bytes(&self.key_data)?;

        e.int(Int::from(KEY_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.key_path, e, _ctx)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashFullViewingKey {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashFullViewingKey::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                KEY_PATH => {
                    d.tag()?;
                    obj.key_path = Some(CryptoKeyPath::decode(d, ctx)?);
                }
                KEY_DATA => {
                    obj.key_data = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {}
