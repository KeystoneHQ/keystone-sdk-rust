use crate::registry_types::{RegistryType, XMR_TXUNSIGNED};
use crate::traits::RegistryItem;
use crate::types::Bytes;
use crate::impl_template_struct;
use minicbor::data::Type;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

impl_template_struct!(XmrTxUnsigned {
    payload: Bytes
});

impl RegistryItem for XmrTxUnsigned {
    fn get_registry_type() -> RegistryType<'static> {
        XMR_TXUNSIGNED
    }
}

impl<'b, C> minicbor::Decode<'b, C> for XmrTxUnsigned {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        match d.datatype()? {
            Type::Bytes => {
                Ok(XmrTxUnsigned::new(d.bytes()?.to_vec()))
            }
            _ => Err(minicbor::decode::Error::message("Invalid datatype for XmrTxUnsigned")),
        }
    }
}

impl<C> minicbor::Encode<C> for XmrTxUnsigned {
    fn encode<W: Write>(
        &self,
        _: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_decode() {
        let ur = hex::decode("590002aaff").unwrap();

        let result = XmrTxUnsigned::try_from(ur.clone()).unwrap();
        assert_eq!(hex::encode(result.payload), "aaff");
    }
}
