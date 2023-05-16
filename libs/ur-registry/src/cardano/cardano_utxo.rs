use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_UTXO, CRYPTO_KEYPATH};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const TRANSACTION_HASH: u8 = 1;
const INDEX: u8 = 2;
const AMOUNT: u8 = 3;
const KEY_PATH: u8 = 4;
const ADDRESS: u8 = 5;

impl_template_struct!(CardanoUTXO {
    transaction_hash: Bytes,
    index: u32,
    amount: u64,
    key_path: CryptoKeyPath,
    address: String
});

impl MapSize for CardanoUTXO {
    fn map_size(&self) -> u64 {
        5
    }
}

impl RegistryItem for CardanoUTXO {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_UTXO
    }
}

impl<C> minicbor::Encode<C> for CardanoUTXO {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(TRANSACTION_HASH))?
            .bytes(&self.get_transaction_hash())?;

        e.int(Int::from(INDEX))?.u32(self.get_index())?;

        e.int(Int::from(AMOUNT))?.u64(self.get_amount())?;

        e.int(Int::from(KEY_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;

        CryptoKeyPath::encode(&self.key_path, e, _ctx)?;

        e.int(Int::from(ADDRESS))?.str(&self.address)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoUTXO {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut cardano_utxo = CardanoUTXO::default();
        cbor_map(d, &mut cardano_utxo, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                TRANSACTION_HASH => obj.set_transaction_hash(d.bytes()?.to_vec()),
                INDEX => obj.set_index(d.u32()?),
                AMOUNT => obj.set_amount(d.u64()?),
                KEY_PATH => {
                    d.tag()?;
                    obj.set_key_path(CryptoKeyPath::decode(d, _ctx)?);
                }
                ADDRESS => {
                    obj.set_address(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(cardano_utxo)
    }
}

impl To for CardanoUTXO {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl FromCbor<CardanoUTXO> for CardanoUTXO {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoUTXO> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}
