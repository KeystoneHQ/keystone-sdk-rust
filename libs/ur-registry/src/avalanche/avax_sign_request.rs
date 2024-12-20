use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, AVAX_SIGN_REQUEST};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

#[derive(Debug, Clone, Default)]
pub struct AvaxSignRequest {
    data: Bytes,
}

impl AvaxSignRequest {
    pub fn new(data: Bytes) -> Self {
        AvaxSignRequest { data }
    }

    pub fn get_tx_data(&self) -> Bytes {
        self.data.clone()
    }

    pub fn set_tx_data(&mut self, data: Bytes) {
        self.data = data;
    }
}

impl RegistryItem for AvaxSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        AVAX_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for AvaxSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.bytes(&self.data)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for AvaxSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        Ok(Self {
            data: d.bytes()?.to_vec(),
        })
    }
}

impl To for AvaxSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<AvaxSignRequest> for AvaxSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<AvaxSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::RegistryItem;
    use alloc::vec::Vec;
    use hex::FromHex;
    extern crate std;
    use std::println;

    #[test]
    fn test_avax_encode() {
        let unsigned_data = AvaxSignRequest {
            data: Vec::from_hex("00000000001a000000050000000000000000000000000000000000000000000000000000000000000000000000013d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000007000000003b9a9e0400000000000000000000000100000001e0beb088f94b8224eb5d6f1115561d7173cd6e7f00000002295a7b15e26c6cafda8883afd0f724e0e0b1dad4517148711434cb96fb3c8a61000000013d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000005000000003b9aca0000000001000000006109bc613691602ca0811312357676416252412a87ded6c56c240baba1afe042000000013d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000005000000003b9aca000000000100000000000000007072a3df0cd056d9b9ef00c09630bad3027dc312000000006760c3b100000000676215a9000000003b9aca000000000000000000000000000000000000000000000000000000000000000000000000013d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000007000000003b9aca0000000000000000000000000100000001e0beb088f94b8224eb5d6f1115561d7173cd6e7f0000000b00000000000000000000000100000001a0f4d4d9a0ea219da5ed5499ad083e1942a0846a000000020000000900000001438c3a393f49bb27791ca830effec456c2642a487ee4ce89300dd2e591fc22ab6b2aa8e08515ca229f2a2f14168700e05a1f96bd61d1fc3ab31e9e71ef9f16bb000000000900000001438c3a393f49bb27791ca830effec456c2642a487ee4ce89300dd2e591fc22ab6b2aa8e08515ca229f2a2f14168700e05a1f96bd61d1fc3ab31e9e71ef9f16bb005c3d047c")
                .unwrap(),
        };
        let result: Vec<u8> = unsigned_data.try_into().unwrap();
        let ur = ur::encode(&result, AvaxSignRequest::get_registry_type().get_type());
        assert_eq!(ur, "ur:avax-sign-request/hkaddmaeaeaeaeaecpaeaeaeahaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaofsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaeahykvyaeaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadltjsmobwadtlrszmykmotnvsiymdolbzryqzoxfpfsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeataeaeaeaechstjstdaeaeaeaeaeaeaeaeaeaeaeadaeaeaeadasinwdidvorkdyvajnlfvsdlvoioweynltckonylaeaeaeadhgtlvofmdmctfgbdhslurdcwgomefhwftoqdbzwtttpsssctvafzmnuogtwlzssnaeaeaeaefsndtnrtwecakobwdytkisbazcwmcyfwbznnqdlttbtdmdbnmtyltdmyhsrkvopkaeaeaeahaeaeaeaecaryiobtaeaeaeadaeaeaeaeaeaeaeaeecfhesrp");
    }

    #[test]
    fn test_avax_decode() {
        let bytes =
            Vec::from_hex("59012E000000000022000000050000000000000000000000000000000000000000000000000000000000000000000000023D9BDAC0ED1D761330CF680EFDEB1A42159EB387D6D2950C96F7D28F61BBE2AA000000070000000005F5E100000000000000000000000001000000018771921301D5BFFFF592DAE86695A615BDB4A4413D9BDAC0ED1D761330CF680EFDEB1A42159EB387D6D2950C96F7D28F61BBE2AA000000070000000017C771D2000000000000000000000001000000010969EA62E2BB30E66D82E82FE267EDF6871EA5F70000000157D5E23E2E1F460B618BBA1B55913FF3CEB315F0D1ACC41FE6408EDC4DE9FACD000000003D9BDAC0ED1D761330CF680EFDEB1A42159EB387D6D2950C96F7D28F61BBE2AA00000005000000001DBD670D000000010000000000000000")
                .unwrap();
        let data = AvaxSignRequest::try_from(bytes).unwrap();
        assert_eq!(
            data.get_tx_data(),
            Vec::from_hex("000000000022000000050000000000000000000000000000000000000000000000000000000000000000000000023d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa000000070000000005f5e100000000000000000000000001000000018771921301d5bffff592dae86695a615bdb4a4413d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa000000070000000017c771d2000000000000000000000001000000010969ea62e2bb30e66d82e82fe267edf6871ea5f70000000157d5e23e2e1f460b618bba1b55913ff3ceb315f0d1acc41fe6408edc4de9facd000000003d9bdac0ed1d761330cf680efdeb1a42159eb387d6d2950c96f7d28f61bbe2aa00000005000000001dbd670d000000010000000000000000")
                .unwrap()
        );
    }
}
