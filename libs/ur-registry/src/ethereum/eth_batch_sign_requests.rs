use crate::cbor::{cbor_array, cbor_map};
use crate::registry_types::{RegistryType, ETH_BATCH_SIGN_REQUEST, ETH_SIGN_REQUEST};
use crate::traits::{MapSize, RegistryItem};
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

use super::eth_sign_request::EthSignRequest;

const REQUESTS: u8 = 1;

#[derive(Clone, Debug, Default)]
pub struct EthBatchSignRequest {
    requests: Vec<EthSignRequest>,
}

impl EthBatchSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn new(requests: Vec<EthSignRequest>) -> Self {
        Self { requests }
    }

    pub fn set_requests(&mut self, requests: Vec<EthSignRequest>) {
        self.requests = requests;
    }

    pub fn get_requests(&self) -> &Vec<EthSignRequest> {
        &self.requests
    }
}

impl RegistryItem for EthBatchSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ETH_BATCH_SIGN_REQUEST
    }
}

impl MapSize for EthBatchSignRequest {
    fn map_size(&self) -> u64 {
        1
    }
}

impl<C> minicbor::Encode<C> for EthBatchSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(Int::from(REQUESTS))?
            .array(self.requests.len() as u64)?;
        for request in &self.requests {
            e.tag(Tag::Unassigned(ETH_SIGN_REQUEST.get_tag()))?;
            request.encode(e, _ctx)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for EthBatchSignRequest {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = EthBatchSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUESTS => {
                    let mut requests = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        d.tag()?;
                        requests.push(EthSignRequest::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.set_requests(requests);
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
    use super::*;

    #[test]
    fn test_encode_decode() {
        let cbor = hex::decode("a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02584bf849808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000578080800301040105d90130a2018a182cf501f501f500f401f4021a1234567807686d6574616d61736b").unwrap();

        let request1 = EthSignRequest::try_from(cbor).unwrap();

        let batch_request = EthBatchSignRequest::new(vec![request1]);

        let encoded: Vec<u8> = batch_request.try_into().unwrap();

        let message = encoded;
        let ur_type = ETH_BATCH_SIGN_REQUEST.get_type();
        let ur = ur::encode(message.as_slice(), ur_type);
        assert_eq!(ur, "ur:eth-batch-sign-request/oyadlytaadmeoladtpdagdndcawmgtfrkigrpmndutdnbtkgfssbjnaohdgryagalalnascsgljpnbaelfdibemwaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaelaoxlbjyihjkjyeyaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaehnaehglalalaaxadaaadahtaaddyoeadlecsdwykadykadykaewkadwkaocybgeehfksatisjnihjyhsjnhsjkjekoamykvw");
    }

    #[test]
    fn encode_testcase2() {
        let cbor = hex::decode("a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d02587702f87482a86901841dcd6500849502f9008252089446a836a6d5800dd3ab9a6b914c904ef8017b48c8880dcac353ec227a0080c001a03cebc64b4bd58567b7205897f1f68922c3f142366b3236fba169bea5ab875284a05291dae91b105ac2c0dc5479ecf1ed7890d93c2ab1e12695f1e8ecbc92a42e5a03040419a86905d90130a2018a182cf5183cf500f500f400f4021a52744703076b636f72652077616c6c6574").unwrap();
        let request1 = EthSignRequest::try_from(cbor).unwrap();

        let batch_request = EthBatchSignRequest::new(vec![request1]);

        let encoded: Vec<u8> = batch_request.try_into().unwrap();

        let message = encoded;
        let ur_type = ETH_BATCH_SIGN_REQUEST.get_type();
        let ur = ur::encode(message.as_slice(), ur_type);
        assert_eq!(ur, "ur:eth-batch-sign-request/oyadlytaadmeoladtpdagdndcawmgtfrkigrpmndutdnbtkgfssbjnaohdktaoyajylfpdinadlrcasnihaelrmdaoytaelfgmaymwfgpdenoltllabttepynyjemegsmhglyaadkgfdsplobtsgsrguwpcpknaelartadnbfnwmswgrgrtllpiorlcxhdmswnynldcpsrwnfwenjeeyenzooyinrnonpyltgmlrnbgmmetnwlcwbehtsartuoghkkwpwnweksmhtafndrpavydsmdwnvswprfmooxdmhtaxaaaacfpdinahtaaddyoeadlecsdwykcsfnykaeykaewkaewkaocygmjyflaxatjeiajljpihcxkthsjzjzihjyfsbswdvd");
    }
}
