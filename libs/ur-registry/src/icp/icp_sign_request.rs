use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use crate::{cbor::cbor_map, registry_types::ICP_SIGN_REQUEST};
use crate::{
    crypto_key_path::CryptoKeyPath,
    error::{URError, URResult},
};
use crate::{
    impl_template_struct,
    registry_types::{RegistryType, CRYPTO_KEYPATH},
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const MASTER_FINGERPRINT: u8 = 1;
const SIGN_DATA: u8 = 2;
const SIGN_TYPE: u8 = 3;
const ORIGIN: u8 = 4;
const DERIVATION_PATH: u8 = 5;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SignType {
    #[default]
    Transaction = 1,
    Message = 2,
}

impl SignType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(SignType::Transaction),
            2 => Ok(SignType::Message),
            x => Err(format!(
                "invalid value for sign_type in icp-sign-request, expected (1, 2, 3), received {:?}",
                x
            )),
        }
    }
}

impl_template_struct!(IcpSignRequest {
    master_fingerprint: Fingerprint,
    sign_data: Bytes,
    sign_type: SignType,
    origin: Option<String>,
    derivation_path: CryptoKeyPath
});

impl RegistryItem for IcpSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ICP_SIGN_REQUEST
    }
}

impl MapSize for IcpSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 4;
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for IcpSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;

        e.int(Int::from(MASTER_FINGERPRINT))?.int(
            Int::try_from(u32::from_be_bytes(self.master_fingerprint))
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?;

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        e.int(Int::from(SIGN_TYPE))?
            .int(Int::from(self.sign_type.clone() as u8))?;

        e.int(Int::from(DERIVATION_PATH))?
            .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
        CryptoKeyPath::encode(&self.derivation_path, e, ctx)?;

        if let Some(origin) = &self.origin {
            e.int(Int::from(ORIGIN))?.str(origin)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for IcpSignRequest {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = IcpSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    let mfp = u32::try_from(d.int()?)
                        .map_err(|e| minicbor::decode::Error::message(e.to_string()));
                    obj.master_fingerprint = u32::to_be_bytes(mfp?);
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                SIGN_TYPE => {
                    obj.sign_type = SignType::from_u32(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(minicbor::decode::Error::message)?;
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = CryptoKeyPath::decode(d, ctx)?;
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

impl To for IcpSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<IcpSignRequest> for IcpSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<IcpSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        crypto_key_path::CryptoKeyPath,
        traits::{From, To},
    };
    use crate::{
        crypto_key_path::PathComponent,
        icp::icp_sign_request::{IcpSignRequest, SignType},
    };
    use alloc::vec::Vec;
    use alloc::{string::ToString, vec};
    use hex::FromHex;
    use crate::traits::RegistryItem;

    #[test]
    fn test_encode() {
        let master_fingerprint: [u8; 4] = [233, 24, 28, 243];
        let sign_data =
            hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca")
                .unwrap();
        let sign_type = SignType::Transaction;
        let origin = Some("plugwallet".to_string());
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(223), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();

        let source_fingerprint: [u8; 4] = [242, 63, 159, 210];
        let components = vec![path1, path2, path3];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let sign_request = IcpSignRequest::new(
            master_fingerprint,
            sign_data,
            sign_type,
            origin,
            crypto_key_path,
        );
        assert_eq!(
            "a5011ae9181cf3025820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca030105d90130a20186182cf518dff500f5021af23f9fd2046a706c756777616c6c6574",
            hex::encode(sign_request.to_bytes().unwrap()).to_lowercase()
        );

        // convert ur
        let ur_string = ur::encode(&sign_request.to_bytes().unwrap(), IcpSignRequest::get_registry_type().get_type());
        assert_eq!("ur:icp-sign-request/onadcywlcscewfaohdcxpeksyahpdttplehswygatejtlrbwnnspgycegomybbhsdkbwwngdfrmninhkpmsgaxadahtaaddyoeadlncsdwykcsurykaeykaocywzfhnetdaaimjojzkpiokthsjzjzihjychfmiave", ur_string);
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a5011ae9181cf3025820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca030105d90130a20186182cf518dff500f5021af23f9fd2046a706c756777616c6c6574",
        ).unwrap();
        let icp_sign_request = IcpSignRequest::from_cbor(bytes).unwrap();
        let sign_data =
            hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca")
                .unwrap();

        assert_eq!(
            [233, 24, 28, 243],
            icp_sign_request.get_master_fingerprint()
        );
        assert_eq!(sign_data, icp_sign_request.get_sign_data());
        assert_eq!(SignType::Transaction, icp_sign_request.get_sign_type());
        assert_eq!(
            "44'/223'/0'",
            icp_sign_request.get_derivation_path().get_path().unwrap()
        );
    }
}
