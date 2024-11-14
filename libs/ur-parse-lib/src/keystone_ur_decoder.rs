use crate::ur::UR;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use ur::ur::Kind;
use ur_registry::error::{URError, URResult};
use ur_registry::registry_types::URType;

pub fn probe_decode<T: TryFrom<Vec<u8>, Error = URError>>(
    part: String,
) -> URResult<URParseResult<T>> {
    let mut ur_parse_result = URParseResult {
        is_multi_part: false,
        progress: 0,
        ur_type: None,
        data: None,
        decoder: None,
    };
    let decoded = ur::decode(&part).map_err(|e| URError::UrDecodeError(e.to_string()))?;
    match decoded.0 {
        Kind::SinglePart => {
            ur_parse_result.is_multi_part = false;
            ur_parse_result.progress = 100;
            let ur_type = get_type(&part)?;
            ur_parse_result.ur_type = Some(ur_type.clone());
            let ur = UR::new(ur_type, decoded.1);
            ur_parse_result.data = Some(ur.parse()?.1);
        }
        Kind::MultiPart => {
            ur_parse_result.is_multi_part = true;
            let mut decoder = ur::Decoder::default();
            decoder
                .receive(&part)
                .map_err(|e| URError::UrDecodeError(e.to_string()))?;
            ur_parse_result.progress = decoder.progress();
            ur_parse_result.decoder = Some(KeystoneURDecoder { decoder })
        }
    }
    Ok(ur_parse_result)
}

pub fn get_type(part: &String) -> URResult<URType> {
    let part = part.to_lowercase();
    let strip_scheme = part.strip_prefix("ur:").ok_or(URError::NotAUr)?;
    let (type_, _) = strip_scheme
        .split_once('/')
        .ok_or(URError::TypeUnspecified)?;
    URType::from(type_)
}

pub struct KeystoneURDecoder {
    decoder: ur::Decoder,
}

impl KeystoneURDecoder {
    pub fn parse_ur<T: TryFrom<Vec<u8>, Error = URError>>(
        &mut self,
        part: String,
    ) -> URResult<MultiURParseResult<T>> {
        let mut ur_parse_result = MultiURParseResult {
            is_complete: false,
            progress: 0,
            ur_type: None,
            data: None,
        };
        self.decoder
            .receive(&part)
            .map_err(|e| URError::UrDecodeError(e.to_string()))?;
        if self.decoder.complete() {
            let cbor = self
                .decoder
                .message()
                .map_err(|e| URError::UrDecodeError(e.to_string()))?;
            match cbor {
                Some(cbor) => {
                    ur_parse_result.is_complete = true;
                    ur_parse_result.progress = 100;
                    let ur_type = get_type(&part)?;
                    ur_parse_result.ur_type = Some(ur_type.clone());
                    let ur = UR::new(ur_type, cbor);
                    ur_parse_result.data = Some(ur.parse()?.1);
                }
                None => {
                    return Err(URError::UrDecodeError("cbor is none".to_string()));
                }
            }
        } else {
            ur_parse_result.data = None;
            ur_parse_result.is_complete = false;
            ur_parse_result.progress = self.decoder.progress();
        }

        Ok(ur_parse_result)
    }
}

pub struct URParseResult<T> {
    pub is_multi_part: bool,
    pub progress: u8,
    pub ur_type: Option<URType>,
    pub data: Option<T>,
    pub decoder: Option<KeystoneURDecoder>,
}

#[derive(Debug)]
pub struct MultiURParseResult<T> {
    pub is_complete: bool,
    pub progress: u8,
    pub ur_type: Option<URType>,
    pub data: Option<T>,
}

impl<T: fmt::Debug> fmt::Debug for URParseResult<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("")
            .field(&self.is_multi_part)
            .field(&self.progress)
            .field(&self.ur_type)
            .field(&self.data)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use crate::keystone_ur_decoder::{probe_decode, MultiURParseResult, URParseResult};
    use alloc::string::ToString;
    use alloc::vec;
    use alloc::{string::String, vec::Vec};
    use ur_registry::crypto_key_path::CryptoKeyPath;
    use ur_registry::crypto_key_path::PathComponent;
    use ur_registry::crypto_psbt::CryptoPSBT;
    use ur_registry::ethereum::eth_sign_request::EthSignRequest;
    use ur_registry::sui::sui_sign_request::SuiSignRequest;
    use ur_registry::sui::sui_signature::SuiSignature;
    #[test]
    fn test_decode_psbt() {
        let ur = "ur:crypto-psbt/hdcxlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypkvoonhknt";
        let result: URParseResult<CryptoPSBT> = probe_decode(ur.to_string()).unwrap();
        if !result.is_multi_part {
            let crypto = result.data.unwrap();
            assert_eq!(
                "8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa",
                hex::encode(crypto.get_psbt()).to_lowercase()
            );
        }

        let ur1 = "ur:crypto-psbt/1-3/lpadaxcfaxiacyvwhdfhndhkadclhkaxhnlkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbnychpmiy";
        let ur2 = "ur:crypto-psbt/2-3/lpaoaxcfaxiacyvwhdfhndhkadclaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaylbntahvo";
        let ur3 = "ur:crypto-psbt/3-3/lpaxaxcfaxiacyvwhdfhndhkadclpklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypklkahssqzwfvslofzoxwkrewngotktbmwjkwdcmnefsaaehrlolkskncnktlbaypknseoskve";
        let result: URParseResult<CryptoPSBT> = probe_decode(ur1.to_string()).unwrap();
        if result.is_multi_part {
            let mut decoder = result.decoder.unwrap();
            let _result: MultiURParseResult<CryptoPSBT> =
                decoder.parse_ur(ur2.to_string()).unwrap();
            let result: MultiURParseResult<CryptoPSBT> = decoder.parse_ur(ur3.to_string()).unwrap();
            let psbt = result.data.unwrap();
            assert_eq!("8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa8c05c4b4f3e88840a4f4b5f155cfd69473ea169f3d0431b7a6787a23777f08aa",
                       hex::encode(psbt.get_psbt()).to_lowercase())
        }
    }

    #[test]
    fn test_decode_eth_sign_request() {
        let ur = "ur:eth-sign-request/onadtpdagdwnbstbpfkidafxlbprqzdiktfldlaxheaohddlaoweaalalrhkisdlaelrhkisdlcwlfgmaymwvttkvsptykhkfwswosbdlrhhtiknftkihsnbfxdalnhtwfbeknfzaelartaxaaaaaaahtaaddyoeadlocsdwykcsfnykaeykaewkaocyjokbwejzvdrtpssp";
        let result: URParseResult<EthSignRequest> = probe_decode(ur.to_string()).unwrap();
        if !result.is_multi_part {
            let crypto = result.data.unwrap();
            assert_eq!("02ed04808459682f008459682f1b82520894e0cfe8a9f55942c6a70b845cd07a3a7d61a04325865af3107a400080c0",
                       hex::encode(crypto.get_sign_data()).to_lowercase());
        }
    }

    #[test]
    fn test_decode_sui_sign_request() {
        let ur3 = "UR:SUI-SIGN-REQUEST/1268-2/LPCFAAWKAOCFADTTCYBENTKOSPHDWLATROUTDAGDSAIOSPYKDYRTDLBDSAGWCAENTPNNVWMOAYCEWETPIDYTTADWGHBZWFKBHFGROTHLOEAHRSDLFRADKNGHAAVLGOAHHDEMMWYLBZBKWLLKDKGESBHFAHNBLSWLOXAALUBWFLSRCTWELFVOKKLEURENZTCSTIPRNTRSOTGEZTHNGLDIWTMYDSQDFDMDWYPDIYTEMKMSFMFHETSALFCAKIGUCFCTGDFDLNSOSFZEGLSSCTJSGEPSZCSTEYNSYTFESGJPGTDETSFDWETOCESNUEWSYAQZYANELGZCOEAOAOADAEDPDYAEADAEADADAOAXLYTPAXDYWZGABNTTRTRPOLWKCYAMLBNBLNONMWESETIATOSEAXKNYNTELUDWLSESYTUOAENERYZEWTQDWKQZDMLUAETOGYGTIHGMHTFZBNWTSOPYMWZEPRHTQDKTSBCFGMURIDLTWTWPLPUEWFKN";
        let ur1 = "UR:SUI-SIGN-REQUEST/418-2/LPCFADOEAOCFADTTCYBENTKOSPHDWLONADTPDAGDSAIOVSSSWFIAFGGLNEDKSKFTPYYAGWVTAOHKADJPAEAEAEAEAEAXADAEMNHHCEBKDIBDRLFRCPKNTDTTJOCWDISFRYWEYKADGMGSLEIDLTHKJSBTKSKOLTJNWDOERHAHAEAEAEAECXHPKELEURENZTETDPRKRKLYEEBZPYFTNDPRCFCAJYSEWNEEGMTNKGLONYGASODRIHAEAYUOAEAEAEAEAEAEAEAECXRYWNEOBZLFECYTKGMSOLGDSRFNHFSTASGYDACYFMSATKRFOYZSBBISCMNTLGZCOEAOAOADAEAEADADADAEADADAOAEAEADAOAEGDFDLNSOWPFXRSYLBKWFLBGOLNGDMWSFFTKKNSREFYKKWZGMTEBNTEJSLBBZWPUOAOASZSFZLUVYPDUYMWAOJEEEYTVDCKESZEOECYOSFLMYSNGWHEPYTOKEADPKBDVOLRWPGMBDRKJS";
        let ur2 = "UR:SUI-SIGN-REQUEST/1072-2/LPCFAADYAOCFADTTCYBENTKOSPHDWLOERHAHAEAEAEAECXEHSROTINFEHLJETPBNJKIYPKJPBKFEWPPKIDYTTADWGHCMWZKBTPCHRSHGLPBAAYBBCFKGPDLPJYYAJPSOVWTNHSYNFLFGIAWYOTBWRDHPKITBAALRGLOLEYCMFLSRCTWEOERHAHAEAEAEAECXZCASDSFMMSHEHGHTTLMDWLMOGMJPRHOYRFJPCAHPAOUEYLBZHLSALESEKIGUCFCTGDFDLNSOWPFXRSYLBKWFLBGOLNGDMWSFFTKKNSREFYKKWZGMTEBNTEJSLBBZWPUOWYAOAEAEAEAEAEAEAEDPEHADAEAEAEAEAEAXLYTAADDYOEADLECSDWYKCFAXBEYKAEYKAEYKAEYKAOCYGMJYFLAXAALYHDCXGDFDLNSOWPFXRSYLBKWFLBGOLNGDMWSFFTKKNSREFYKKWZGMTEBNTEJSLBBZWPUOAHIHGUKPINIHJYAESTZSVEGA";
        let result: URParseResult<SuiSignRequest> =
            probe_decode(ur1.to_string().to_lowercase()).unwrap();
        if result.is_multi_part {
            let mut decoder = result.decoder.unwrap();
            let _result: MultiURParseResult<SuiSignRequest> =
                decoder.parse_ur(ur2.to_string().to_lowercase()).unwrap();
            let result: MultiURParseResult<SuiSignRequest> =
                decoder.parse_ur(ur3.to_string().to_lowercase()).unwrap();
            let sui_sign_request = result.data.unwrap();
            assert_eq!("00000000000301008e5c1c0a270bb73b227ad2d1701b27ccbdedf501524c8a628759710d7876876deaa2b90500000000205b7c8adf36fc382dbbbb813415ab3a9bb2191d74c1f13452da7b889a49c92a650008dc000000000000000020bdf133158235f97b97a650c33c56c70951251a3ec2cfbca1fa1468169d8dfda20202010000010101000101020000010200504886c9ec43bff70af37f55865094cc3a799cb54479f252d30cd3717f15ecdc0209fa408be1a8db94026b34f9e71e39fea21aa7478fcd4f5fabce7c01aa0be284eca2b905000000002031c3a369455d6bd80c7366aa720a45ecaa62f9d92c5416f27ed817bf57850e0814197ba88574f872c9e5da61f6474663eea313ba5b7dd604844ea6321647c31feda2b9050000000020fd09263e975f575ad595e9925272b9a1bc721d5b02def7155dc28ac17d53191f504886c9ec43bff70af37f55865094cc3a799cb54479f252d30cd3717f15ecdcee02000000000000002d31010000000000",
                       hex::encode(sui_sign_request.get_intent_message()).to_lowercase());
            let components = vec![
                PathComponent::new(Some(44), true).unwrap(),
                PathComponent::new(Some(784), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
                PathComponent::new(Some(0), true).unwrap(),
            ];
            let source_fingerprint = hex::decode("52744703").unwrap().try_into().unwrap();
            let crypto_key_path = vec![CryptoKeyPath::new(
                components,
                Some(source_fingerprint),
                None,
            )];
            assert_eq!(crypto_key_path, sui_sign_request.get_derivation_paths());
            // request id
            assert_eq!(
                "c267e8c4f363464e9f24c53aabf84fe0",
                hex::encode(sui_sign_request.get_request_id().unwrap())
            );
            // addresses
            let addresses = sui_sign_request.get_addresses().unwrap();
            assert_eq!(
                vec!["504886c9ec43bff70af37f55865094cc3a799cb54479f252d30cd3717f15ecdc"],
                addresses
                    .iter()
                    .map(|a| hex::encode(a))
                    .collect::<Vec<String>>()
            );
            // origin origin
            assert_eq!("Suiet", sui_sign_request.get_origin().unwrap());
        }
    }
}
