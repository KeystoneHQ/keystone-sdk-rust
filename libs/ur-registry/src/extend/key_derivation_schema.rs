use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::URError;
use crate::extend::chain_type::ChainType;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, KEY_DERIVATION_SCHEMA};
use crate::traits::{MapSize, RegistryItem};
use alloc::format;
use alloc::string::{String, ToString};
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};

const KEY_PATH: u8 = 1;
const CURVE: u8 = 2;
const ALGO: u8 = 3;
const CHAIN_TYPE: u8 = 4;

#[derive(Clone, Debug, Default)]
pub enum Curve {
    #[default]
    Secp256k1 = 0,
    Ed25519,
}

impl TryFrom<u32> for Curve {
    type Error = URError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Secp256k1),
            1 => Ok(Self::Ed25519),
            _ => Err(URError::CborDecodeError(format!(
                "KeyDerivationSchema: invalid curve type {}",
                value
            ))),
        }
    }
}

#[derive(Clone, Debug, Default)]
pub enum DerivationAlgo {
    #[default]
    Slip10 = 0,
    Bip32Ed25519,
}

impl TryFrom<u32> for DerivationAlgo {
    type Error = URError;
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Slip10),
            1 => Ok(Self::Bip32Ed25519),
            _ => Err(URError::CborDecodeError(format!(
                "KeyDerivationSchema: invalid algo type {}",
                value
            ))),
        }
    }
}

impl_template_struct!(KeyDerivationSchema {
    key_path: CryptoKeyPath,
    curve: Option<Curve>,
    algo: Option<DerivationAlgo>,
    chain_type: Option<ChainType>
});

impl KeyDerivationSchema {
    pub fn get_curve_or_default(&self) -> Curve {
        match self.get_curve() {
            Some(c) => c,
            None => Curve::Secp256k1,
        }
    }
    pub fn get_algo_or_default(&self) -> DerivationAlgo {
        match self.get_algo() {
            Some(a) => a,
            None => DerivationAlgo::Slip10,
        }
    }

    pub fn get_chain_type_or_default(&self) -> ChainType {
        self.get_chain_type()
            .unwrap_or_else(|| ChainType::default())
    }
}

impl RegistryItem for KeyDerivationSchema {
    fn get_registry_type() -> RegistryType<'static> {
        KEY_DERIVATION_SCHEMA
    }
}

impl MapSize for KeyDerivationSchema {
    fn map_size(&self) -> u64 {
        let mut size = 1;
        if let Some(_) = self.curve {
            size += 1;
        }
        if let Some(_) = self.algo {
            size += 1;
        }

        if let Some(_) = self.chain_type {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for KeyDerivationSchema {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(KEY_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        self.key_path.encode(e, _ctx)?;

        if let Some(curve) = &self.curve {
            e.int(Int::from(CURVE))?
                .int(Int::from(curve.clone() as u32))?;
        }

        if let Some(algo) = &self.algo {
            e.int(Int::from(ALGO))?
                .int(Int::from(algo.clone() as u32))?;
        }
        if let Some(chain_type) = &self.chain_type {
            let mut chain_type = chain_type.to_string();
            e.int(Int::from(CHAIN_TYPE))?.str(&chain_type)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for KeyDerivationSchema {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = KeyDerivationSchema::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                KEY_PATH => {
                    d.tag()?;
                    obj.set_key_path(CryptoKeyPath::decode(d, ctx)?);
                }
                CURVE => {
                    let curve = Curve::try_from(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                    obj.set_curve(Some(curve))
                }
                ALGO => {
                    let algo = DerivationAlgo::try_from(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                    obj.set_algo(Some(algo))
                }
                CHAIN_TYPE => {
                    let chain_type = ChainType::try_from(
                        String::try_from(d.str()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                    obj.set_chain_type(Some(chain_type))
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}
