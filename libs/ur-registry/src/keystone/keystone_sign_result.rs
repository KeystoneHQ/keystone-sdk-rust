use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, KEYSTONE_SIGN_RESULT};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const SIGN_RESULT: u8 = 1;

#[derive(Clone, Debug, Default)]
pub struct KeystoneSignResult {
    sign_result: Bytes,
}

impl KeystoneSignResult {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_sign_result(&mut self, signature: Bytes) {
        self.sign_result = signature;
    }

    pub fn new(signature: Bytes) -> Self {
        KeystoneSignResult {
            sign_result: signature,
        }
    }

    pub fn get_sign_result(&self) -> Bytes {
        self.sign_result.clone()
    }
}

impl RegistryItem for KeystoneSignResult {
    fn get_registry_type() -> RegistryType<'static> {
        KEYSTONE_SIGN_RESULT
    }
}

impl<C> minicbor::Encode<C> for KeystoneSignResult {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(1)?;
        e.int(Int::from(SIGN_RESULT))?.bytes(&self.sign_result)?;
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for KeystoneSignResult {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = KeystoneSignResult::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                SIGN_RESULT => {
                    obj.sign_result = d.bytes()?.to_vec();
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for KeystoneSignResult {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<KeystoneSignResult> for KeystoneSignResult {
    fn from_cbor(bytes: Vec<u8>) -> URResult<KeystoneSignResult> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::keystone::keystone_sign_result::KeystoneSignResult;
    use crate::traits::{From as FromCbor, To};
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let sign_result = hex::decode("1F8B08000000000000004D923D8E14300C85B5628B0121214DB9150505CD48FE8B635351A0A9B942123B0D122376B7E12C9C8C1B50517002CCA241B88822C7FE64BF97C3CDF1D5A7FCFAF078F99CAFBFDCAF4BE4DD8FDBC3F3E3E14C7CF6F3077AF7FDF6C59BB55C74269D2C659D84669E06139E9A8EC6C3D66EA8C7F7B2713919E0748BEC113EF7F018C8A2446D933832C2DE9B87EFB0EE0A163B8AD17AF77DF7F319105C030195B70E9BB46016CF8C53862E6C0DA9B1B7E4241113A69014666B3E07E2C831D6AC6B2BC65F5447AD53960866065AB22F81EC93338D3036E54A27ECA0DCC176ECA70042A00EFF05F6E128C9C164620BB7F49A0D427DDA469ECDC97B20470930B7F55CAA38AE735CFBFB54D6EC69211A634CCF05630CF134FCB31FC2EA5533AD03496710012290DDA2D74B099292C05A45E62BB74CF766A3483A143266AE51F6706D365A69B3CB26C9F9C468E808D6C0A66BF26AB01741CCCA4CCBA0020E3441DCCAA1ADAA2672B9DB5BB9D4A9142D49B8A94994538930178EB261468FE6595340DFB4D173BB5A3524962E59A84466D7DA5554AF6ABCFDF6EB865EFEFB7F1FEF2FBF01CB596BB490020000").unwrap();
        let keystone_sign_result = KeystoneSignResult::new(sign_result);
        assert_eq!(
            "a1015901b11f8b08000000000000004d923d8e14300c85b5628b0121214db9150505cd48fe8b635351a0a9b942123b0d122376b7e12c9c8c1b50517002cca241b88822c7fe64bf97c3cdf1d5a7fcfaf078f99cafbfdcaf4be4dd8fdbc3f3e3e14c7cf6f3077af7fdf6c59bb55c74269d2c659d84669e06139e9a8ec6c3d66ea8c7f7b2713919e0748bec113ef7f018c8a2446d933832c2de9b87efb0ee0a163b8ad17af77df7f319105c030195b70e9bb46016cf8c53862e6c0da9b1b7e4241113a69014666b3e07e2c831d6ac6b2bc65f5447ad53960866065ab22f81ec93338d3036e54a27eca0dcc176eca70042a00eff05f6e128c9c164620bb7f49a0d427dda469ecdc97b20470930b7f55caa38ae735cfbfb54d6ec69211a634ccf05630cf134fcb31fc2ea5533ad03496710012290dda2d74b099292c05a45e62bb74cf766a3483a143266ae51f6706d365a69b3cb26c9f9c468e808d6c0a66bf26ab01741ccca4ccba0020e3441dccaa1adaa2672b9db5bb9d4a9142d49b8a94994538930178eb261468fe6595340dfb4d173bb5a3524962e59a84466d7da5554af6abcfdf6eb865efefb7f1fef2fbf01cb596bb490020000",
            hex::encode(keystone_sign_result.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a1015901b11f8b08000000000000004d923d8e14300c85b5628b0121214db9150505cd48fe8b635351a0a9b942123b0d122376b7e12c9c8c1b50517002cca241b88822c7fe64bf97c3cdf1d5a7fcfaf078f99cafbfdcaf4be4dd8fdbc3f3e3e14c7cf6f3077af7fdf6c59bb55c74269d2c659d84669e06139e9a8ec6c3d66ea8c7f7b2713919e0748bec113ef7f018c8a2446d933832c2de9b87efb0ee0a163b8ad17af77df7f319105c030195b70e9bb46016cf8c53862e6c0da9b1b7e4241113a69014666b3e07e2c831d6ac6b2bc65f5447ad53960866065ab22f81ec93338d3036e54a27eca0dcc176eca70042a00eff05f6e128c9c164620bb7f49a0d427dda469ecdc97b20470930b7f55caa38ae735cfbfb54d6ec69211a634ccf05630cf134fcb31fc2ea5533ad03496710012290dda2d74b099292c05a45e62bb74cf766a3483a143266ae51f6706d365a69b3cb26c9f9c468e808d6c0a66bf26ab01741ccca4ccba0020e3441dccaa1adaa2672b9db5bb9d4a9142d49b8a94994538930178eb261468fe6595340dfb4d173bb5a3524962e59a84466d7da5554af6abcfdf6eb865efefb7f1fef2fbf01cb596bb490020000",
        ).unwrap();

        let keystone_sign_result = KeystoneSignResult::from_cbor(bytes).unwrap();
        let expect_result = hex::decode("1F8B08000000000000004D923D8E14300C85B5628B0121214DB9150505CD48FE8B635351A0A9B942123B0D122376B7E12C9C8C1B50517002CCA241B88822C7FE64BF97C3CDF1D5A7FCFAF078F99CAFBFDCAF4BE4DD8FDBC3F3E3E14C7CF6F3077AF7FDF6C59BB55C74269D2C659D84669E06139E9A8EC6C3D66EA8C7F7B2713919E0748BEC113EF7F018C8A2446D933832C2DE9B87EFB0EE0A163B8AD17AF77DF7F319105C030195B70E9BB46016CF8C53862E6C0DA9B1B7E4241113A69014666B3E07E2C831D6AC6B2BC65F5447AD53960866065AB22F81EC93338D3036E54A27ECA0DCC176ECA70042A00EFF05F6E128C9C164620BB7F49A0D427DDA469ECDC97B20470930B7F55CAA38AE735CFBFB54D6EC69211A634CCF05630CF134FCB31FC2EA5533AD03496710012290DDA2D74B099292C05A45E62BB74CF766A3483A143266AE51F6706D365A69B3CB26C9F9C468E808D6C0A66BF26AB01741CCCA4CCBA0020E3441DCCAA1ADAA2672B9DB5BB9D4A9142D49B8A94994538930178EB261468FE6595340DFB4D173BB5A3524962E59A84466D7DA5554AF6ABCFDF6EB865EFEFB7F1FEF2FBF01CB596BB490020000").unwrap().to_vec();
        assert_eq!(expect_result, keystone_sign_result.get_sign_result());
    }
}
