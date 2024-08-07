use crate::cbor::cbor_map;
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, CARDANO_SIGN_DATA_REQUEST, CRYPTO_KEYPATH, UUID};
use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::Bytes;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::Encoder;

impl_template_struct!(CardanoVotingRegistration {
    delegations: Vec<(String, u8)>,
    stake_pub: String,
    payment_address: String,
    nonce: u64,
    voting_purpose: u8
});

impl CardanoVotingRegistration {
    pub fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(1)?.u32(61284)?.map(5)?;

        e.u8(1)?.array(self.delegations.len() as u64)?;
        for (addr, weight) in &self.delegations {
            e.array(2)?
                .bytes(&hex::decode(addr).unwrap())?
                .u8(*weight)?;
        }

        e.u8(2)?
            .bytes(&hex::decode(&self.stake_pub).unwrap())?
            .u8(3)?
            .bytes(&hex::decode(&self.payment_address).unwrap())?
            .u8(4)?
            .u64(self.nonce)?
            .u8(5)?
            .u8(self.voting_purpose)?;
        Ok(())
    }
}

impl TryInto<Vec<u8>> for CardanoVotingRegistration {
    type Error = URError;

    fn try_into(self) -> URResult<Vec<u8>> {
        let mut buf = Vec::new();
        let mut e = Encoder::new(&mut buf);
        match self.encode(&mut e) {
            Ok(_) => Ok(buf),
            Err(e) => Err(URError::CborDecodeError(e.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_cardano_voting_registration_encoding() {
        let registration = CardanoVotingRegistration {
            delegations: vec![
                ("0036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a0".to_string(), 1),
            ],
            stake_pub: "e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369".to_string(),
            payment_address: "004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9".to_string(),
            nonce: 1234,
            voting_purpose: 0,
        };

        let buf: Vec<u8> = registration.try_into().unwrap();

        assert_eq!(hex::encode(buf), "a119ef64a501818258200036ef3e1f0d3f5989e2d155ea54bdb2a72c4c456ccb959af4c94868f473f5a001025820e3cd2404c84de65f96918f18d5b445bcb933a7cda18eeded7945dd191e432369035839004777561e7d9ec112ec307572faec1aff61ff0cfed68df4cd5c847f1872b617657881e30ad17c46e4010c9cb3ebb2440653a34d32219c83e9041904d20500");
    }
}
