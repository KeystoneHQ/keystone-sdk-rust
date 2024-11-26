use alloc::string::ToString;
use minicbor::data::Int;

use crate::{
    cbor::cbor_map,
    impl_template_struct,
    registry_types::{RegistryType, ZCASH_PCZT},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

const DATA: u8 = 1;

impl_template_struct!(ZcashPczt { data: Bytes });

impl MapSize for ZcashPczt {
    fn map_size(&self) -> u64 {
        1
    }
}

impl RegistryItem for ZcashPczt {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_PCZT
    }
}

impl<C> minicbor::Encode<C> for ZcashPczt {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(DATA))?.bytes(&self.data)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashPczt {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashPczt::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                DATA => {
                    obj.data = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}
