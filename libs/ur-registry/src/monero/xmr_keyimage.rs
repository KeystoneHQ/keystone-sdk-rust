use crate::{
  impl_template_struct,
  registry_types::{RegistryType, XMR_KEYIMAGE},
  traits::RegistryItem,
  types::Bytes,
};

impl_template_struct!(XmrKeyImage {
    payload: Bytes
});

impl RegistryItem for XmrKeyImage {
    fn get_registry_type() -> RegistryType<'static> {
        XMR_KEYIMAGE
    }
}

impl<'b, C> minicbor::Decode<'b, C> for XmrKeyImage {
    fn decode(
        _: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        Ok(XmrKeyImage::default())
    }
}

impl<C> minicbor::Encode<C> for XmrKeyImage {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.payload)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec::Vec;

    #[test]
    pub fn test_encode() {
        let payload = hex::decode("aabbccdd").unwrap();
        let key_image = XmrKeyImage::new(payload);

        assert_eq!(
            hex::encode::<Vec<u8>>(key_image.try_into().unwrap()),
            "44aabbccdd"
        );
    }
}