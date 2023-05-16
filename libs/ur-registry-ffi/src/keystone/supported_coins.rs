use anyhow::{format_err, Error};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum SupportedChains {
    LTC,
    DASH,
    BCH,
}

impl SupportedChains {
    fn as_str(&self) -> &'static str {
        match self {
            SupportedChains::LTC => "LTC",
            SupportedChains::DASH => "DASH",
            SupportedChains::BCH => "BCH",
        }
    }
}

impl From<SupportedChains> for String {
    fn from(value: SupportedChains) -> Self {
        value.as_str().to_string()
    }
}

impl TryInto<SupportedChains> for i32 {
    type Error = Error;

    fn try_into(self) -> Result<SupportedChains, Self::Error> {
        match self {
            2 => Ok(SupportedChains::LTC),
            5 => Ok(SupportedChains::DASH),
            145 => Ok(SupportedChains::BCH),
            _ => Err(format_err!("")),
        }
    }
}
