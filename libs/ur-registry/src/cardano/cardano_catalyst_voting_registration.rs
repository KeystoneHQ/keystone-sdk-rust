use crate::cardano::cardano_delegation::CardanoDelegation;
use crate::cbor::{cbor_map, cbor_array};
use crate::crypto_key_path::CryptoKeyPath;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, CRYPTO_KEYPATH, CARDANO_CATALYST_VOTING_REGISTRATION, UUID};
use crate::traits::{From as FromCbor, RegistryItem, To, MapSize};
use crate::types::Bytes;
use alloc::format;
use crate::impl_template_struct;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const DELEGATIONS: u8 = 2;
const STAKE_PUB: u8 = 3;
const PAYMENT_ADDRESS: u8 = 4;
const NONCE: u8 = 5;
const VOTING_PURPOSE: u8 = 6;
const DERIVATION_PATH: u8 = 7;
const ORIGIN: u8 = 8;
const SIGN_TYPE: u8 = 9;

impl_template_struct!(CardanoCatalystVotingRegistrationRequest {
    request_id: Option<Bytes>,
    delegations: Vec<CardanoDelegation>,
    stake_pub: Bytes,
    payment_address: Bytes,
    nonce: u64,
    voting_purpose: u8,
    derivation_path: CryptoKeyPath,
    origin: Option<String>,
    sign_type: u8
});

impl MapSize for CardanoCatalystVotingRegistrationRequest {
    fn map_size(&self) -> u64 {
        let mut size = 7;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for CardanoCatalystVotingRegistrationRequest {
    fn get_registry_type() -> RegistryType<'static> {
        CARDANO_CATALYST_VOTING_REGISTRATION
    }
}

impl<C> minicbor::Encode<C> for CardanoCatalystVotingRegistrationRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(DELEGATIONS))?
            .array(self.delegations.len() as u64)?;
        for delegation in &self.delegations {
            delegation.encode(e, _ctx)?;
        }

        e.int(Int::from(STAKE_PUB))?.bytes(&self.stake_pub)?
            .int(Int::from(PAYMENT_ADDRESS))?.bytes(&self.payment_address)?
            .int(Int::from(NONCE))?.u64(self.nonce)?
            .int(Int::from(VOTING_PURPOSE))?.u8(self.voting_purpose)?
            .int(Int::from(DERIVATION_PATH))?;
        e.tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, _ctx)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        e.int(Int::from(SIGN_TYPE))?.u8(self.sign_type)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for CardanoCatalystVotingRegistrationRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result: CardanoCatalystVotingRegistrationRequest = CardanoCatalystVotingRegistrationRequest::default();
        cbor_map(d, &mut result, |key, obj, d: &mut Decoder| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.set_request_id(Some(d.bytes()?.to_vec()));
                }
                DELEGATIONS => {
                    cbor_array(d, &mut obj.delegations, |_index, array, d| {
                        d.tag()?;
                        let item = CardanoDelegation::decode(d, _ctx)?;
                        array.push(item);
                        Ok(())
                    })?;
                }
                STAKE_PUB => {
                    obj.set_stake_pub(d.bytes()?.to_vec());
                }
                PAYMENT_ADDRESS => {
                    obj.set_payment_address(d.bytes()?.to_vec());
                }
                NONCE => {
                    obj.nonce = d.u64()?;
                }
                VOTING_PURPOSE => {
                    obj.voting_purpose = d.u8()?;
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.set_derivation_path(CryptoKeyPath::decode(d, _ctx)?);
                }
                ORIGIN => {
                    obj.origin = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for CardanoCatalystVotingRegistrationRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CardanoCatalystVotingRegistrationRequest> for CardanoCatalystVotingRegistrationRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<CardanoCatalystVotingRegistrationRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_cardano_catalyst_voting_registration_request() {
        let cbor = hex::decode("a801d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0282d908a1a201d90130a2018a19069ef5190717f500f500f500f5021a527447030201d908a1a201d90130a2018a19069ef5190717f501f500f500f5021a527447030203035820ad4b948699193634a39dd56f779a2951a24779ad52aa7916f6912b8ec4702cee04583900588e8e1d18cba576a4d35758069fe94e53f638b6faf7c07b8abd2bc5c5cdee47b60edc7772855324c85033c638364214cbfc6627889f81c4051a00539c2b060007d90130a2018a19073cf5190717f500f502f400f4021a52744703086e63617264616e6f2d77616c6c6574");
        let cbor = cbor.unwrap();
        let request = CardanoCatalystVotingRegistrationRequest::from_cbor(cbor).unwrap();
        assert_eq!(request.request_id, Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap()));
        assert_eq!(request.delegations.len(), 2);
        assert_eq!(request.delegations[0].get_path().get_path().unwrap(), "1694'/1815'/0'/0'/0'");
        assert_eq!(request.delegations[0].get_weidth(), 1);
        assert_eq!(request.delegations[1].get_path().get_path().unwrap(), "1694'/1815'/1'/0'/0'");
        assert_eq!(request.stake_pub, hex::decode("ad4b948699193634a39dd56f779a2951a24779ad52aa7916f6912b8ec4702cee").unwrap());
        assert_eq!(request.payment_address, hex::decode("00588e8e1d18cba576a4d35758069fe94e53f638b6faf7c07b8abd2bc5c5cdee47b60edc7772855324c85033c638364214cbfc6627889f81c4").unwrap());
        assert_eq!(request.nonce, 5479467);
        assert_eq!(request.voting_purpose, 0);
    }
}