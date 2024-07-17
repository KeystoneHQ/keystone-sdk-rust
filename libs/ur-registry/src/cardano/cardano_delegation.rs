use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_DELEGSTION, CRYPTO_KEYPATH};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const PUBKEY: u8 = 1;
const WEIDTH: u8 = 2;

impl_template_struct!(CardanoDelegation {
    pub_key: Bytes,
    weidth: u8
});

impl MapSize for CardanoDelegation {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for CardanoDelegation {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_DELEGSTION
    }
}

impl<C> minicbor::Encode<C> for CardanoDelegation {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(PUBKEY))?.bytes(&self.pub_key)?;

        e.int(Int::from(WEIDTH))?.u8(self.weidth)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoDelegation {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result: CardanoDelegation = CardanoDelegation::default();
        cbor_map(d, &mut result, |key, obj, d: &mut Decoder| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                PUBKEY => {
                    obj.set_pub_key(d.bytes()?.to_vec());
                }
                WEIDTH => {
                    obj.weidth = d.u8()?;
                }
                _ => {
                    d.skip()?;
                }
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CardanoDelegation {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl FromCbor<CardanoDelegation> for CardanoDelegation {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoDelegation> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
