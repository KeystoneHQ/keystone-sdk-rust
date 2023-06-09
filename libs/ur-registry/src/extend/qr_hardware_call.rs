use crate::extend::key_derivation::KeyDerivationCall;
use crate::extend::qr_hardware_call::CallType::KeyDerivation;
use crate::impl_template_struct;
use alloc::format;
use alloc::string::{String, ToString};
use minicbor::data::{Int, Tag};
use minicbor::encode::{Error, Write};
use minicbor::{Decoder, Encoder};
use crate::cbor::cbor_map;
use crate::error::URError;
use crate::error::URError::CborDecodeError;
use crate::registry_types::{KEY_DERIVATION_CALL, QR_HARDWARE_CALL, RegistryType};
use crate::traits::{MapSize, RegistryItem};

const CALL_TYPE: u8 = 1;
const PARAMS: u8 = 2;
const ORIGIN: u8 = 3;

#[derive(Clone, Debug, Default)]
pub enum CallType {
    #[default]
    KeyDerivation = 0,
}

impl TryFrom<u32> for CallType {
    type Error = URError;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(KeyDerivation),
            _ => Err(CborDecodeError(format!("QRHardwareCall: invalid call type {}", value)))
        }
    }
}

#[derive(Debug, Clone)]
pub enum CallParams {
    KeyDerivation(KeyDerivationCall),
}

impl Default for CallParams {
    fn default() -> Self {
        Self::KeyDerivation(KeyDerivationCall::default())
    }
}

impl_template_struct!(QRHardwareCall {
    call_type: CallType,
    params: CallParams,
    origin: Option<String>
});

impl RegistryItem for QRHardwareCall {
    fn get_registry_type() -> RegistryType<'static> {
        QR_HARDWARE_CALL
    }
}

impl MapSize for QRHardwareCall {
    fn map_size(&self) -> u64 {
        match self.origin {
            Some(_) => 3,
            None => 2
        }
    }
}

impl<C> minicbor::Encode<C> for QRHardwareCall {
    fn encode<W: Write>(&self, e: &mut Encoder<W>, ctx: &mut C) -> Result<(), Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(CALL_TYPE))?
            .int(Int::from(self.call_type.clone() as u32))?;

        e.int(Int::from(PARAMS))?;
        match &self.params {
            CallParams::KeyDerivation(k) => {
                e.tag(Tag::Unassigned(KEY_DERIVATION_CALL.get_tag()))?;
                k.encode(e, ctx)?;
            }
        }

        if let Some(origin) = self.get_origin() {
            e.int(Int::from(ORIGIN))?
                .str(&origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for QRHardwareCall {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = QRHardwareCall::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                CALL_TYPE => {
                    let call_type = CallType::try_from(
                        u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?)
                        .map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
                    obj.set_call_type(call_type)
                }
                PARAMS => {
                    let tag = d.tag()?;
                    if let Tag::Unassigned(tag) = tag {
                        if tag.eq(&KEY_DERIVATION_CALL.get_tag()) {
                            obj.set_params(CallParams::KeyDerivation(KeyDerivationCall::decode(d, ctx)?));
                            return Ok(());
                        }
                    }
                    return Err(minicbor::decode::Error::message(format!("invalid QRHardwareCall params")))
                }
                ORIGIN => {
                    let origin = d.str()?.to_string();
                    obj.set_origin(Some(origin))
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
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::crypto_key_path::{CryptoKeyPath, PathComponent};
    use crate::extend::key_derivation::KeyDerivationCall;
    use crate::extend::key_derivation_schema::{Curve, DerivationAlgo, KeyDerivationSchema};
    use crate::extend::qr_hardware_call::{CallParams, CallType, QRHardwareCall};

    #[test]
    fn test_encode() {
        let key_path1 = CryptoKeyPath::new(
            vec![
                PathComponent::new(Some(44), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
            ],
            None,
            None,
        );
        let schema1 = KeyDerivationSchema::new(key_path1, None, None);
        let key_path2 = CryptoKeyPath::new(
            vec![
                PathComponent::new(Some(44), true).unwrap(),
                PathComponent::new(Some(501), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
            ],
            None,
            None,
        );
        let schema2 = KeyDerivationSchema::new(key_path2, Some(Curve::Ed25519), None);
        let schemas = vec![schema1, schema2];
        let call = QRHardwareCall::new(CallType::KeyDerivation, CallParams::KeyDerivation(
            KeyDerivationCall::new(schemas)
        ), None);
        let bytes: Vec<u8> = call.try_into().unwrap();
        assert_eq!("a2010002d90515a10182d90516a101d90130a10186182cf500f500f5d90516a201d90130a1018a182cf51901f5f500f500f500f50201", hex::encode(bytes));
    }

    #[test]
    fn test_decode() {
        let ur_bytes = hex::decode("a2010002d90515a10182d90516a101d90130a10186182cf500f500f5d90516a201d90130a1018a182cf51901f5f500f500f500f50201").unwrap();
        let call = QRHardwareCall::try_from(ur_bytes).unwrap();
        assert_eq!(CallType::KeyDerivation as u32, call.get_call_type() as u32);
        assert_eq!(None, call.get_origin());
        let params = call.get_params();
        match params {
            CallParams::KeyDerivation(k) => {
                let schemas = k.get_schemas();
                assert_eq!(2, schemas.len());
                let schema1 = schemas.get(0).unwrap();
                assert_eq!("44'/0'/0'", schema1.get_key_path().get_path().unwrap());
                assert_eq!(Curve::Secp256k1 as u32, schema1.get_curve_or_default() as u32);
                assert_eq!(DerivationAlgo::Slip10 as u32, schema1.get_algo_or_default() as u32);
                let schema2 = schemas.get(1).unwrap();
                assert_eq!("44'/501'/0'/0'/0'", schema2.get_key_path().get_path().unwrap());
                assert_eq!(Curve::Ed25519 as u32, schema2.get_curve_or_default() as u32);
                assert_eq!(DerivationAlgo::Slip10 as u32, schema2.get_algo_or_default() as u32);
            }
        }
    }

    #[test]
    fn test_encode2() {
        let key_path1 = CryptoKeyPath::new(
            vec![
                PathComponent::new(Some(1852), true).unwrap(),
                PathComponent::new(Some(1815), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
            ],
            None,
            None,
        );
        let schema1 = KeyDerivationSchema::new(key_path1, Some(Curve::Ed25519), Some(DerivationAlgo::Bip32Ed25519));
        let key_path2 = CryptoKeyPath::new(
            vec![
                PathComponent::new(Some(1852), true).unwrap(),
                PathComponent::new(Some(1815), true).unwrap(),
                PathComponent::new(Some(1), true).unwrap(),
            ],
            None,
            None,
        );
        let schema2 = KeyDerivationSchema::new(key_path2, Some(Curve::Ed25519), Some(DerivationAlgo::Bip32Ed25519));
        let schemas = vec![schema1, schema2];
        let call = QRHardwareCall::new(CallType::KeyDerivation, CallParams::KeyDerivation(
            KeyDerivationCall::new(schemas)
        ), None);
        let bytes: Vec<u8> = call.try_into().unwrap();
        assert_eq!("a2010002d90515a10182d90516a301d90130a1018619073cf5190717f500f502010301d90516a301d90130a1018619073cf5190717f501f502010301", hex::encode(bytes));
    }
}