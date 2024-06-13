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
    use ur_registry::bytes::Bytes;
    use ur_registry::crypto_psbt::CryptoPSBT;
    use ur_registry::ethereum::eth_sign_request::EthSignRequest;

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
    fn test_decode_keystone_cold_wallet_ur() {
        let ur1 = "UR:BYTES/6037-3/LPCFCHMDAXCFAAGWCYDSSBCHDAHKADJOOLCMWPKPGUFRDEFYBNTEURKTLYMTSPIDJEEHOXCMIHGWEHCAIMKTGOLUTTTSKICKJNKGPLAYBGGLMNNTEOMNSAVAEMTBVDMKFRPYESVTLAZCKEWKSWNLVOTTUYYLOSWFFLJLGMCPCKSKBBKOBBCHJKNDIMTBJZJYLPSNRPGRFGTOYKWZSGCTYLWYMERPPSBZHHHHTYWMNBLGUTBBTBPLTBFDEHWMTDRNCWLOENTLGEEMCNOYGUHNPKSWFWWKVABYTDCXFLETEHGWDRTLMUWYROJNFYPYFGHSSNNLCNOECSLTVAMSLTBZPABSNYCTGLVDMYURISBEZTWZDECEIMBSSGRKTDAEHYHDTDPYDMWYSKEYSTEYPRFGQDJZOTTPMHFDRKFMVYHGPMSBRFNLLFRPLKQDHDKIRYNDLUFPVDJPESDMHPKIHPFDLGOSPRGMMUDNFXLDZOLYGATEWYVSSGFZASJYNSDKHGDPHTGOWNBYTOFPURLRWFCLOXBTWNZMGOCTNNFLNYRHLEHDKNHPAXLDIODNGMDMVYCECMTIVYTNUTYNPKBNCPDREEEMKBBAMDCYVWLRDMFYDEUEBGRLCFATGMSGIMGHREMEAERSVSSKONYLMNPYLAOXLGZTSNFWVYHLNELSCXTEKIVYNNZOAXPRBSCPSTGSLNMODRLTVAVYLDUTUEKGIDVSRKGTRHRNRPPTZTVOKTASCNFGRYIMCLCEFDHPSFDKKIVWETSANSMH";
        let ur2 = "UR:BYTES/6109-3/LPCFCHUTAXCFAAGWCYDSSBCHDAHKADJOHKAAGSCTLUAYAEAEAEAEAEAEAEHLTYGATOUOFYAEAHHNREHNBYKPFZFZHFGYHFWPHNCHFNQZWEHTTNDMBSJSRLLGVDOYEMMEVDASTKQDEMBNDNSSCXKODWROADCHVLAOHHAEZMGELAZTMEWDAEHERYYLPKNNNSNNKIGMSWUYDYENKPZTKKTSLTGTBBRSYAWFWLMUTEQDDIAADKVTHGBNSKRNYAVLWLYTOSTEYTAXSWIEFHFSRYYAWYJYKBDRRSSSSECHDLRSKNETTKNYRENTAOLFGACMADWNMOGSMHMUFYHFRTNYBSHKRNGREOFHOYLUHSTPJTMSMHEMRTVLNYCNRTPYWSIHBEIAMDJETNDNMOCYGLPFFWAOBTYKPROEUYJSPRTEWLTAFGNTKKIMLOBYJPIMCWSABYPTPEDSIOONBKBWCLFGRPAEWZYTWLZTUYWLKEFMDYPEJLNSFZQDUERSCPZTPALODPTAPMSEEEPFVDETBSWSLSGRDPBZTDLNJPLDVDCXTDCAKOFEWMFWSTRTAOWYGDPLTTGDSOWEHDPLVYSWGRPLBNGTSKSSSFONKOWPDRFZYTFEGESGEHWTDILBPRPEGSFNJKJNFEIMCLCXJLIAGEHYPELSVWCNSNCLZSZCJYZEWPFPOXTIVADNNDKGJNJOLFWFSGKEBDOTUELSEHYTMKOXHLDSBZYLHNROPFRYHFVYEEGAMHKEBYIOCMCNMWDPSKTTLDBTRNLKTAWE";
        let ur3 = "UR:BYTES/6161-3/LPCFCSBYAXCFAAGWCYDSSBCHDAHKADJOZMBGNBIMTPEODEFYBNTEURKTLYSBCEDNONWEVTCMHNDLLRKIKGAOBZSBLTLNDNWZBTJZMORFZMBBGHQDFNZMKPJETIKTTIASUOOEYNGURLWNHGDYVAWSTOINTNVTFYWNCWJLPMISNNESLPNSBBFDTOJZRTFDWTWDYANEJOMHKOYALABAQDSPJOOTLPASGHVARETKATHDLTLDYTWKLYOEBWYNSOAYFRFLRFHPTKTBLKGUCENSWYMKFYPRFNUEHKTLBWEMISLTGRKTVWGWDSJKRDWSBTRYFLMHHETLQDEHHHTTDSBTLOGSBSFYPTCXWSJZWYATAMLTJZSKVSZMWTDWBKBEFSIHGLFRFLRTWKSKHGDELDLFWTFYRNNLBYKNGRESASWEAYMNWMHFSKWFBKOSZETAFXRSPSEYDKIOLATSEOFSKNFRWEFDLBHYHPPLFDSAFHRLSBDMDSGWPLHPBBZMDNLODPPSKNWKYLMWSKOYDTKKTIJEJOIMLDCSCNKKDIMTKKIOFDOXZTCAMECFSSGAIHSKAOLADYGSURFSRHJYFNDYVYOEDMIOWNWFPRPYVLYNTKSSTSADIDBBCNFLFMKSVTIHDSISSWZTFETLSOVWSGMWSSYKDLDIISURAEIACWNBWMKISFMOTAEHKSTIMKSOIDGMFERSGSWYCLDKETIDRODKFGURFHTOPLRDTAAMAMBBPKAXFXFZQDFTPSBTEMFHUOKOASYKWKVSEHCXDYHT";
        let ur4 = "UR:BYTES/6207-3/LPCFCSFHAXCFAAGWCYDSSBCHDAHKADJOUEUOPYNSDADNGAMUNTMKVLIDTLTPKIFXHDSRONHKKKCEIDSGBGMTFEDTHHNYIEPARELESRAHGYCSGSIAMKLUPSGDKNVAMKTYNTBWFSKEHTIDFMAHWMNNFMEEYKWMBYLGPTLBPMFNPLBKRFCWBALRNBSWFNMDOEDKSGTAGRDWINRESSCKDYCAWEUYTLWDPSTIFXIYJZWSGSJLETRPUOTTEHRLCMLPGYGEPLWMTBETIYGONSBAJTDPNNSOMOBYTTRDFYMKAXBBLPPYKOVYRFKPBGLDAHPAYADTTEAXFXFYINMWHFRHRLSBJSKPYKRYVWVTWSSWIAAHCFBNVARTRFKNKPNSHPHNNSPENDWEIDJKDMASMNSNHDSAAMOENNGWTPFTNLTAJOSGWSEYDAWSGRRLMODSSFWLRFGUEYTSBWLNIDBKATDINTFETLVDLFJZAMGYNDUYINOSNLCNUOFLSOMWLNLUGEHLMKIETOBATOMYATATKBJNCKNSJLTKVDMSDLUYVANDTDMYNYWNDPLPWZZONDMUVALPDSRELNIMAHEEGTKKBAOYSFGYCPCPONIALGCHIOPSCMRNUREHLAGAFDCLADKNHYLDWKLFBEVTMNADSWFGGTNLLEBEBDPELRJZPEDWGWDAWFPDRHSPSBVSDSSNWPKNGHPYROBYMOMTONEOECUOBEFSFHKIYTTSURDIWKOTZMFMJOREJLZEADNTAEPMVDTTAHAEAEAEWSLYKEJS";
        let ur5 = "UR:BYTES/6251-3/LPCFCSJEAXCFAAGWCYDSSBCHDAHKADJOCLTOBDYNZCCSHSTSMEGRFNBZGHBWHSISZCDMFEGWCFEOVARLINMWGDVOUYCEGWFXROVAGYRHPLBNCSTIOXJYTAFRPKMEFDUTFPPASBDLWEMUINECBTJSWTHLDLBDGOKEPRBEAEGHDYEOESLTCYSFJTPKZTUTGMTOEYFGFRRFCTGTFYBELSTLNTKSGDVLYAENYNPTJERLSBVASEFWHLJKCPFPURLGIMBTBGPFCFWYWDAMLAMOLARETNKGPLTKLOJLHGPEJEMUTOUOMUPLNYAMPDIYAYBNRSRHLKTBWTKPECFEJOQZFHLTKBEHHHNTBKLKADSEIHLFKPSOBAFHGSHFLBLKIYAHTDMWUODPMTRPKKCLATGWPDLNROFRMYECMUAXMHEEKSFYAAIEVTCEFPBEJZZMMYHFBEHSCMPFMUGYGYEMKICEJOBTPKRHTASAGLMUOXJZOELDRSJZJPCEUTJEPMAXIOWNVOMHESNYBDDMDMKBPLAMJTYNVATSSSWYAYGTNEZTNYDNIYWPRFNSENPRZEHFVEAHCMYTHKHGRFFZJSGAWSAXVOENTETTCHSPJTVYPDISSERSRYDAOTBAKOHKVYCTKSVYEYKBGOECFLVEBNTDLDJZONEMIAJOLRBSQZLKOXHDFHFTHNYTQDETRNAAMNDEBYBBWKZMQDPRNTGYLGYAHFVOAEQDHGJNAMCLWZRLGOFSEOYKUOSSPMMHEMMOFROSBNYKWKVSOSEMLRIN";

        let result : URParseResult<Bytes> = probe_decode(ur1.to_lowercase()).unwrap();
        if  result.is_multi_part {
            let mut decoder = result.decoder.unwrap();
            let _result: MultiURParseResult<Bytes> =
                decoder.parse_ur(ur2.to_lowercase()).unwrap();
            let result: MultiURParseResult<Bytes> = decoder.parse_ur(ur3.to_lowercase()).unwrap();
            let result :MultiURParseResult<Bytes> = decoder.parse_ur(ur4.to_lowercase()).unwrap();
            let result:MultiURParseResult<Bytes> = decoder.parse_ur(ur5.to_lowercase()).unwrap();
            let keystone_cold_wallet_bytes = result.data.unwrap();
            assert_eq!(true,result.is_complete);
            assert_eq!("1f8b08000000000000005dd449cedc44000560b56011754040565156ec60173cb4ed5ada2e0f71b78de7a13791e709cfb3370c2bc420762cb80117e3025c00ff4a80fc91ea005fbdf7aa9e9c9e7d52c6db303675fc79d7874d14bff8f3e993d3b3270424e0570cc5bef8e3e9f9a7d3f903c6643f3dbdf8ee747e2abfc4c1172fbf7a38cf9ab59d0282491601f1924c90934456c09a0f59be4b333fa18b61d86e979037c0e39a23c0abef651063956bda2b921a4eb042020df5b2a2db71b2d3e9d9469d796a8811726a1bc211a9af2667a50a132146b600f2f9e9fcdbe97c3e30af6f9c40b3debf22fcb1882dd9adc134b0e7380fef834b2d15d2867289e720d21d7645eb42c7c002ee50aed150c9ed58aee1c64bae0c4dc5c4cca576ec2a40f9454aca31f0277fb2af4c3c736d456a21206f634a5eaf83e523cd21fafd74feec41a4d0e62b9b7b6d7082f3ca7c0ba3de8331f998a45d2615f760b8b0bd56e13449907c11671623942dc5d1890da616ec75533b28440cd3df778196c8626b31a416654f311d6a77558bd1d77d1e6d7bae08124e8e9d338ec2e637d6e7983bab39e080fd7cf4c699e2d1dbf7a7f3476f52221ec514761417739b6ad66c7485cdb64b46cef5f2ca1ff7ee91b6ac155c5cd4eba08ddd14d6aed64831ebd2be1b8836d54a3723a15360aac642f4e611d2204738314f2ad593eeb86d44ab4661cd9923a21887e6978715b10f9a1f4ee78fdf6810fcf2281c6a0fcabbd2005e58d2ab2eeec532c732b246b36ca3d89048bb3ee157adcbbc9982b68cb3587dbd9b8b41e772392e5b7d5b488da7b252932b4389fb8149d3eee8ca4009749c24572d5a55f111ce41df84f321a40df1ff551f9e479ab98a587a5b0389672b522ee11c16d0e1daddf6aa0c222a34377e0e951ae5842e4428de12b7190752ca6a54b59100bfe8c5a5f78eab80a48dfccd42e15d9f8320d37de19efb03b20f22c74c86922a87e6e189ddde7b62e8bb4db9beb6a9fce277092346bd6a211c485bcc247de5dedcab9c252b49939d98e362d5d87d4358c3a559791c62ca129645295c9a64b1b58ac30551184c63988bac507ae698d49d133d7c5a623e05eb9e3e34f5eb118da97fad3cae0abc1b0e84a0c63c95a224cad94b2c69b5c41e301deddbd5eaacd043666cef4c6f38b6dcd131b71685514aaeebd63866559c0e6e2d9ec99211d1ba4498031485ab76e1bc75128905b1f829d3034344699456b9b7cb7175f5bde5e0efc66305190ce6c0bc7a759c5b609caf9bed62732e098ecd58c206a29e4fd83a99d970caef3225ef4bb79226cce9bc5332d71386620a07279d45d5e7826c06519bdb69a79923dc47c994868b4a5d9864ce0ece8f07077e6d1e9c6fcfe7972fdbe69bd28f9af12d85f2fb9b93e68526b5866a05344d790ea1cc512222a5638d1767ac16bedf3180494821017a5e89f48210e08e01c6464d998a100baf846caf2c4f25f3a8b9c8cbe826cdec7a54abb8119296a53335dc103d3f7df9d7df27f4a3ff3e70b56ffe019d00ade7d1050000",
                       hex::encode(keystone_cold_wallet_bytes.get_bytes()).to_lowercase())
        }
    }

    #[test]
    fn test_decode_keystone_cold_wallet_ur2() {
        let ur1 = "UR:BYTES/263-3/LPCFADATAXCFAAGTCYSNIMBGGTHKADJLHKAAGECTLUAYAEAEAEAEAEAEAEHLTYESPLUTJYBBAMJOHLFPWNJYBWAAOXLEGMTTFPYLVOTAZMTDQDIAURWMKSNNNYSPWFRECATKQDCWLNBKEHLOMNLFCAPFEHENRTAMWTGUAOVEFEFTBDYAWLFRURESKTOSIONEMSSOENLKGTNTKETLYKGYBWDIDLZEKEJPKTKNKOATSETPGRCYOSNLCHLBFNESZMJYFTLBFWNDSFCHOSCHURNTTOGWPLYLDEYAZSZEVWSRFNJETBKOBKJSRDLUKEGRCAFGAEMWGAPSGHSPGHCPSWGTFLNYRPLEMWDKDWDRWFRYGHWTDMJPCAOXVYSNISFZHEEEHTIAURHHTBHPOEBBONFLIDNLHSIYTPPTNSKOTEZEMTPMTPTBFDGEFXKOASSSUYNYPKJZDSEMBALYCNRECSZTZTJYZEWEJYFMCTMKEMCHGLNBCSWSHEBYZSHDSSWPIDWYRPRNFNPYHYSNDWKPLKKTCAAMFPHPGTTDETPRTTIETKLKWMVTOYSENNGSSFPKKNGMLGTELRPSJOADFMPFGSPTGDAHLNLECSVWCXRKVDMETNSKTPAHRFGRAYSROSIOTAFPWZWPCMPYLFWLINCFWMPLPTPDCAOEURGWVDDLCTFYBKIHRNPRRHEMAMDIETPESFYLDYWZIAHDHPDECXPRVYSSDYSSKKOSBABBTDWDCALUKPLYOTURDWTSCXDEKNRTPDWZWZMWFY";
        let ur2 = "UR:BYTES/223-3/LPCSURAXCFAAGTCYSNIMBGGTHKADJLCHSFSOCMDRGRPKKOVTJYLPOXLPRNFWCNVOFYESJSDPGLTNWPRDRYGWONCLYAQDHDVLFSYNCSTLNBKSYKLYSOBWRTGSWDZMUTGUCYCXCSJOYTLARSFZMTMOBTGSGAZSESLGSSBECYBTNLFPVALRNSLBPSSGGLSNDEPAISVSOXYAZTWZCANDYKPEHYFPVLPYTOOSBEUOJPVEKBTBHHSFPMSSUOAAKPKKIHDKWYTKQZKGKKYKPKMEJPOTFZPTFGOSJEHECYWNADWPMECTSSCNLBIADYKBONIECEUYTOCTOEHTAHKGDPWMDLHKPLLDCMHTRKMEYTQDMDOETTASGUFPZSSBMTHTAYUTGEHPMTWZEYDKJPWDPRLSMDVLOLBEUTPTAAWLRDFWRHNTDPWPOEVLCLBZFPPDRSKOLUAMROAOCMWYMHKGFLFPESZTOEENOXRFWYSBCKROHHKSSBGHUTDYTIGAMTLNPAKNNNFGAEECAXTEJKDIMOURVYDLSARNAEDYPKDSUYQDNNBTMDGLGHGOFYWMCLIDNSMWJSZTJOCKRKPEMEKBWLMSLRHHGMSSURGMHNISCWTIMKEMRPGUTAFSMYFZWDATQDCELDWYGMQDVTLRSBSEDIWDMHRTFNIHECCECLATAHPKFNPLDKURSWLYKEMEOSLNTOAOONSAKPLPRHNEAMFHDLYTETBYFWGRTEJTGLVYGYPMOYGWNBTAWNTOZTFMAOPMJZFEJOGUHDRL";
        let ur3 = "UR:BYTES/178-3/LPCSPRAXCFAAGTCYSNIMBGGTHKADJLSTIHATJPVTWYLFLDDLFZPSCWDRWNTPGSLAYKLRDKDNFHGYTSFTEYDRSKVSOTLUNDDYLFLSHDIEDKGHRHTAPKYTLGGDCWMYJNPYOLVARLDWUEHFMTWSKEURFHDYRSCKTTMKZSJEVWWNPKRTOTJODYLUVOPEUELNEMETEOFEWDPMPYJKLBYKRHMEMTFSAXPSCKLECXBNHNOSKNCEHLKNRHLFFPPTWDHSIDIDNEOSTPGRIYRHLSGYBBVASNWZQZLUWPDWIDPMYNFPBEDTBGDAWDWFGYADBTHKKNLTLSMTIMSEVLKKJEUREMSTTLTLMYNYLSKBYALYDREMHHBBLYHDTDLOCWDYUYCYISHKSWMKFEDRBYFRDAECHYVAMESKGSOLTTLYKOIOEHFGLGVSAAHEWDLYLSSBJEGSWFHDRKWESKAABYIDFHGSAXHTCMJSEHKGWLLRPDEELETELSONKKETVEISJOBNKNBZNDLDZTVTZTKSJOTPTSVASEYTYNKERNRSJLNDRLIHBEEMVLKGBKRHSPIMSFSOFEVSOTBTGHCFAOMEDEKSFDLRYADWRDPMJSRKNYPYSSOYGAVWAXMOBSATLAFYWDTNMSWZPKHGMSPRCNTNCECXFYOSJTSKFTRKDAWDTSJOJNEHWFOXIHYACKWDJKOSREYNWYIMHPJODAPYKELDAMLFKGKBZSVAPERSGWWTTEZMCKROTNEMZMAEBAGDMNKOTTAHAEAEBDVELKAA";
        let ur4 = "UR:BYTES/134-3/LPCSLNAXCFAAGTCYSNIMBGGTHKADJLGLSPLSASOYFXPKKOVTJYLPOXLPVLMTCYGSNLGTIHDNFMLTPMGRSOHHOYLPJPVYLDOESGBBSEDRJPSBMTHYCPJEHYTBCPBNISGLTLMUAXYNWFPAEMTOBBMYRYKILBFTFHKIMSBGZEFDOTGECKJNOSNBMDRYWLPKRLDSOYUEDEREHSMNSPJTOXRFKKJTCATSRFTIIMPKKPDAOLNTFGJEEETEOTETGSLNBYCKMELGDLRLJTGMRYGLWSJNBSATPAMYMUONVEBBSATIZSSOPRDTBATARKAOWYKKHTUYHTHFBABASNDLBSDPIDCKEEFHNSTONERYTEFZDEYNCLDICLHHHYDRHPEYFDLFKBADYKDPJTWZDTFDOLDSTDLYFHJSRKJSPMKPSFMEFLBDLAEEJYPYJEHFEMOYKGPMBYPSTYDKCLVTBYHDWZHKSKAETBSPGASPTITYLNMYGRENJEGSEYJLSEQDTOFWHLCSJOWTRNASPDLGRNBDVDGUMTEYSSZMHPKIKSCKINIDZSSACFONQZWKLPKPJNPLENWYCNJSOTNYCHURMHFZHKUYDPBNHGFWGOGELPFDNBEMASWEJKLUUOLYSSFDDTNBTYSKSPCERNONGRAMCPPDFMADFMINMWKSMSSRJTVTDRREKSOXFPHSJYETGRMSLARNHSTOGMWZLTVAVYSSDMCTNTCSZTCLLNEYJYHNHTEORKPFDRFTCLKNDMVODNCKDRTSPSWEKOPLNDZS";
        let ur5 = "UR:BYTES/72-3/LPCSFDAXCFAAGTCYSNIMBGGTHKADJLSTIHATJPVTWYLFLDDLFZPSCWDRWNTPGSLAYKLRDKDNFHGYTSFTEYDRSKVSOTLUNDDYLFLSHDIEDKGHRHTAPKYTLGGDCWMYJNPYOLVARLDWUEHFMTWSKEURFHDYRSCKTTMKZSJEVWWNPKRTOTJODYLUVOPEUELNEMETEOFEWDPMPYJKLBYKRHMEMTFSAXPSCKLECXBNHNOSKNCEHLKNRHLFFPPTWDHSIDIDNEOSTPGRIYRHLSGYBBVASNWZQZLUWPDWIDPMYNFPBEDTBGDAWDWFGYADBTHKKNLTLSMTIMSEVLKKJEUREMSTTLTLMYNYLSKBYALYDREMHHBBLYHDTDLOCWDYUYCYISHKSWMKFEDRBYFRDAECHYVAMESKGSOLTTLYKOIOEHFGLGVSAAHEWDLYLSSBJEGSWFHDRKWESKAABYIDFHGSAXHTCMJSEHKGWLLRPDEELETELSONKKETVEISJOBNKNBZNDLDZTVTZTKSJOTPTSVASEYTYNKERNRSJLNDRLIHBEEMVLKGBKRHSPIMSFSOFEVSOTBTGHCFAOMEDEKSFDLRYADWRDPMJSRKNYPYSSOYGAVWAXMOBSATLAFYWDTNMSWZPKHGMSPRCNTNCECXFYOSJTSKFTRKDAWDTSJOJNEHWFOXIHYACKWDJKOSREYNWYIMHPJODAPYKELDAMLFKGKBZSVAPERSGWWTTEZMCKROTNEMZMAEBAGDMNKOTTAHAEAELUGTHHHK";

        let result : URParseResult<Bytes> = probe_decode(ur1.to_lowercase()).unwrap();
        if  result.is_multi_part {
            let mut decoder = result.decoder.unwrap();
            let _result: MultiURParseResult<Bytes> =
                decoder.parse_ur(ur2.to_lowercase()).unwrap();
            let result: MultiURParseResult<Bytes> = decoder.parse_ur(ur3.to_lowercase()).unwrap();
            let result :MultiURParseResult<Bytes> = decoder.parse_ur(ur4.to_lowercase()).unwrap();
            let result:MultiURParseResult<Bytes> = decoder.parse_ur(ur5.to_lowercase()).unwrap();
            let keystone_cold_wallet_bytes = result.data.unwrap();
            assert_eq!(true,result.is_complete);
            assert_eq!("1f8b08000000000000005dd439aedd741406705d41f1741304a48a52d141f7e2d9ffd2b363dfeb789e9ac8f3b51dcfb31b860a31888e821db03136c006f05302e4453a0bf8e93bdf3977a7679f97c9368c4d9d7cd5f55113272ffe7c72777a7607c1d84b1aa799177f3c39ff743a7f429bcc17a717df9dce4faef728f8fafee5c33c6bd6760a71ba8b7c4b1d46009449ac54c85422c64d479ab68a94242c2af3bd54f02e721da4e1cd68405f345a63df5cd65ba214a54762996166d8a99c76d3fe96add8d6484a437609c4db9aaa6c26370e8123b518fcfc74feed743e1f9837174ea018ef5f11fa58c4ec62eeb6be3cab5ecd2c758c771d06415b4dd238b2d164cf8cebe0a1c19e4cccaa7a528dd384ac70013eb04ca95005868a18e520bbe791dac5d805bc4b08c3a767d941f2ec16ab82e96919ebaea9a81da2df4fe72f1f440a65beb2b937062738afccf730f263585b2820b2e1c430c479a70e14d2ea1d8b7581a3df2cd720287ac0a84ec88309a143aa76e07485a485e3961a4c994d652b3e87ad4bc95ca18572e189a2ca14c12a72cb965e226b5ed6220c684ed59303f6f3b137ce148fbd7d7f3a3f7d9712fe48a34a1e6da7a095bde9aab726a1de28b5618ec86ea4bc796e1dd7bcd06aaa7525a69d466b34d3a3384c86111e918d2fb76e52bd4eef6d0f07b18f93a5e414c2d0fac9b2290ed9bb02ee795adb5a560e0ecd2f0f2d621e343f9cce9fbdd34028f62127215c5e2a5b3248827e01f52d6ef22948a626d2813f71bb71ad75cc91470b803474ab6b5637a17bad11acd42421e01158f259c500d6c849c8d0d4868f4b366b4c326fc1b3ce425d1870f0be09a88dbe0be7539632c4ff5b7d781e6962fac219a5b4f485756dae36ee2371a39a17df904059db2d0c5742554a8548a03709ed738bdc81c44829a0d4c5c81cbea54b0622a83e013e69947897c36ee02ab578a4416174384b9780be61ce52f287e6e1c42e1f9d18fc21863274605a33bbb02a3a217a2ee22b1e2ad7acedc7650772e0ee82892f40ac1b2af1d84c80f584242b3f51d73a322ac5e8a38b9b30828358642454b9d9aaf98d501b8f6daba6e6b72cde5696ef7cdf3f30bf1ed198fa6be5f1aac0a370308be2afde8637383345eaadab737ff5b991963d03ac1e8a200c60a77a1c5d7ab98241a9ea6162629fa7d84b66b9835114e6cdf2b48bec2c62adf64110291225eaf351010d597a8783966ac1e3796bdf37c7d5d58f9a837ef8812a375c148158d2881b30db1a6859c698452a113b25355ee691c54ca6d181766731468de8045fea8183cb6b4cf358bbedc50411623f4c035a1671317be984a8348ad383a57938e468700c7a159b89fce0fc7870d8d7e6c1f9f67cbebf6f9bb7651037e37b0ab9c86accc945e8a30d5419029128784884f82cbaad71bb9aabc4a149e503920f078044eada97f2aa5797b223da1c2044a76ec53abb25ead7706d31f3a465f81eea73a7b5f6ee6a5b7025ab7c8906827b7efae6afbf4ff0d3ff1eb8da37ff000e508e76d1050000",
                       hex::encode(keystone_cold_wallet_bytes.get_bytes()).to_lowercase())
        }
    }


    #[test]
    fn test_decode_hot_wallet_eth_tx_ur() {
        let ur1 = "ur:bytes/34-2/lpcscpaocssgcytbhlzsadhdihhdspctluayaeaeaeaeaeaeaxvlhnbgvobsdwjptogwgagoaydesgdlsogwtotkmehtsrspsedksshsisieimvtievavepseemsmelutaecssfxfddaehdasossdytafnhktstpsasahdtsdksnspgdemdtttethkemsotysartdettcpdadttacxgogerthgurssfyhlurmyqdfgfd";
        let ur2 = "ur:bytes/26-2/lpcscyaocssgcytbhlzsadhdihhdspctluayaeaeaeaeaeaeaxvlhnbgvobsdwjptogwgagoaydesgdlsogwtotkmehtsrspsedksshsisieimvtievavepseemsmelutaecssfxfddaehdasossdytafnhktstpsasahdtsdksnspgdemdtttethkemsotysartdettcpdadttacxgogerthgurssfyhlurzotttbds";
        let ur3 = "ur:bytes/17-2/lpbyaocssgcytbhlzsadhdihsffzhlctlrwkbtbbhfsnwdjljneetdbeprbkvwtdeypdfdgrecgeeneyengegsjsjseoeoeoecrpjyjpgrehgtehjygeehjseygeglrpeejyehamcfiymwisdkssjlielaaooxrobtgtsfgsdwsfgsgssfsnsnmdhdgtsflaidhtbnamlebkataoaeiesekbzoskaeaeaedevozeas";
        let ur4 =  "ur:bytes/3-2/lpaxaocssgcytbhlzsadhdihmwlofwmwlkwkbtbbhfsnwdjzmnghrtwzrydsmscekivycafxcalacfzokklrlsvtdnwtzowzbyjpbzcydlhppmgomoplntfeonuyskjladpfjpglfnhgpaoyvtwkrphdtatlkekntkbzcwisvyaacekgvwceyksfjllrcsfwoejputdkpecnuecpgodmaddtdkadfyhluraxhhrony";
        let result : URParseResult<Bytes> = probe_decode(ur1.to_lowercase()).unwrap();
        if  result.is_multi_part {
            let mut decoder = result.decoder.unwrap();
            let _result: MultiURParseResult<Bytes> =
                decoder.parse_ur(ur2.to_lowercase()).unwrap();
            let result: MultiURParseResult<Bytes> = decoder.parse_ur(ur3.to_lowercase()).unwrap();
            let result :MultiURParseResult<Bytes> = decoder.parse_ur(ur4.to_lowercase()).unwrap();
            let keystone_cold_wallet_bytes = result.data.unwrap();
            assert_eq!(true,result.is_complete);
            assert_eq!("1f8b0800000000000003e36012e20f2c72ce4f49550828ca2fc94fcecf915ac3c8c124c46168646ae064e6e4ac3497918bd935c44348253125c9c430d93c59d7d8c2c258d724cdc8503729d1385937c9d4c2c028d1222529d920554ac057dfc4445ddfcc405d1f84f40d1456cdea6f6d34d210b20ae5d232a8484b354a3632364a4c717133333335b674724b314d31744a3171324a4eb6347431061966946824c46f648002a4b80d4dcc4c2ccc4c4ccccdcd95584dcc80625a0c068a0a07020064c17efbc5000000",
                       hex::encode(keystone_cold_wallet_bytes.get_bytes()).to_lowercase())
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
}
