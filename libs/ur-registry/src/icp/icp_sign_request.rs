use crate::traits::{From as FromCbor, MapSize, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use crate::{cbor::cbor_map, registry_types::ICP_SIGN_REQUEST};
use crate::{
    crypto_key_path::CryptoKeyPath,
    error::{URError, URResult},
};
use crate::{
    impl_template_struct,
    registry_types::{RegistryType, CRYPTO_KEYPATH, UUID},
};
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const MASTER_FINGERPRINT: u8 = 1;
const REQUEST_ID: u8 = 2;
const SIGN_DATA: u8 = 3;
const SIGN_TYPE: u8 = 4;
const SALT_LEN: u8 = 5;
const ORIGIN: u8 = 6;
const ACCOUNT: u8 = 7;
const DERIVATION_PATH: u8 = 8;

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SignType {
    #[default]
    Transaction = 1,
    DataItem = 2,
    Message = 3,
}

impl SignType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(SignType::Transaction),
            2 => Ok(SignType::DataItem),
            3 => Ok(SignType::Message),
            x => Err(format!(
                "invalid value for sign_type in icp-sign-request, expected (1, 2, 3), received {:?}",
                x
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default)]
pub enum SaltLen {
    #[default]
    Zero = 0,
    Digest = 32,
}

impl SaltLen {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            0 => Ok(SaltLen::Zero),
            32 => Ok(SaltLen::Digest),
            x => Err(format!(
                "invalid value for salt_len in arweave-sign-request, expected (0, 32), received {:?}",
                x
            )),
        }
    }
}

impl_template_struct!(IcpSignRequest {
    master_fingerprint: Fingerprint,
    request_id: Option<Bytes>,
    sign_data: Bytes,
    sign_type: SignType,
    salt_len: SaltLen,
    account: Option<Bytes>,
    origin: Option<String>,
    derivation_path: Option<CryptoKeyPath>
});

impl RegistryItem for IcpSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ICP_SIGN_REQUEST
    }
}

impl MapSize for IcpSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 3;
        if self.request_id.is_some() {
            size += 1;
        }
        if self.derivation_path.is_some() {
            size += 1;
        }
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

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?.bytes(&self.sign_data)?;

        e.int(Int::from(SIGN_TYPE))?
            .int(Int::from(self.sign_type.clone() as u8))?;

        e.int(Int::from(SALT_LEN))?
            .u32(self.salt_len.clone() as u32)?;

        if let Some(derivation_path) = &self.derivation_path {
            e.int(Int::from(DERIVATION_PATH))?
                .tag(Tag::Unassigned(CRYPTO_KEYPATH.get_tag()))?;
            CryptoKeyPath::encode(derivation_path, e, ctx)?;
        }

        if let Some(account) = &self.account {
            e.int(Int::from(ACCOUNT))?.bytes(account)?;
        }

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
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
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
                SALT_LEN => {
                    obj.salt_len = SaltLen::from_u32(
                        u32::try_from(d.int()?)
                            .map_err(|e| minicbor::decode::Error::message(e.to_string()))?,
                    )
                    .map_err(minicbor::decode::Error::message)?;
                }
                DERIVATION_PATH => {
                    d.tag()?;
                    obj.derivation_path = Some(CryptoKeyPath::decode(d, ctx)?);
                }
                ACCOUNT => {
                    obj.account = Some(d.bytes()?.to_vec());
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
        icp::icp_sign_request::{IcpSignRequest, SaltLen, SignType},
    };
    use alloc::vec::Vec;
    use alloc::{string::ToString, vec};
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let master_fingerprint: [u8; 4] = [233, 24, 28, 243];
        let request_id: Option<Vec<u8>> = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );
        let sign_data =
            hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca")
                .unwrap();
        let sign_type = SignType::Transaction;
        let salt_len = SaltLen::Zero;
        let origin = Some("plugwallet".to_string());
        let path1 = PathComponent::new(Some(44), true).unwrap();
        let path2 = PathComponent::new(Some(223), true).unwrap();
        let path3 = PathComponent::new(Some(0), true).unwrap();

        let source_fingerprint: [u8; 4] = [242, 63, 159, 210];
        let components = vec![path1, path2, path3];
        let crypto_key_path = CryptoKeyPath::new(components, Some(source_fingerprint), None);

        let sign_request = IcpSignRequest::new(
            master_fingerprint,
            request_id,
            sign_data,
            sign_type,
            salt_len,
            None,
            origin,
            Some(crypto_key_path),
        );

        assert_eq!(
            "a6011ae9181cf302d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d035820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca0401050008d90130a20186182cf518dff500f5021af23f9fd2066a706c756777616c6c6574",
            hex::encode(sign_request.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a6011ae9181cf302d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d035820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca0401050008d90130a20186182cf518dff500f5021af23f9fd2066a706c756777616c6c6574",
        ).unwrap();

        let sign_request = IcpSignRequest::from_cbor(bytes).unwrap();

        let request_id = Some(
            [
                155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109,
            ]
            .to_vec(),
        );
        let sign_data =
            hex::decode("af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca")
                .unwrap();

        assert_eq!([233, 24, 28, 243], sign_request.get_master_fingerprint());
        assert_eq!(request_id, sign_request.get_request_id());
        assert_eq!(sign_data, sign_request.get_sign_data());
        assert_eq!(SignType::Transaction, sign_request.get_sign_type());
        assert_eq!(SaltLen::Zero, sign_request.get_salt_len());
    }
}
