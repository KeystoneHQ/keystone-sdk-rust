use crate::cbor::cbor_map;
use crate::error::{URError, URResult};
use crate::registry_types::{RegistryType, UUID, XRP_BATCH_SIGN_REQUEST};
use crate::traits::{From as FromCbor, RegistryItem, To};
use crate::types::Bytes;
use alloc::string::ToString;
use alloc::vec::Vec;
use minicbor::data::{Int, Tag};
use minicbor::encode::Write;
use minicbor::{Decoder, Encoder};

const REQUEST_ID: u8 = 1;
const SIGN_DATA: u8 = 2;

#[derive(Clone, Debug, Default)]
pub struct XrpBatchSignRequest {
    request_id: Option<Bytes>,
    sign_data: Vec<Bytes>,
}

impl XrpBatchSignRequest {
    pub fn default() -> Self {
        Default::default()
    }

    pub fn set_request_id(&mut self, id: Bytes) {
        self.request_id = Some(id);
    }

    pub fn set_sign_data(&mut self, data: Vec<Bytes>) {
        self.sign_data = data;
    }

    pub fn new(request_id: Option<Bytes>, sign_data: Vec<Bytes>) -> XrpBatchSignRequest {
        XrpBatchSignRequest {
            request_id,
            sign_data,
        }
    }
    pub fn get_request_id(&self) -> Option<Bytes> {
        self.request_id.clone()
    }
    pub fn get_sign_data(&self) -> Vec<Bytes> {
        self.sign_data.clone()
    }

    fn get_map_size(&self) -> u64 {
        let mut size = 2;
        if self.request_id.is_some() {
            size += 1;
        }
        size
    }
}

impl RegistryItem for XrpBatchSignRequest {
    fn get_registry_type() -> RegistryType<'static> {
        XRP_BATCH_SIGN_REQUEST
    }
}

impl<C> minicbor::Encode<C> for XrpBatchSignRequest {
    fn encode<W: Write>(
        &self,
        e: &mut Encoder<W>,
        _ctx: &mut C,
    ) -> Result<(), minicbor::encode::Error<W::Error>> {
        e.map(self.get_map_size())?;

        if let Some(request_id) = &self.request_id {
            e.int(Int::from(REQUEST_ID))?
                .tag(Tag::Unassigned(UUID.get_tag()))?
                .bytes(request_id)?;
        }

        e.int(Int::from(SIGN_DATA))?;
        let sign_data_len = self.sign_data.len().try_into().unwrap();
        e.array(sign_data_len)?;
        for ele in &self.sign_data {
            e.bytes(ele)?;
        }

        e.int(Int::from(SIGN_DATA))?;
        let sign_data_len = self.sign_data.len().try_into().unwrap();
        e.array(sign_data_len)?;
        for ele in &self.sign_data {
            e.bytes(ele)?;
        }

        Ok(())
    }
}

impl<'b, C> minicbor::Decode<'b, C> for XrpBatchSignRequest {
    fn decode(d: &mut Decoder<'b>, _ctx: &mut C) -> Result<Self, minicbor::decode::Error> {
        let mut result = XrpBatchSignRequest::default();
        cbor_map(d, &mut result, |key, obj, d| {
            let key =
                u8::try_from(key).map_err(|e| minicbor::decode::Error::message(e.to_string()))?;
            match key {
                REQUEST_ID => {
                    d.tag()?;
                    obj.request_id = Some(d.bytes()?.to_vec());
                }
                SIGN_DATA => {
                    let sign_data_len = d.array()?;
                    obj.sign_data = Vec::new();
                    if sign_data_len.is_some() {
                        for _ in 0..sign_data_len.unwrap() {
                            obj.sign_data.push(d.bytes()?.to_vec());
                        }
                    }
                }
                _ => {}
            }
            Ok(())
        })?;
        Ok(result)
    }
}

impl To for XrpBatchSignRequest {
    fn to_bytes(&self) -> URResult<Vec<u8>> {
        minicbor::to_vec(self.clone()).map_err(|e| URError::CborEncodeError(e.to_string()))
    }
}

impl FromCbor<XrpBatchSignRequest> for XrpBatchSignRequest {
    fn from_cbor(bytes: Vec<u8>) -> URResult<XrpBatchSignRequest> {
        minicbor::decode(&bytes).map_err(|e| URError::CborDecodeError(e.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::traits::{From as FromCbor, To};
    use crate::xrp::xrp_batch_sign_request::XrpBatchSignRequest;
    use alloc::vec::Vec;
    use hex::FromHex;
    extern crate std;
    use std::println;

    #[test]
    fn test_xrp_batch_sign_request_encode() {
        let request_id = Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap());
        let sign_data_str = [
            r#"
            {
                "TransactionType": "Payment",
                "Amount": "101",
                "Destination": "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m",
                "DestinationTag": 987654321,
                "Flags": 2147483648,
                "Account": "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m",
                "Fee": "12",
                "Sequence": 82376319,
                "LastLedgerSequence": 83749165,
                "SigningPubKey": "03B91E16E98BA86B62A52AAA2D41C114B36C8BFCD862B1ECED77DC5D77676510F8"
            }
            "#,
            r#"
            {
                "TransactionType": "Payment",
                "Amount": "1001",
                "Destination": "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m",
                "DestinationTag": 123456,
                "Flags": 22222,
                "Account": "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m",
                "Fee": "12",
                "Sequence": 123456,
                "LastLedgerSequence": 123456,
                "SigningPubKey": "03B91E16E98BA86B62A52AAA2D41C114B36C8BFCD862B1ECED77DC5D77676510F8"
            }
            "#,
        ];
        let sign_data: Vec<Vec<u8>> = sign_data_str
            .iter()
            .map(|s| {
                let json_value: serde_json::Value =
                    serde_json::from_str(s.trim()).expect("Invalid JSON format");
                serde_json::to_vec(&json_value).expect("Failed to serialize JSON to bytes")
            })
            .collect();
        let request = XrpBatchSignRequest::new(request_id, sign_data);
        assert_eq!(hex::encode(request.to_bytes().unwrap()), 
                  "a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d028259014e7b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a22313031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3938373635343332312c22466565223a223132222c22466c616773223a323134373438333634382c224c6173744c656467657253657175656e6365223a38333734393136352c2253657175656e6365223a38323337363331392c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d5901437b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a2231303031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3132333435362c22466565223a223132222c22466c616773223a32323232322c224c6173744c656467657253657175656e6365223a3132333435362c2253657175656e6365223a3132333435362c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d028259014e7b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a22313031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3938373635343332312c22466565223a223132222c22466c616773223a323134373438333634382c224c6173744c656467657253657175656e6365223a38333734393136352c2253657175656e6365223a38323337363331392c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d5901437b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a2231303031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3132333435362c22466565223a223132222c22466c616773223a32323232322c224c6173744c656467657253657175656e6365223a3132333435362c2253657175656e6365223a3132333435362c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d"
        );
    }

    #[test]
    fn test_xrp_batch_sign_request_decode() {
        let sign_data: Vec<u8> = hex::decode("a301d825509b1deb4d3b7d4bad9bdd2b0d7b3dcb6d028259014e7b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a22313031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3938373635343332312c22466565223a223132222c22466c616773223a323134373438333634382c224c6173744c656467657253657175656e6365223a38333734393136352c2253657175656e6365223a38323337363331392c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d5901437b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a2231303031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3132333435362c22466565223a223132222c22466c616773223a32323232322c224c6173744c656467657253657175656e6365223a3132333435362c2253657175656e6365223a3132333435362c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d028259014e7b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a22313031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3938373635343332312c22466565223a223132222c22466c616773223a323134373438333634382c224c6173744c656467657253657175656e6365223a38333734393136352c2253657175656e6365223a38323337363331392c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d5901437b224163636f756e74223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c22416d6f756e74223a2231303031222c2244657374696e6174696f6e223a227244757239675336446a717277476e50526134525a656950706b717476324766326d222c2244657374696e6174696f6e546167223a3132333435362c22466565223a223132222c22466c616773223a32323232322c224c6173744c656467657253657175656e6365223a3132333435362c2253657175656e6365223a3132333435362c225369676e696e675075624b6579223a22303342393145313645393842413836423632413532414141324434314331313442333643384246434438363242314543454437374443354437373637363531304638222c225472616e73616374696f6e54797065223a225061796d656e74227d").unwrap();
        let xrp_batch_sign_request = XrpBatchSignRequest::from_cbor(sign_data).unwrap();
        assert_eq!(
            xrp_batch_sign_request.get_request_id(),
            Some(hex::decode("9b1deb4d3b7d4bad9bdd2b0d7b3dcb6d").unwrap())
        );
        let first_sign_data = xrp_batch_sign_request.get_sign_data()[0].clone();
        let json_value: serde_json::Value = serde_json::from_slice(&first_sign_data).unwrap();
        assert_eq!(
            json_value.get("TransactionType").unwrap().as_str().unwrap(),
            "Payment"
        );
        assert_eq!(
            json_value.get("Amount").unwrap().as_str().unwrap(),
            "101"
        );
        assert_eq!(
            json_value.get("Destination").unwrap().as_str().unwrap(),
            "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m"
        );
        assert_eq!(
            json_value.get("DestinationTag").unwrap().as_u64().unwrap(),
            987654321
        );
        assert_eq!(
            json_value.get("Flags").unwrap().as_u64().unwrap(),
            2147483648
        );
        assert_eq!(
            json_value.get("Account").unwrap().as_str().unwrap(),
            "rDur9gS6DjqrwGnPRa4RZeiPpkqtv2Gf2m"
        );
        assert_eq!(json_value.get("Fee").unwrap().as_str().unwrap(), "12");
        assert_eq!(
            json_value.get("Sequence").unwrap().as_u64().unwrap(),
            82376319
        );
        assert_eq!(
            json_value
                .get("LastLedgerSequence")
                .unwrap()
                .as_u64()
                .unwrap(),
            83749165
        );
        assert_eq!(
            json_value.get("SigningPubKey").unwrap().as_str().unwrap(),
            "03B91E16E98BA86B62A52AAA2D41C114B36C8BFCD862B1ECED77DC5D77676510F8"
        );
    }
}
