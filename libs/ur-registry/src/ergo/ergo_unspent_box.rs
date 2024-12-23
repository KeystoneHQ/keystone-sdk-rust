use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};
use crate::cbor::{cbor_array, cbor_map};
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, ERGO_ASSET, ERGO_UNSPENT_BOX};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};

const BOX_ID: u8 = 1;
const VALUE: u8 = 2;
const ERGO_TREE: u8 = 3;
const ASSETS: u8 = 4;

impl_template_struct!(ErgoUnspentBox {
    box_id: String,
    value: u64,
    ergo_tree: String,
    assets: Option<Vec<ErgoAsset>>
});

impl MapSize for ErgoUnspentBox {
    fn map_size(&self) -> u64 {
        let mut size = 3;
        if self.assets.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for ErgoUnspentBox{
    fn get_registry_type() -> RegistryType<'static> {
        ERGO_UNSPENT_BOX
    }
}

impl<C> minicbor::Encode<C> for ErgoUnspentBox {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(BOX_ID))?.str(&self.box_id)?;

        e.int(Int::from(VALUE))?.u64(self.get_value())?;

        e.int(Int::from(ERGO_TREE))?.str(&self.ergo_tree)?;

        if let Some(assets) = &self.assets {
            e.int(Int::from(ASSETS))?;
            e.array(assets.len() as u64)?;
            for asset in assets {
                e.tag(Tag::Unassigned(ErgoAsset::get_registry_type().get_tag()))?;
                ErgoAsset::encode(asset, e, _ctx)?;
            }
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ErgoUnspentBox {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut ergo_unspent_box = ErgoUnspentBox::default();
        cbor_map(d, &mut ergo_unspent_box, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                BOX_ID => obj.set_box_id(d.str()?.to_string()),
                VALUE => obj.set_value(d.u64()?),
                ERGO_TREE => obj.set_ergo_tree(d.str()?.to_string()),
                ASSETS => {
                    cbor_array(d, obj, |_index, obj, d| {
                        let tag = d.tag()?;
                        if let Tag::Unassigned(n) = tag {
                            if n == ERGO_ASSET.get_tag() {
                                match &mut obj.assets {
                                    Some(assets) => assets.push(ErgoAsset::decode(d, _ctx)?),
                                    None => {
                                        obj.assets = Some(vec![ErgoAsset::decode(d, _ctx)?]);
                                    }
                                }
                            }
                        }
                        Ok(())
                    })?;
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(ergo_unspent_box)
    }
}

impl To for ErgoUnspentBox {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

impl FromCbor<ErgoUnspentBox> for ErgoUnspentBox {

    fn from_cbor(bytes: Vec<u8>) -> URResult<ErgoUnspentBox> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

const TOKEN_ID: u8 = 1;
const AMOUNT: u8 = 2;

impl_template_struct!(ErgoAsset {
    token_id: String,
    amount: u64
});

impl MapSize for ErgoAsset {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for ErgoAsset{
    fn get_registry_type() -> RegistryType<'static> {
        ERGO_ASSET
    }
}

impl<C> minicbor::Encode<C> for ErgoAsset {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, _ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(TOKEN_ID))?.str(&self.token_id)?;

        e.int(Int::from(AMOUNT))?.u64(self.amount)?;

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ErgoAsset {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut ergo_asset = ErgoAsset::default();
        cbor_map(d, &mut ergo_asset, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                TOKEN_ID => obj.set_token_id(d.str()?.to_string()),
                AMOUNT => obj.set_amount(d.u64()?),
                _ => {}
            }
            Ok(())
        })?;
        Ok(ergo_asset)
    }
}
