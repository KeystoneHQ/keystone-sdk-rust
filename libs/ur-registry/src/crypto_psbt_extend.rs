use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_PSBT_EXTEND};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const PSBT: u8 = 1;
const COIN_ID: u8 = 2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SupportedPsbtCoin {
    Bitcoin,
    Litecoin,
    Dogecoin,
    Dash,
    BitcoinCash,
}

impl SupportedPsbtCoin {
    pub fn from_coin_id(coin_id: Option<i128>) -> Self {
        match coin_id {
            Some(2) => SupportedPsbtCoin::Litecoin,
            Some(3) => SupportedPsbtCoin::Dogecoin,
            Some(4) => SupportedPsbtCoin::Dash,
            Some(145) => SupportedPsbtCoin::BitcoinCash,
            _ => SupportedPsbtCoin::Bitcoin,
        }
    }

    pub fn is_valid_coin_id(coin_id: i128) -> bool {
        matches!(coin_id, 2 | 3 | 4 | 145)
    }
}

#[derive(Debug, Clone, Default)]
pub struct CryptoPSBTExtend {
    psbt: Bytes,
    coin_id: Option<i128>,
}

impl CryptoPSBTExtend {
    fn is_valid_coin_id(coin_id: i128) -> bool {
        SupportedPsbtCoin::is_valid_coin_id(coin_id)
    }
    
    pub fn new(psbt: Bytes, coin_id: Option<i128>) -> Result<Self, URError> {
        if coin_id.is_some() && !Self::is_valid_coin_id(coin_id.unwrap()) {
            return Err(URError::NotSupportURTypeError("invalid coin_id".to_string()));
        }
        Ok(CryptoPSBTExtend { psbt, coin_id })
    }

    pub fn get_psbt(&self) -> Bytes {
        self.psbt.clone()
    }

    pub fn get_coin_id(&self) -> Option<i128> {
        self.coin_id
    }

    pub fn set_psbt(&mut self, psbt: Bytes) {
        self.psbt = psbt;
    }

    pub fn set_coin_id(&mut self, coin_id: i128) {
        self.coin_id = Some(coin_id);
    }
}

impl RegistryItem for CryptoPSBTExtend {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_PSBT_EXTEND
    }
}

impl<C> minicbor::Encode<C> for CryptoPSBTExtend {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size: u64 = 1;
        if self.coin_id.is_some() {
            size += 1;
        }
        e.map(size)?;
        e.int(Int::from(PSBT))?.bytes(&self.psbt)?;
        if let Some(coin_id) = self.coin_id {
            e.int(Int::from(COIN_ID))?.int(
                Int::try_from(coin_id)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CryptoPSBTExtend {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoPSBTExtend::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                PSBT => {
                    obj.psbt = d.bytes()?.to_vec();
                }
                COIN_ID => {
                    obj.coin_id = Some(i128::from(d.int()?));
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CryptoPSBTExtend {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoPSBTExtend> for CryptoPSBTExtend {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CryptoPSBTExtend> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::crypto_psbt_extend::CryptoPSBTExtend;
    use crate::traits::RegistryItem;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_psbt_extend_encode() {
        let crypto = CryptoPSBTExtend::new(
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap(),
            Some(2),
        ).unwrap();
        let result: Vec<u8> = crypto.try_into().unwrap();
        assert_eq!(
            "A20158208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA0202",
            hex::encode::<Vec<u8>>(result.clone()).to_uppercase()
        );

        let ur = ur::encode(&result, CryptoPSBTExtend::get_registry_type().get_type());
        assert_eq!(ur, "ur:crypto-psbt-extend/oeadhdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkaoaoveprfefr");

        let ur = ur::decode(ur.as_str()).unwrap();
        assert_eq!(ur.1, result);
    }

    #[test]
    fn test_psbt_extend_decode() {
        let bytes =
            Vec::from_hex("A20158208C05C4B4F3E88840A4F4B5F155CFD69473EA169F3D0431B7A6787A23777F08AA0201")
                .unwrap();
        let crypto = CryptoPSBTExtend::try_from(bytes).unwrap();
        assert_eq!(
            crypto.get_psbt(),
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap()
        );
        assert_eq!(crypto.get_coin_id(), Some(1));
    }
}
