use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, ARWEAVE_CRYPTO_ACCOUNT};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::{Bytes, Fingerprint};
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::Int;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const MASTER_FINGERPRINT: u8 = 1;
const KEY_DATA: u8 = 2;
const DEVICE: u8 = 3;

#[derive(Clone, Debug, Default)]
pub struct ArweaveCryptoAccount {
    master_fingerprint: Fingerprint,
    key_data: Bytes,
    device: Option<String>,
}

impl ArweaveCryptoAccount {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_master_fingerprint(&mut self, mfp: Fingerprint) {
        self.master_fingerprint = mfp;
    }

    pub fn set_key_data(&mut self, data: Bytes) {
        self.key_data = data;
    }

    pub fn set_device(&mut self, device: String) {
        self.device = Some(device)
    }

    pub fn new(
        master_fingerprint: Fingerprint,
        key_data: Bytes,
        device: Option<String>,
    ) -> ArweaveCryptoAccount {
        ArweaveCryptoAccount {
            master_fingerprint,
            key_data,
            device,
        }
    }
    pub fn get_master_fingerprint(&self) -> Fingerprint {
        self.master_fingerprint
    }
    pub fn get_key_data(&self) -> Bytes {
        self.key_data.clone()
    }
    pub fn get_device(&self) -> Option<String> {
        self.device.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 2;
        if self.device.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for ArweaveCryptoAccount {
    fn get_registry_type() -> RegistryType<'static> {
        ARWEAVE_CRYPTO_ACCOUNT
    }
}

impl<C> minicbor::Encode<C> for ArweaveCryptoAccount {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        e.int(Int::from(MASTER_FINGERPRINT))?.int(
            Int::try_from(u32::from_be_bytes(self.master_fingerprint))
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?;

        e.int(Int::from(KEY_DATA))?.bytes(&self.key_data)?;

        if let Some(device) = &self.device {
            e.int(Int::from(DEVICE))?.str(device)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ArweaveCryptoAccount {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ArweaveCryptoAccount::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    let mfp = u32::try_from(d.int()?)
                        .map_err(|e| minicbor::decode::Error::message(e.to_string()));
                    obj.master_fingerprint = u32::to_be_bytes(mfp?);
                }
                KEY_DATA => {
                    obj.key_data = d.bytes()?.to_vec();
                }
                DEVICE => {
                    obj.device = Some(d.str()?.to_string());
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for ArweaveCryptoAccount {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<ArweaveCryptoAccount> for ArweaveCryptoAccount {
    fn from_cbor(bytes: Vec<u8>) -> URResult<ArweaveCryptoAccount> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::arweave::arweave_crypto_account::ArweaveCryptoAccount;
    use crate::traits::{From, To};
    use alloc::string::ToString;
    use alloc::vec::Vec;
    use hex::FromHex;

    #[test]
    fn test_encode() {
        let master_fingerprint: [u8; 4] = [233, 24, 28, 243];
        let key_data = hex::decode("c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c95").unwrap();
        let device = Some("keystone".to_string());

        let arweave_account = ArweaveCryptoAccount::new(master_fingerprint, key_data, device);

        assert_eq!(
            "a3011ae9181cf302590200c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c9503686b657973746f6e65",
            hex::encode(arweave_account.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "a3011ae9181cf302590200c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c9503686b657973746f6e65",
        ).unwrap();

        let arweave_account = ArweaveCryptoAccount::from_cbor(bytes).unwrap();

        let expect_key_data = hex::decode("c41a50ed2155a5740b45df8e3815774d6b8d193e5ad80c9efaaf6d6d0253f350c85becf39eb7056d75841f6a064acf8381383eceb218e16859ef72be7273321a2b4855b87bc6f14c734e2a9c90850c34a8a0a4279ac9be3186b086db5b302fb68176b4c1fee337456c42f972c7993f618fdedc0bf1658c2d59cf2c0c6ac31a61ac1260e0fd4a761ca3707e27611c14b4c6b6abe698c11009ddf5d1511ae47ea271079b6892d229a27d0822e0c7aa12a4cf7f7c28fe23d201eae2adb7f403c9c5a1762c2d8cc96898ce41fe529ab0ef8184e50063e6fc62e0a808e8602254c142c9e7f7e94e6ef2c767ac0e99810d09a44bfde8db46298bc0e25b4a333b4ef86cd7ce658ff661ab0d1789b603b8770a6b433851a91c8ff07a7a8a0767702f6887098ea34bf4a8309eaab9baadd16d45cdd9b1899b6a303a2dce23745cec9fc2ecd9735a66c77fdea1bfd4cdb2be7bfb407a4fd5d3405c3cb33b5316e16559f0c4bf0bc7d1a3ada78917217b289c4d75eb60e0396f03035fd8d553727c790189cfd8dabcee8a4ae6607925b9a27ff7ad7ede26b98f8acd2532cf3175693f3eede9989a0aeedbdb3ff14fec823017531aead4cd22733ab30dbce76cebcdac64424128d6eeff3cdc1825d7cdb7113e74db126e6d931544467c6979aa8d50ac803f36084ed7077f34acfcf3f77bb13d5ebb723fc5d3f45212d2dd6ef20ea757fb4c95").unwrap();

        assert_eq!([233, 24, 28, 243], arweave_account.get_master_fingerprint());
        assert_eq!(expect_key_data, arweave_account.get_key_data());
        assert_eq!(
            "keystone".to_string(),
            arweave_account.get_device().unwrap()
        );
    }
}
