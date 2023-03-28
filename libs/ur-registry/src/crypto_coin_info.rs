use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::Int;
use crate::cbor::cbor_map;
use crate::error::{URError, UrResult};
use crate::registry_types::{CRYPTO_COIN_INFO, RegistryType};
use crate::traits::{From as FromCbor, RegistryItem, To};

const COIN_TYPE: u8 = 1;
const NETWORK: u8 = 2;

#[derive(Clone, Debug, PartialEq)]
pub enum CoinType {
    Bitcoin = 0,
}

impl CoinType {
    pub fn from_u32(i: u32) -> CoinType {
        match i {
            0 => CoinType::Bitcoin,
            _ => CoinType::Bitcoin,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Network {
    MainNet = 0,
    TestNet = 1,
}

impl Network {
    pub fn from_u32(i: u32) -> Network {
        match i {
            0 => Network::MainNet,
            1 => Network::TestNet,
            _ => Network::TestNet,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CryptoCoinInfo {
    coin_type: Option<CoinType>,
    network: Option<Network>,
}

impl CryptoCoinInfo {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_coin_type(&mut self, coin_type: CoinType) {
        self.coin_type = Some(coin_type)
    }

    pub fn set_network(&mut self, network: Network) {
        self.network = Some(network)
    }

    pub fn new(coin_type: Option<CoinType>, network: Option<Network>) -> CryptoCoinInfo {
        CryptoCoinInfo { coin_type, network }
    }
    pub fn get_coin_type(&self) -> CoinType {
        self.coin_type.clone().unwrap_or(CoinType::Bitcoin)
    }
    pub fn get_network(&self) -> Network {
        self.network.clone().unwrap_or(Network::MainNet)
    }
}


impl RegistryItem for CryptoCoinInfo {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_COIN_INFO
    }
}

impl<C> minicbor::Encode<C> for CryptoCoinInfo {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        _ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        let mut size = 0;
        if let Some(_coin_type) = &self.coin_type {
            size = size + 1;
        }
        if let Some(_network) = &self.network {
            size = size + 1;
        }
        e.map(size)?;
        if let Some(coin_type) = &self.coin_type {
            e.int(Int::from(COIN_TYPE))?.int(
                Int::from(coin_type.clone() as u8)
            )?;
        }

        if let Some(network) = &self.network {
            e.int(Int::from(NETWORK))?.int(
                Int::from(network.clone() as u8)
            )?;
        }
        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for CryptoCoinInfo {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoCoinInfo::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                COIN_TYPE => {
                    obj.coin_type = Some(CoinType::from_u32(
                        u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?
                    ));
                }
                NETWORK => {
                    obj.network = Some(Network::from_u32(
                        u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?
                    ));
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(result)
    }
}

impl To for CryptoCoinInfo {
    fn to_cbor(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}


impl FromCbor<CryptoCoinInfo> for CryptoCoinInfo {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<CryptoCoinInfo> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use hex::FromHex;
    use crate::crypto_coin_info::{CoinType, CryptoCoinInfo, Network};

    #[test]
    fn test_encode() {
        let crypto = CryptoCoinInfo::new(Some(CoinType::from_u32(0)), Some(Network::from_u32(1)));
        assert_eq!(
            "a201000201",
            hex::encode(crypto.to_cbor().unwrap()).to_lowercase()
        );

        let ur = ur::encode(&*(crypto.to_cbor().unwrap()), CryptoCoinInfo::get_registry_type().get_type());
        assert_eq!(ur, "ur:crypto-coin-info/oeadaeaoadehfdbany");
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a201000201",
        )
            .unwrap();
        let crypto = CryptoCoinInfo::from_cbor(bytes).unwrap();
        assert_eq!(crypto.get_network(), Network::TestNet);
        assert_eq!(crypto.get_coin_type(), CoinType::Bitcoin);
    }
}