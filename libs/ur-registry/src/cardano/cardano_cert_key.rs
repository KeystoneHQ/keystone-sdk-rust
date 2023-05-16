use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::CRYPTO_KEYPATH;
use crate::traits::{From as FromCbor, MapSize, To};
use crate::types::Bytes;

use alloc::string::ToString;
use alloc::vec::Vec;
use core::convert::From;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const KEY_HASH: u8 = 1;
const KEY_PATH: u8 = 2;

use crate::impl_template_struct;

impl_template_struct!(CardanoCertKey {
    key_hash: Bytes,
    key_path: CryptoKeyPath
});

impl MapSize for CardanoCertKey {
    fn map_size(&self) -> u64 {
        2
    }
}

impl<C> minicbor::Encode<C> for CardanoCertKey {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(KEY_HASH))?.bytes(&self.get_key_hash())?;

        e.int(Int::from(KEY_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        &self.key_path.encode(e, _ctx)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoCertKey {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_cert_key = CardanoCertKey::default();
        cbor_map(d, &mut cardano_cert_key, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                KEY_HASH => {
                    obj.set_key_hash(d.bytes()?.to_vec());
                }
                KEY_PATH => {
                    d.tag()?;
                    obj.set_key_path(CryptoKeyPath::decode(d, _ctx)?);
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(cardano_cert_key)
    }
}

impl To for CardanoCertKey {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl FromCbor<CardanoCertKey> for CardanoCertKey {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoCertKey> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
