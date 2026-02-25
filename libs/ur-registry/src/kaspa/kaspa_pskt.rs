use alloc::string::ToString;
use minicbor::data::Int;
use crate::{
    cbor::cbor_map,
    impl_template_struct,
    registry_types::{RegistryType, KASPA_PSKT},
    traits::{MapSize, RegistryItem},
    types::Bytes,
};

const PSKT: u8 = 1;

impl_template_struct!(KaspaPskt { pskt: Bytes });

impl MapSize for KaspaPskt {
    fn map_size(&self) -> u64 {
        1
    }
}

impl RegistryItem for KaspaPskt {
    fn get_registry_type() -> RegistryType<'static> {
        KASPA_PSKT
    }
}

impl<C> minicbor::Encode<C> for KaspaPskt {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(PSKT))?.bytes(&self.pskt)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for KaspaPskt {
    fn decode(d: &mut minicbor::Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = KaspaPskt::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key)
                .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                PSKT => {
                    obj.pskt = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_kaspa_pskt_encode_decode() {
        let data = vec![1, 2, 3, 4, 5];
        let pskt = KaspaPskt::new(data.clone());
        
        let cbor = minicbor::to_vec(&pskt).expect("Failed to encode");
        let decoded: KaspaPskt = minicbor::decode(&cbor).expect("Failed to decode");
        
        assert_eq!(decoded.get_pskt(), data);
    }
}