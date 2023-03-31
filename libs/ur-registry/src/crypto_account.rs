use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Int, Tag};
use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_output::CryptoOutput;
use crate::error::{URError, UrResult};
use crate::registry_types::{CRYPTO_ACCOUNT, CRYPTO_OUTPUT, RegistryType};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Fingerprint;

const MASTER_FINGERPRINT: u8 = 1;
const OUTPUT_DESCRIPTORS: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct CryptoAccount {
    master_fingerprint: Fingerprint,
    output_descriptors: Vec<CryptoOutput>,
}


impl CryptoAccount {
    pub fn new(master_fingerprint: Fingerprint, output_descriptors: Vec<CryptoOutput>) -> Self {
        CryptoAccount {
            master_fingerprint,
            output_descriptors,
        }
    }

    pub fn get_master_fingerprint(&self) -> Fingerprint {
        self.master_fingerprint.clone()
    }

    pub fn get_output_descriptors(&self) -> Vec<CryptoOutput> {
        self.output_descriptors.clone()
    }

    pub fn set_master_fingerprint(&mut self, fingerprint: Fingerprint) {
        self.master_fingerprint = fingerprint;
    }

    pub fn set_output_descriptors(&mut self, outputs: Vec<CryptoOutput>) {
        self.output_descriptors = outputs;
    }
}

impl RegistryItem for CryptoAccount {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_ACCOUNT
    }
}


impl<C> minicbor::Encode<C> for CryptoAccount {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(2)?;
        e.int(
            Int::try_from(MASTER_FINGERPRINT)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?
            .int(
                Int::try_from(u32::from_be_bytes(self.master_fingerprint))
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?
            )?;

        e.int(
            Int::try_from(OUTPUT_DESCRIPTORS)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?)?;
        e.array(self.output_descriptors.len() as u64)?;
        for output_descriptor in &self.output_descriptors {
            e.tag(Tag::Unassigned(CryptoOutput::get_registry_type().get_tag()))?;
            CryptoOutput::encode(output_descriptor, e, ctx)?;
        }
        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for CryptoAccount {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoAccount::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key = u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                MASTER_FINGERPRINT => {
                    obj.master_fingerprint = u32::to_be_bytes(u32::try_from(d.int()?).map_err(|e| minicbor::decode::Error::message(e.to_string()))?);
                }
                OUTPUT_DESCRIPTORS => {
                    let mut output_descriptors: Vec<CryptoOutput> = vec![];
                    cbor_array(d, obj, |_index, _obj, d| {
                        if let Tag::Unassigned(n) = d.probe().tag()? {
                            if n == CRYPTO_OUTPUT.get_tag() {
                                d.tag()?;
                            }
                        }
                        output_descriptors.push(CryptoOutput::decode(d, ctx)?);
                        Ok(())
                    })?;
                    obj.output_descriptors = output_descriptors;
                }
                _ => {}
            }
            Ok(())
        })?;

        Ok(result)
    }
}

impl To for CryptoAccount {
    fn to_cbor(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoAccount> for CryptoAccount {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<CryptoAccount> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}


#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use hex::FromHex;
    use crate::crypto_account::CryptoAccount;
    use crate::crypto_ec_key::CryptoECKey;
    use crate::crypto_output::CryptoOutput;
    use crate::multi_key::MultiKey;
    use crate::script_expression::ScriptExpression;

    #[test]
    fn test_encode() {
        let master_fingerprint: [u8; 4] = [120, 35, 8, 4];
        let script_expressions = vec![ScriptExpression::PublicKeyHash];
        let bytes = Vec::from_hex(
            "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        )
            .unwrap();
        let ec_keys = CryptoECKey::new(None, None, bytes);
        let crypto1 = CryptoOutput::new(script_expressions, Some(ec_keys), None, None);

        let script_expressions = vec![ScriptExpression::ScriptHash, ScriptExpression::WitnessPublicKeyHash];
        let bytes = Vec::from_hex(
            "03fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556",
        )
            .unwrap();
        let ec_keys = CryptoECKey::new(None, None, bytes);
        let crypto2 = CryptoOutput::new(script_expressions, Some(ec_keys), None, None);

        let bytes = Vec::from_hex(
            "022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01",
        )
            .unwrap();
        let ec1 = CryptoECKey::new(None, None, bytes);
        let bytes = Vec::from_hex(
            "03acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
        )
            .unwrap();
        let ec2 = CryptoECKey::new(None, None, bytes);
        let script_expressions = vec![ScriptExpression::ScriptHash, ScriptExpression::MultiSig];
        let multi_key = MultiKey::new(2, Some(vec![ec1, ec2]), None);
        let crypto3 = CryptoOutput::new(script_expressions, None, None, Some(multi_key));

        let crypto = CryptoAccount::new(master_fingerprint, vec![crypto1, crypto2, crypto3]);
        assert_eq!("a2011a782308040283d90134d90193d90132a103582102c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5d90134d90190d90194d90132a103582103fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556d90134d90190d90196a201020282d90132a1035821022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01d90132a103582103acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
                   hex::encode(crypto.to_cbor().unwrap()).to_lowercase());

        let ur = ur::encode(&*(crypto.to_cbor().unwrap()), CryptoAccount::get_registry_type().get_type());

        assert_eq!(ur, "ur:crypto-account/oeadcykscnayaaaolstaadeetaadmutaadeyoyaxhdclaoswaalbmwfpwekijndyfefzjtmdrtketphhktmngrlkwsfnospypsasrhhhjonnvwtaadeetaadmhtaadmwtaadeyoyaxhdclaxzmytkgtlkphywyoxcxfeftbbecgmectelfynfldllpisoyludlahknbbhndtkphftaadeetaadmhtaadmtoeadaoaolftaadeyoyaxhdclaodladvwvyhhsgeccapewflrfhrlbsfndlbkcwutahvwpeloleioksglwfvybkdradtaadeyoyaxhdclaxpstylrvowtstynguaspmchlenegonyryvtmsmtmsgshgvdbbsrhebybtztdisfrnuyhneets");
    }

    #[test]
    fn test_decode() {
        let part = "ur:crypto-account/oeadcykscnayaaaolstaadeetaadmutaadeyoyaxhdclaoswaalbmwfpwekijndyfefzjtmdrtketphhktmngrlkwsfnospypsasrhhhjonnvwtaadeetaadmhtaadmwtaadeyoyaxhdclaxzmytkgtlkphywyoxcxfeftbbecgmectelfynfldllpisoyludlahknbbhndtkphftaadeetaadmhtaadmtoeadaoaolftaadeyoyaxhdclaodladvwvyhhsgeccapewflrfhrlbsfndlbkcwutahvwpeloleioksglwfvybkdradtaadeyoyaxhdclaxpstylrvowtstynguaspmchlenegonyryvtmsmtmsgshgvdbbsrhebybtztdisfrnuyhneets";

        let decode_data = ur::decode(&part);

        assert_eq!("a2011a782308040283d90134d90193d90132a103582102c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5d90134d90190d90194d90132a103582103fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556d90134d90190d90196a201020282d90132a1035821022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01d90132a103582103acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
                   hex::encode(decode_data.unwrap().1).to_lowercase());

        let bytes = Vec::from_hex(
            "a2011a782308040283d90134d90193d90132a103582102c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5d90134d90190d90194d90132a103582103fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556d90134d90190d90196a201020282d90132a1035821022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01d90132a103582103acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
        )
            .unwrap();
        let crypto = CryptoAccount::from_cbor(bytes).unwrap();
        assert_eq!([120, 35, 8, 4], crypto.master_fingerprint);
        assert_eq!(vec![ScriptExpression::PublicKeyHash], crypto.output_descriptors[0].get_script_expressions());

        assert_eq!("02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
                   hex::encode(crypto.output_descriptors[0].get_ec_key().unwrap().get_data()).to_lowercase());

        assert_eq!(vec![ScriptExpression::ScriptHash, ScriptExpression::WitnessPublicKeyHash], crypto.output_descriptors[1].get_script_expressions());

        assert_eq!("03fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556",
                   hex::encode(crypto.output_descriptors[1].get_ec_key().unwrap().get_data()).to_lowercase());

        assert_eq!(vec![ScriptExpression::ScriptHash, ScriptExpression::MultiSig], crypto.output_descriptors[2].get_script_expressions());

        assert_eq!("03fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556",
                   hex::encode(crypto.output_descriptors[1].get_ec_key().unwrap().get_data()).to_lowercase());
        assert_eq!(2,
                   crypto.output_descriptors[2].get_multi_key().unwrap().get_threshold());
        assert_eq!("022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01",
                   hex::encode(crypto.output_descriptors[2].get_multi_key().unwrap().get_ec_keys().unwrap()[0].get_data()).to_lowercase());
        assert_eq!("03acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
                   hex::encode(crypto.output_descriptors[2].get_multi_key().unwrap().get_ec_keys().unwrap()[1].get_data()).to_lowercase());
    }
}