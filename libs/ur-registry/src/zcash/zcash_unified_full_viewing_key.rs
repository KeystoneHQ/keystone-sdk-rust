//! Zcash Unified Full Viewing Key Registry Type
//!
//! This module implements the CBOR encoding and decoding for Zcash Unified Full Viewing Keys (UFVKs).
//! It represents a viewing key that can be used to see incoming and outgoing transactions
//! without having spending authority.
//!
//! The structure follows the UR Registry Type specification for Zcash UFVKs,
//! with a map containing:
//! - UFVK: The unified full viewing key string
//! - Index: The account index
//! - Name: An optional account name (if provided)
//!

use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use minicbor::data::Int;

use crate::{
    cbor::cbor_map,
    error::{URError, URResult},
    impl_template_struct,
    registry_types::{RegistryType, ZCASH_UNIFIED_FULL_VIEWING_KEY},
    traits::{MapSize, RegistryItem},
};

use super::cbor_helpers::{reject_duplicate_key, require_key};

const UFVK: u8 = 1;
const INDEX: u8 = 2;
const NAME: u8 = 3;

impl_template_struct!(ZcashUnifiedFullViewingKey {
    ufvk: String,
    index: u32,
    name: Option<String>
});

impl MapSize for ZcashUnifiedFullViewingKey {
    fn map_size(&self) -> u64 {
        let mut size = 2;
        if self.name.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for ZcashUnifiedFullViewingKey {
    fn get_registry_type() -> RegistryType<'static> {
        ZCASH_UNIFIED_FULL_VIEWING_KEY
    }
}

impl<C> minicbor::Encode<C> for ZcashUnifiedFullViewingKey {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(UFVK))?.str(&self.ufvk)?;
        e.int(Int::from(INDEX))?.u32(self.index)?;

        if let Some(name) = &self.name {
            e.int(Int::from(NAME))?.str(name)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ZcashUnifiedFullViewingKey {
    fn decode(
        d: &mut minicbor::Decoder<'b>,
        _ctx: &mut C,
    ) -> Result<Self, minicbor::decode::Error> {
        let mut result = ZcashUnifiedFullViewingKey::default();
        let mut seen_keys = Vec::new();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            reject_duplicate_key(&mut seen_keys, key, d, "duplicate key in zcash-ufvk map")?;
            match key {
                UFVK => {
                    obj.ufvk = d.str()?.to_string();
                }
                INDEX => {
                    obj.index = d.u32()?;
                }
                NAME => {
                    obj.name = Some(d.str()?.to_string());
                }
                _ => {
                    d.skip()?;
                }
            }
            Ok(())
        })?;
        require_key(&seen_keys, UFVK, d, "missing zcash-ufvk ufvk")?;
        require_key(&seen_keys, INDEX, d, "missing zcash-ufvk index")?;
        Ok(result)
    }
}

impl TryFrom<Vec<u8>> for ZcashUnifiedFullViewingKey {
    type Error = URError;

    fn try_from(value: Vec<u8>) -> URResult<Self> {
        let mut decoder = minicbor::Decoder::new(&value);
        let ufvk =
            <ZcashUnifiedFullViewingKey as minicbor::Decode<'_, ()>>::decode(&mut decoder, &mut ())
                .map_err(|e| URError::CborDecodeError(e.to_string()))?;
        if decoder.position() != value.len() {
            return Err(URError::CborDecodeError(
                "trailing data after zcash-ufvk".to_string(),
            ));
        }
        Ok(ufvk)
    }
}

impl TryInto<Vec<u8>> for ZcashUnifiedFullViewingKey {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_zcash_unified_full_viewing_key_encode_decode() {
        let ufvk = ZcashUnifiedFullViewingKey {
            ufvk: "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            index: 0,
            name: Some("Keystone".to_string()),
        };

        let cbor = minicbor::to_vec(&ufvk).unwrap();
        let decoded: ZcashUnifiedFullViewingKey = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.ufvk, ufvk.ufvk);
        assert_eq!(decoded.index, ufvk.index);
        assert_eq!(decoded.name, ufvk.name);
    }

    #[test]
    fn test_zcash_unified_full_viewing_key_without_name() {
        let ufvk = ZcashUnifiedFullViewingKey {
            ufvk: "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            index: 1,
            name: None,
        };

        let cbor = minicbor::to_vec(&ufvk).unwrap();
        let decoded: ZcashUnifiedFullViewingKey = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.ufvk, ufvk.ufvk);
        assert_eq!(decoded.index, ufvk.index);
        assert_eq!(decoded.name, None);
    }

    #[test]
    fn test_zcash_unified_full_viewing_key_rejects_missing_required_keys() {
        for (cbor_hex, message) in [
            ("a10200", "missing zcash-ufvk ufvk"),
            ("a1016175", "missing zcash-ufvk index"),
        ] {
            let cbor = hex::decode(cbor_hex).unwrap();

            let result: Result<ZcashUnifiedFullViewingKey, _> = minicbor::decode(&cbor);

            assert!(result.unwrap_err().to_string().contains(message));
        }
    }

    #[test]
    fn test_zcash_unified_full_viewing_key_rejects_duplicate_keys() {
        let cbor = hex::decode("a30161750161760200").unwrap();

        let result: Result<ZcashUnifiedFullViewingKey, _> = minicbor::decode(&cbor);

        assert!(result
            .unwrap_err()
            .to_string()
            .contains("duplicate key in zcash-ufvk map"));
    }

    #[test]
    fn test_zcash_unified_full_viewing_key_try_from_rejects_trailing_data() {
        let mut cbor = hex::decode("a20161750200").unwrap();
        cbor.push(0x00);

        let err = ZcashUnifiedFullViewingKey::try_from(cbor).unwrap_err();

        assert!(err.to_string().contains("trailing data"));
    }

    #[test]
    fn test_zcash_unified_full_viewing_key_skips_unknown_keys() {
        let cbor = hex::decode("a301617502000982010a").unwrap();

        let decoded: ZcashUnifiedFullViewingKey = minicbor::decode(&cbor).unwrap();

        assert_eq!(decoded.ufvk, "u");
        assert_eq!(decoded.index, 0);
    }

    #[test]
    fn test_map_size_with_name() {
        let ufvk = ZcashUnifiedFullViewingKey {
            ufvk: "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            index: 0,
            name: Some("Keystone".to_string()),
        };

        assert_eq!(ufvk.map_size(), 3);
    }

    #[test]
    fn test_map_size_without_name() {
        let ufvk = ZcashUnifiedFullViewingKey {
            ufvk: "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl".to_string(),
            index: 0,
            name: None,
        };

        assert_eq!(ufvk.map_size(), 2);
    }

    #[test]
    fn test_registry_type() {
        assert_eq!(
            ZcashUnifiedFullViewingKey::get_registry_type().get_type(),
            "zcash-unified-full-viewing-key"
        );
    }

    #[test]
    fn test_new_constructor() {
        let ufvk_str = "uview1qqqqqqqqqqqqqq8rzd0efkm6ej5n0twzum9czt9kj5y7jxjm9qz3uq9qgpqqqqqqqqqqqqqq9en0hkucteqncqcfqcqcpz4wuwl";
        let index = 5;
        let name = "Keystone 1";

        let ufvk =
            ZcashUnifiedFullViewingKey::new(ufvk_str.to_string(), index, Some(name.to_string()));

        assert_eq!(ufvk.get_ufvk(), ufvk_str);
        assert_eq!(ufvk.get_index(), index);
        assert_eq!(ufvk.get_name(), Some(name.to_string()));
    }
}
