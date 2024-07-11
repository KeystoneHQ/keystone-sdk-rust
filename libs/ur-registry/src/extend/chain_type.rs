use crate::error::URError;
use alloc::format;
use alloc::string::{String, ToString};
use core::fmt::Display;

/// https://github.com/satoshilabs/slips/blob/master/slip-0044.md
/// https://github.com/cosmos/chain-registry

// todo add more chain type
#[derive(Clone, Debug, Default)]
pub enum ChainType {
    #[default]
    BTC,
    ETH,
    LTC,
    SOL,
    // cosmos
    COSMOS,
    SCRT,
    CRO,
    IOV,
    BLD,
    KAVA,
    TERRA,
    EVOMS,
}

impl TryFrom<String> for ChainType {
    type Error = URError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let value = value.as_str();
        match value {
            "BTC" => Ok(Self::BTC),
            "ETH" => Ok(Self::ETH),
            "LTC" => Ok(Self::LTC),
            "SOL" => Ok(Self::SOL),
            "COSMOS" => Ok(Self::COSMOS),
            "SCRT" => Ok(Self::SCRT),
            "CRO" => Ok(Self::CRO),
            "IOV" => Ok(Self::IOV),
            "BLD" => Ok(Self::BLD),
            "KAVA" => Ok(Self::KAVA),
            "TERRA" => Ok(Self::TERRA),
            "EVOMS" => Ok(Self::EVOMS),
            _ => Err(URError::CborDecodeError(format!(
                "KeyDerivationSchema: invalid chain type {}",
                value
            ))),
        }
    }
}
impl Display for ChainType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let str = match self {
            ChainType::BTC => "BTC".to_string(),
            ChainType::ETH => "ETH".to_string(),
            ChainType::LTC => "LTC".to_string(),
            ChainType::SOL => "SOL".to_string(),
            ChainType::COSMOS => "COSMOS".to_string(),
            ChainType::SCRT => "SCRT".to_string(),
            ChainType::CRO => "CRO".to_string(),
            ChainType::IOV => "IOV".to_string(),
            ChainType::BLD => "BLD".to_string(),
            ChainType::KAVA => "KAVA".to_string(),
            ChainType::TERRA => "TERRA".to_string(),
            ChainType::EVOMS => "EVOMS".to_string(),
            _ => "UnSupport Chain Type".to_string(),
        };
        write!(f, "{}", str)
    }
}
