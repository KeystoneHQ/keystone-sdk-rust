use alloc::{
    format,
    string::{String, ToString},
};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::cbor_map,
    crypto_hd_key::CryptoHDKey,
    crypto_key_path::CryptoKeyPath,
    impl_template_struct,
    registry_types::{
        RegistryType, CRYPTO_HDKEY, CRYPTO_KEYPATH, ZCASH_FULL_VIEWING_KEY,
        ZCASH_UNIFIED_FULL_VIEWING_KEY,
    },
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

use super::zcash_full_viewing_key::ZcashFullViewingKey;

const TRANSPARENT: u8 = 1;
const ORCHARD: u8 = 2;
const NAME: u8 = 3;

impl_template_struct!(ZcashUnifiedFullViewingKey {
    transparent: Option<CryptoHDKey>,
    orchard: ZcashFullViewingKey,
    name: Option<String>
});

impl MapSize for ZcashUnifiedFullViewingKey {
    fn map_size(&self) -> u64 {
        let mut size = 1;
        if self.transparent.is_some() {
            size += 1;
        }
        if self.name.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for ZcashUnifiedFullViewingKey {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_UNIFIED_FULL_VIEWING_KEY
    }
}

impl<C> minicbor::Encode<C> for ZcashUnifiedFullViewingKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        if let Some(transparent) = &self.transparent {
            e.int(Int::from(TRANSPARENT))?
                .tag(Tag::Unassigned(CRYPTO_HDKEY.get_tag()))?;
            CryptoHDKey::encode(transparent, e, _ctx)
        }

        e.int(Int::from(ORCHARD))?
            .tag(Tag::Unassigned(ZCASH_FULL_VIEWING_KEY.get_tag()))?;
        ZcashFullViewingKey::encode(&self.orchard, e, _ctx);

        if let Some(name) = &self.name {
            e.int(Int::from(NAME))?.str(name)
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashUnifiedFullViewingKey {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashUnifiedFullViewingKey::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                TRANSPARENT => {
                    d.tag()?;
                    obj.transparent = Some(CryptoHDKey::decode(d, ctx)?);
                }
                ORCHARD => {
                    d.tag()?;
                    obj.orchard = ZcashFullViewingKey::decode(d, ctx)
                }
                NAME => {
                    obj.name = Some(d.str()?.to_string());
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
