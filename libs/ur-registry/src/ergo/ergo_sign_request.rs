use crate::cbor::{cbor_array, cbor_map};
use crate::crypto_key_path::CryptoKeyPath;
use crate::impl_template_struct;
use crate::registry_types::{RegistryType, ERGO_SIGN_REQUEST, UUID};
use crate::traits::{MapSize, RegistryItem};
use crate::types::Bytes;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use crate::ergo::ergo_unspent_box::ErgoUnspentBox;

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;
const DATA_TYPE: u8 = 3;
const DERIVATION_PATHS: u8 = 4;
const BOXES: u8 = 5;
const ORIGIN: u8 = 6;

#[derive(Clone, Debug, Default)]
pub enum DataType {
    #[default]
    Transaction = 1
}

impl DataType {
    pub fn from_u32(i: u32) -> Result<Self, String> {
        match i {
            1 => Ok(DataType::Transaction),
            x => Err(format!(
                "invalid value for data_type in ergo-sign-request, expected (1), received {:?}",
                x
            ))
        }
    }
}

impl_template_struct!(ErgoSignRequest {
    request_id: Bytes,
    sign_data: Bytes,
    data_type: DataType,
    derivation_paths: Vec<CryptoKeyPath>,
    boxes: Vec<ErgoUnspentBox>,
    origin: Option<String>
});

impl RegistryItem for ErgoSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        ERGO_SIGN_REQUEST
    }
}

impl MapSize for ErgoSignRequest {
    fn map_size(&self) -> u64 {
        let mut size = 5;
        if self.origin.is_some() {
            size += 1;
        }
        size
    }
}

impl<C> minicbor::Encode<C> for ErgoSignRequest {
    fn encode<W: minicbor::encode::Write>(
        &self,
        e: &mut minicbor::Encoder<W>,
        ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.map_size())?;
        e.int(
            Int::try_from(REQUEST_ID)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .tag(Tag::Unassigned(UUID.get_tag()))?
            .bytes(&self.get_request_id())?;
        e.int(
            Int::try_from(SIGN_DATA)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .bytes(&self.get_sign_data())?;
        e.int(
            Int::try_from(DATA_TYPE)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .int(
                Int::try_from(self.get_data_type() as u8)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?;

        let derivation_paths = self.get_derivation_paths();
        if derivation_paths.is_empty() {
            return Result::Err(minicbor::encode::Error::message(
                "derivation_paths is invalid",
            ));
        }
        e.int(
            Int::try_from(DERIVATION_PATHS)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .array(derivation_paths.len() as u64)?;
        for path in derivation_paths {
            e.tag(Tag::Unassigned(
                CryptoKeyPath::get_registry_type().get_tag(),
            ))?;
            CryptoKeyPath::encode(&path, e, ctx)?;
        }

        let boxes = self.get_boxes();
        if boxes.is_empty() {
            return Result::Err(minicbor::encode::Error::message(
                "boxes is invalid",
            ));
        }
        e.int(
            Int::try_from(BOXES)
                .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
        )?
            .array(boxes.len() as u64)?;
        for ubox in boxes {
            e.tag(Tag::Unassigned(
                ErgoUnspentBox::get_registry_type().get_tag(),
            ))?;
            ErgoUnspentBox::encode(&ubox, e, ctx)?;
        }

        if let Some(origin) = self.get_origin() {
            e.int(
                Int::try_from(ORIGIN)
                    .map_err(|e| minicbor::encode::Error::message(e.to_string()))?,
            )?
                .str(&origin)?;
        }
        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for ErgoSignRequest {
    fn decode(d: &mut minicbor::Decoder<'b>, ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = ErgoSignRequest::default();

        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    let tag = d.tag()?;
                    if !tag.eq(&Tag::Unassigned(UUID.get_tag())) {
                        return Result::Err(minicbor::decode::Error::message(
                            "UUID tag is invalid",
                        ));
                    }
                    obj.request_id = d.bytes()?.to_vec();
                }
                SIGN_DATA => {
                    obj.sign_data = d.bytes()?.to_vec();
                }
                DATA_TYPE => {
                    obj.data_type =
                        DataType::from_u32(d.u32()?).map_err(minicbor::decode::Error::message)?;
                }
                DERIVATION_PATHS => {
                    cbor_array(d, &mut obj.derivation_paths, |_key, obj, d| {
                        let tag = d.tag()?;
                        if !tag.eq(&Tag::Unassigned(
                            CryptoKeyPath::get_registry_type().get_tag(),
                        )) {
                            return Result::Err(minicbor::decode::Error::message(
                                "CryptoKeyPath tag is invalid",
                            ));
                        }
                        obj.push(CryptoKeyPath::decode(d, ctx)?);
                        Ok(())
                    })?;
                }
                BOXES => {
                    cbor_array(d, &mut obj.boxes, |_index, array, d| {
                        d.tag()?;
                        let item = ErgoUnspentBox::decode(d, ctx)?;
                        array.push(item);
                        Ok(())
                    })?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto_key_path::CryptoKeyPath;
    use crate::crypto_key_path::PathComponent;
    use alloc::vec;
    use minicbor::Encode;
    use crate::ergo::ergo_unspent_box::ErgoAsset;

    extern crate std;

    #[test]
    fn test_construct() {
        let sign_data = hex::decode("9402011a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c00000001fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e4003c0843d0008cd02dc5b9d9d2081889ef00e6452fb5ad1730df42444ceccb9ea02258256d2fbd262e4f25601006400c0843d1005040004000e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a701730073011001020402d19683030193a38cc7b2a57300000193c2b2a57301007473027303830108cdeeac93b1a57304e4f2560000809bee020008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aae4f25601006400cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa0000").unwrap();

        let unspent_boxes = vec![
            ErgoUnspentBox::new(
          "1a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c".to_string(),
          8000000,
          "0008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa".to_string(),
          Some(
              vec![
                  ErgoAsset::new(
                      "fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e40".to_string(),
                      200
                  )
              ]
          ))
        ];

        let derivation_paths = vec![
            CryptoKeyPath::new(
                vec![
                    PathComponent::new(Some(44), true).unwrap(),
                    PathComponent::new(Some(429), true).unwrap(),
                    PathComponent::new(Some(0), true).unwrap(),
                    PathComponent::new(Some(0), false).unwrap(),
                    PathComponent::new(Some(6), false).unwrap(),
                ],
                None,
                None,
            )
        ];

        let request_id = [155, 29, 235, 77, 59, 125, 75, 173, 155, 221, 43, 13, 123, 61, 203, 109, ].to_vec();

        let ergo_sign_request = ErgoSignRequest::new(
            request_id,
            sign_data,
            DataType::Transaction,
            derivation_paths,
            unspent_boxes,
            Some("ergo-wallet".to_string()),
        );

        let sign_request: Vec<u8> = ergo_sign_request.try_into().unwrap();
        assert_eq!("a601d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d0259013a9402011a9f15bfac9379c882fe0b7ecb2288153ce4f2def4f272214fb80f8e2630f04c00000001fbbaac7337d051c10fc3da0ccb864f4d32d40027551e1c3ea3ce361f39b91e4003c0843d0008cd02dc5b9d9d2081889ef00e6452fb5ad1730df42444ceccb9ea02258256d2fbd262e4f25601006400c0843d1005040004000e36100204a00b08cd0279be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798ea02d192a39a8cc7a701730073011001020402d19683030193a38cc7b2a57300000193c2b2a57301007473027303830108cdeeac93b1a57304e4f2560000809bee020008cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aae4f25601006400cd0388fa54338147371023aacb846c96c57e72cdcd73bc85d20250467e5b79dfa2aa000003010481d90130a1018a182cf51901adf500f500f406f40581d920d3a401784031613966313562666163393337396338383266653062376563623232383831353363653466326465663466323732323134666238306638653236333066303463021a007a12000378483030303863643033383866613534333338313437333731303233616163623834366339366335376537326364636437336263383564323032353034363765356237396466613261610481d920d4a2017840666262616163373333376430353163313066633364613063636238363466346433326434303032373535316531633365613363653336316633396239316534300218c8066b6572676f2d77616c6c6574", hex::encode(sign_request));
    }
}

