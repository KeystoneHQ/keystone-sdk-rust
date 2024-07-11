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

impl ChainType {
    pub fn check_path_is_match_chain_type(&self, path: &str) -> bool {
        // convert path to lowercase
        let path = path.to_lowercase();
        match self {
            ChainType::BTC => path.starts_with("m/44'/0'"),

            ChainType::ETH => path.starts_with("m/44'/60'"),
            ChainType::LTC => path.starts_with("m/44'/2'"),
            ChainType::SOL => path.starts_with("m/44'/501'"),
            ChainType::COSMOS => path.starts_with("m/44'/118'"),
            ChainType::SCRT => path.starts_with("m/44'/529'"),
            ChainType::CRO => path.starts_with("m/44'/394'"),
            ChainType::IOV => path.starts_with("m/44'/234'"),
            ChainType::BLD => path.starts_with("m/44'/564'"),
            ChainType::KAVA => path.starts_with("m/44'/459'"),
            ChainType::TERRA => path.starts_with("m/44'/330'"),
            ChainType::EVOMS => path.starts_with("m/44'/60'"),
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_check_path_is_match_chain_type() {
        let path = "m/44'/60'/0'/0/0";
        assert_eq!(
            true,
            super::ChainType::ETH.check_path_is_match_chain_type(path)
        );
        assert_eq!(
            true,
            super::ChainType::EVOMS.check_path_is_match_chain_type(path)
        );
        assert_eq!(
            false,
            super::ChainType::BTC.check_path_is_match_chain_type(path)
        );
    }
}
