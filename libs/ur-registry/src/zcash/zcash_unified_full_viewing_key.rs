use alloc::string::{String, ToString};
use minicbor::data::Int;

use crate::{
    cbor::cbor_map,
    impl_template_struct,
    registry_types::{RegistryType, ZCASH_UNIFIED_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
};

const UFVK: u8 = 1;
const INDEX: u8 = 2;
const NAME: u8 = 3;

impl_template_struct!(ZcashUnifiedFullViewingKey {
    ufvk: String,
    index: u32,
    name: Option<String>
});

impl MapSize for ZcashUnifiedFullViewingKey {
    fn map_size(&self) -> u64 {
        let mut size = 2;
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

        e.int(Int::from(UFVK))?.str(&self.ufvk)?;
        e.int(Int::from(INDEX))?.u32(self.index)?;

        if let Some(name) = &self.name {
            e.int(Int::from(NAME))?.str(name)?;
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
                UFVK => {
                    obj.ufvk = d.str()?.to_string();
                }
                INDEX => {
                    obj.index = d.u32()?;
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
