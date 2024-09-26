use alloc::{
    format,
    string::{String, ToString},
};
use minicbor::data::{Int, Tag};

use crate::{
    cbor::cbor_map,
    crypto_key_path::CryptoKeyPath,
    impl_template_struct,
    registry_types::{
        RegistryType, CRYPTO_KEYPATH, TON_SIGN_REQUEST, UUID, ZCASH_ACCOUNTS,
        ZCASH_UNIFIED_FULL_VIEWING_KEY,
    },
    traits::{MapSize, RegistryItem},
    types::{Bytes, Fingerprint},
};

use super::zcash_unified_full_viewing_key::ZcashUnifiedFullViewingKey;

const MASTER_FINGERPRINT: u8 = 1;
const ACCOUNTS: u8 = 2;

impl_template_struct!(ZcashAccounts {
    master_fingerprint: Fingerprint,
    accounts: Vec<ZcashUnifiedFullViewingKey>
});

impl MapSize for ZcashAccounts {
    fn map_size(&self) -> u64 {
        2
    }
}

impl RegistryItem for ZcashAccounts {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_ACCOUNTS
    }
}

impl<C> minicbor::Encode<C> for ZcashAccounts {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(
            Int::try_from(MASTER_FINGERPRINT)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
        .int(
            Int::try_from(u32::from_be_bytes(self.master_fingerprint))
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?;

        e.int(Int::from(ACCOUNTS))?
            .array(self.accounts.len() as u64)?;
        for account in &self.accounts {
            e.tag(Tag::Unassigned(ZCASH_UNIFIED_FULL_VIEWING_KEY.get_tag()))?;
            ZcashUnifiedFullViewingKey::encode(account, e, ctx)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashAccounts {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashAccounts::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    obj.master_fingerprint = u32::to_be_bytes(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    );
                }
                ACCOUNTS => {
                    let mut keys: Vec<ZcashUnifiedFullViewingKey> = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        keys.push(ZcashUnifiedFullViewingKey::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.accounts = keys;
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
