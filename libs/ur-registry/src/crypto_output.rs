use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};
use minicbor::data::{Tag, Type};
use crate::crypto_ec_key::CryptoECKey;
use crate::crypto_hd_key::CryptoHDKey;
use crate::error::{URError, UrResult};
use crate::multi_key::MultiKey;
use crate::registry_types::{CRYPTO_ECKEY, CRYPTO_HDKEY, CRYPTO_OUTPUT, RegistryType};
use crate::script_expression::ScriptExpression;
use crate::traits::{From as FromCbor, RegistryItem, To};

#[derive(Clone, Debug, Default)]
pub struct CryptoOutput {
    script_expressions: Vec<ScriptExpression>,
    ec_key: Option<CryptoECKey>,
    hd_key: Option<CryptoHDKey>,
    multi_key: Option<MultiKey>,
}

impl CryptoOutput {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn new(script_expressions: Vec<ScriptExpression>, ec_key: Option<CryptoECKey>, hd_key: Option<CryptoHDKey>, multi_key: Option<MultiKey>) -> Self {
        CryptoOutput {
            script_expressions,
            ec_key,
            hd_key,
            multi_key,
        }
    }

    pub fn get_script_expressions(&self) -> Vec<ScriptExpression> {
        self.script_expressions.clone()
    }

    pub fn get_ec_key(&self) -> Option<CryptoECKey> {
        self.ec_key.clone()
    }

    pub fn get_hd_key(&self) -> Option<CryptoHDKey> {
        self.hd_key.clone()
    }

    pub fn get_multi_key(&self) -> Option<MultiKey> {
        self.multi_key.clone()
    }
}

impl RegistryItem for CryptoOutput {
    fn get_registry_type() -> RegistryType<'static> {
        CRYPTO_OUTPUT
    }
}

impl<C> minicbor::Encode<C> for CryptoOutput {
    fn encode<W: Write>(&self,
                        e: &mut Encoder<W>,
                        ctx: &mut C) -> Result<(), minicbor::encode::Error<W::Error>> {
        for script_expression in &self.script_expressions {
            e.tag(Tag::Unassigned(script_expression.get_tag_value() as u64))?;
        }
        if let Some(ec_key) = &self.ec_key {
            e.tag(Tag::Unassigned(CRYPTO_ECKEY.get_tag()))?;
            CryptoECKey::encode(ec_key, e, ctx)?;
        }

        if let Some(hd_key) = &self.hd_key {
            e.tag(Tag::Unassigned(CRYPTO_HDKEY.get_tag()))?;
            CryptoHDKey::encode(hd_key, e, ctx)?;
        }

        if let Some(multi_key) = &self.multi_key {
            MultiKey::encode(multi_key, e, ctx)?;
        }

        Ok(())
    }
}


impl<'b, C> minicbor::Decode<'b, C> for CryptoOutput {
    fn decode(d: &mut Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = CryptoOutput::default();
        let mut script_expressions: Vec<ScriptExpression> = vec![];
        let mut last_tag_value: u64 = 0;
        while d.datatype()? == Type::Tag {
            let tag = d.tag()?;
            if let Tag::Unassigned(n) = tag {
                if n != CRYPTO_HDKEY.get_tag() && n != CRYPTO_ECKEY.get_tag() {
                    script_expressions.push(ScriptExpression::from(n));
                }
                last_tag_value = n;
            }
        }
        let is_multi_key = !script_expressions.is_empty() && script_expressions[script_expressions.len() - 1] == ScriptExpression::MultiSig || script_expressions[script_expressions.len() - 1] == ScriptExpression::SortedMultiSig;
        result.script_expressions = script_expressions;
        if is_multi_key {
            result.multi_key = Some(MultiKey::decode(d, ctx)?);
        } else if last_tag_value == CRYPTO_ECKEY.get_tag() {
            result.ec_key = Some(CryptoECKey::decode(d, ctx)?);
        } else if last_tag_value == CRYPTO_HDKEY.get_tag() {
            result.hd_key = Some(CryptoHDKey::decode(d, ctx)?);
        }
        Ok(result)
    }
}

impl To for CryptoOutput {
    fn to_bytes(&self) -> UrResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<CryptoOutput> for CryptoOutput {
    fn from_cbor(bytes: Vec<u8>) -> UrResult<CryptoOutput> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use alloc::vec;
    use alloc::vec::Vec;
    use crate::traits::{From as FromCbor, RegistryItem, To};
    use hex::FromHex;
    use crate::crypto_ec_key::CryptoECKey;
    use crate::crypto_output::CryptoOutput;
    use crate::multi_key::MultiKey;
    use crate::script_expression::ScriptExpression;

    #[test]
    fn test_encode() {
        let script_expressions = vec![ScriptExpression::PublicKeyHash];
        let bytes = Vec::from_hex(
            "02c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
        )
            .unwrap();
        let ec_keys = CryptoECKey::new(None, None, bytes);

        let crypto = CryptoOutput::new(script_expressions, Some(ec_keys), None, None);
        assert_eq!(
            "d90193d90132a103582102c6047f9441ed7d6d3045406e95c07cd85c778e4b8cef3ca7abac09b95c709ee5",
            hex::encode(crypto.to_bytes().unwrap()).to_lowercase()
        );

        let ur = ur::encode(&*(crypto.to_bytes().unwrap()), CryptoOutput::get_registry_type().get_type());

        assert_eq!(ur, "ur:crypto-output/taadmutaadeyoyaxhdclaoswaalbmwfpwekijndyfefzjtmdrtketphhktmngrlkwsfnospypsasrhhhjonnvwtsqzwljy");


        let script_expressions = vec![ScriptExpression::ScriptHash, ScriptExpression::WitnessPublicKeyHash];
        let bytes = Vec::from_hex(
            "03fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556",
        )
            .unwrap();

        let ec_keys = CryptoECKey::new(None, None, bytes);
        let crypto = CryptoOutput::new(script_expressions, Some(ec_keys), None, None);
        assert_eq!(
            "d90190d90194d90132a103582103fff97bd5755eeea420453a14355235d382f6472f8568a18b2f057a1460297556",
            hex::encode(crypto.to_bytes().unwrap()).to_lowercase()
        );
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

        let crypto = CryptoOutput::new(script_expressions, None, None, Some(multi_key));
        assert_eq!(
            "d90190d90196a201020282d90132a1035821022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01d90132a103582103acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
            hex::encode(crypto.to_bytes().unwrap()).to_lowercase()
        );
    }

    #[test]
    fn test_decode() {
        let bytes = Vec::from_hex(
            "d90190d90196a201020282d90132a1035821022f01e5e15cca351daff3843fb70f3c2f0a1bdd05e5af888a67784ef3e10a2a01d90132a103582103acd484e2f0c7f65309ad178a9f559abde09796974c57e714c35f110dfc27ccbe",
        )
            .unwrap();
        let crypto = CryptoOutput::from_cbor(bytes).unwrap();
        assert_eq!(vec![ScriptExpression::ScriptHash, ScriptExpression::MultiSig], crypto.get_script_expressions());
        assert_eq!(2, crypto.get_multi_key().unwrap().get_threshold());
    }
}