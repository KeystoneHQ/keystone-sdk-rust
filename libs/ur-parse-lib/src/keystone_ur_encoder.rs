use alloc::string::{String, ToString};
use core::fmt;
use ur_registry::error::{URError, URResult};

pub fn probe_encode(
    message: &[u8],
    max_fragment_length: usize,
    ur_type: String,
) -> URResult<UREncodeResult> {
    let mut encoder = ur::Encoder::new(message, max_fragment_length, ur_type.clone())
        .map_err(|e| URError::CborEncodeError(e.to_string()))?;
    if encoder.fragment_count() > 1 {
        Ok(UREncodeResult {
            is_multi_part: true,
            data: encoder
                .next_part()
                .map_err(|e| URError::UrEncodeError(e.to_string()))?,
            encoder: Some(KeystoneUREncoder::new(encoder)),
        })
    } else {
        let ur = ur::encode(message, ur_type);
        Ok(UREncodeResult {
            is_multi_part: false,
            data: ur,
            encoder: None,
        })
    }
}

pub struct UREncodeResult {
    pub is_multi_part: bool,
    pub data: String,
    pub encoder: Option<KeystoneUREncoder>,
}

pub struct KeystoneUREncoder {
    encoder: ur::Encoder,
}

impl fmt::Debug for UREncodeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.is_multi_part)
            .field(&self.data)
            .finish()
    }
}

impl KeystoneUREncoder {
    pub fn new(encoder: ur::Encoder) -> Self {
        KeystoneUREncoder { encoder }
    }

    pub fn next_part(&mut self) -> URResult<String> {
        self.encoder
            .next_part()
            .map_err(|e| URError::CborEncodeError(e.to_string()))
    }

    pub fn current_index(&self) -> usize {
        self.encoder.current_index()
    }

    pub fn fragment_count(&self) -> usize {
        self.encoder.fragment_count()
    }
}

#[cfg(test)]
mod tests {
    use crate::keystone_ur_encoder::probe_encode;
    use alloc::vec::Vec;
    use hex::FromHex;
    use ur_registry::crypto_psbt::CryptoPSBT;
    use ur_registry::traits::RegistryItem;
    use ur_registry::icp::icp_sign_request::IcpSignRequest;
    use crate::alloc::string::ToString;
    #[test]
    fn test_encode() {
        let crypto = CryptoPSBT::new(
            Vec::from_hex("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa")
                .unwrap());
        let result: Vec<u8> = crypto.try_into().unwrap();
        let result =
            probe_encode(&result, 400, CryptoPSBT::get_registry_type().get_type()).unwrap();
        assert_eq!("ur:crypto-psbt/1-3/lpadaxcfaxiacyvwhdfhndhkadclhkaxhnlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbnychpmiy",
                   result.data);
        if result.is_multi_part {
            let mut encoder = result.encoder.unwrap();
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/2-3/lpaoaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaylbntahvo",
                       next);
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/3-3/lpaxaxcfaxiacyvwhdfhndhkadclpklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypknseoskve",
                       next
            );
            let next = encoder.next_part().unwrap();
            assert_eq!("ur:crypto-psbt/4-3/lpaaaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbayneieyksn",
                       next);
        }
    }

    #[test]
    fn test_encode_icp_sign_request() {
        let icp_sign_request_encoded_data = "a5011ae9181cf3025820af78f85b29d88a61ee49d36e84139ec8511c558f14612413f1503b8e6959adca030105d90130a20186182cf518dff500f5021af23f9fd2046a706c756777616c6c6574".to_string();
        let result = probe_encode(
            &Vec::from_hex(icp_sign_request_encoded_data).unwrap(),
            200,
            IcpSignRequest::get_registry_type().get_type(),
        );

        assert_eq!("ur:icp-sign-request/onadcywlcscewfaohdcxpeksyahpdttplehswygatejtlrbwnnspgycegomybbhsdkbwwngdfrmninhkpmsgaxadahtaaddyoeadlncsdwykcsurykaeykaocywzfhnetdaaimjojzkpiokthsjzjzihjychfmiave",result.unwrap().data);
    }
}
