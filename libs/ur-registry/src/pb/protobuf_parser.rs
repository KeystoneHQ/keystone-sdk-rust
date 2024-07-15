use crate::error::URError;
use alloc::string::ToString;
use alloc::vec::Vec;
use core2::io::{Read, Write};
use libflate::gzip::{Decoder, Encoder};
use prost::bytes::Bytes;
use prost::Message;

pub fn parse_protobuf<T>(bytes: Vec<u8>) -> Result<T, URError>
where
    T: Message + Default,
{
    Message::decode(Bytes::from(bytes)).map_err(|e| URError::ProtobufDecodeError(e.to_string()))
}

pub fn serialize_protobuf<T>(data: T) -> Vec<u8>
where
    T: Message + Default,
{
    data.encode_to_vec()
}

pub fn unzip(bytes: Vec<u8>) -> Result<Vec<u8>, URError> {
    let mut decoder =
        Decoder::new(&bytes[..]).map_err(|e| URError::GzipDecodeError(e.to_string()))?;
    let mut buf = Vec::new();
    Read::read(&mut decoder, &mut buf).map_err(|e| URError::GzipDecodeError(e.to_string()))?;
    Read::read_to_end(&mut decoder, &mut buf)
        .map_err(|e| URError::GzipDecodeError(e.to_string()))?;
    Ok(buf)
}

pub fn zip(bytes: &Vec<u8>) -> Result<Vec<u8>, URError> {
    let mut encoder = Encoder::new(Vec::new()).unwrap();
    encoder
        .write_all(bytes)
        .map_err(|e| URError::GzipEncodeError(e.to_string()))?;
    let compressed = encoder
        .finish()
        .into_result()
        .map_err(|e| URError::GzipEncodeError(e.to_string()))?;
    Ok(compressed)
}

#[cfg(test)]
mod tests {
    use crate::pb::protobuf_parser::{parse_protobuf, serialize_protobuf, unzip, zip};
    use crate::pb::protoc::Base;
    use alloc::string::{String, ToString};
    use alloc::vec::Vec;
    use hex::FromHex;

    extern crate std;

    use prost::Message;
    use std::println;

    use crate::crypto_key_path::CryptoKeyPath;
    use crate::ethereum::eth_sign_request::DataType;

    #[test]
    fn test_protobuf() {
        let hex = "0802120f5172436f64652050726f746f636f6c1aa901080212083730374545443643229a010a03455448122435646534616138612d643366372d343765332d623966652d3934383231393434383238621a104d2f3434272f3630272f30272f302f3020d4f0ae8f823028123a520a2a307844364362643230333841363635333939333030396335363931324362343531313761423933304438120e31303030303030303030303030301a0b333639353832373934303622053436303030303e20f54e";
        let bytes = Vec::from_hex(hex).unwrap();
        let base: Base = parse_protobuf(bytes).unwrap();
        let encode_result = serialize_protobuf(base);
        assert_eq!(hex, hex::encode(encode_result).to_lowercase());
    }

    #[test]
    fn test_parse_keystone_cold_wallet_protobuffer() {
        let hex = "1f8b08000000000000005dd439aedd741406705d41f1741304a48a52d141f7e2d9ffd2b363dfeb789e9ac8f3b51dcfb31b860a31888e821db03136c006f05302e4453a0bf8e93bdf3977a7679f97c9368c4d9d7cd5f55113272ffe7c72777a7607c1d84b1aa799177f3c39ff743a7f429bcc17a717df9dce4faef728f8fafee5c33c6bd6760a71ba8b7c4b1d46009449ac54c85422c64d479ab68a94242c2af3bd54f02e721da4e1cd68405f345a63df5cd65ba214a54762996166d8a99c76d3fe96add8d6484a437609c4db9aaa6c26370e8123b518fcfc74feed743e1f9837174ea018ef5f11fa58c4ec62eeb6be3cab5ecd2c758c771d06415b4dd238b2d164cf8cebe0a1c19e4cccaa7a528dd384ac70013eb04ca95005868a18e520bbe791dac5d805bc4b08c3a767d941f2ec16ab82e96919ebaea9a81da2df4fe72f1f440a65beb2b937062738afccf730f263585b2820b2e1c430c479a70e14d2ea1d8b7581a3df2cd720287ac0a84ec88309a143aa76e07485a485e3961a4c994d652b3e87ad4bc95ca18572e189a2ca14c12a72cb965e226b5ed6220c684ed59303f6f3b137ce148fbd7d7f3a3f7d9712fe48a34a1e6da7a095bde9aab726a1de28b5618ec86ea4bc796e1dd7bcd06aaa7525a69d466b34d3a3384c86111e918d2fb76e52bd4eef6d0f07b18f93a5e414c2d0fac9b2290ed9bb02ee795adb5a560e0ecd2f0f2d621e343f9cce9fbdd34028f62127215c5e2a5b3248827e01f52d6ef22948a626d2813f71bb71ad75cc91470b803474ab6b5637a17bad11acd42421e01158f259c500d6c849c8d0d4868f4b366b4c326fc1b3ce425d1870f0be09a88dbe0be7539632c4ff5b7d781e6962fac219a5b4f485756dae36ee2371a39a17df904059db2d0c5742554a8548a03709ed738bdc81c44829a0d4c5c81cbea54b0622a83e013e69947897c36ee02ab578a4416174384b9780be61ce52f287e6e1c42e1f9d18fc21863274605a33bbb02a3a217a2ee22b1e2ad7acedc7650772e0ee82892f40ac1b2af1d84c80f584242b3f51d73a322ac5e8a38b9b30828358642454b9d9aaf98d501b8f6daba6e6b72cde5696ef7cdf3f30bf1ed198fa6be5f1aac0a370308be2afde8637383345eaadab737ff5b991963d03ac1e8a200c60a77a1c5d7ab98241a9ea6162629fa7d84b66b9835114e6cdf2b48bec2c62adf64110291225eaf351010d597a8783966ac1e3796bdf37c7d5d58f9a837ef8812a375c148158d2881b30db1a6859c698452a113b25355ee691c54ca6d181766731468de8045fea8183cb6b4cf358bbedc50411623f4c035a1671317be984a8348ad383a57938e468700c7a159b89fce0fc7870d8d7e6c1f9f67cbebf6f9bb7651037e37b0ab9c86accc945e8a30d5419029128784884f82cbaad71bb9aabc4a149e503920f078044eada97f2aa5797b223da1c2044a76ec53abb25ead7706d31f3a465f81eea73a7b5f6ee6a5b7025ab7c8906827b7efae6afbf4ff0d3ff1eb8da37ff000e508e76d1050000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_bytes = unzip(bytes).unwrap();
        let hex_unzip = hex::encode(unzip_bytes.clone()).to_lowercase();
        assert_eq!(
            "0801120f6b657973746f6e65207172636f64651aa90b0801120831323530423642431a9a0b0a8a010a0342544310011a80010a0b4d2f3439272f30272f3027126f78707562364271635a5550737439394e75486d5031544e63435866744242556a6638375541675a596647594c4b6e61666268536f39726a6f516f56685844597763663442743777763243737a41694258667268677835796f4a4e6f38716531465651504e795445747361574a70353218010a8f010a0a4254435f4c45474143591a80010a0b4d2f3434272f30272f3027126f7870756236437a486958705a4b7650596e43776e64367171353131796e3842363379423872437478735934617a6575437850594a6e3642374b4e4561367344436b4e416a353448354157337a595938514c537a4736716537535a42764b57336967686450475459516744587866485118010a96010a114254435f4e41544956455f5345475749541a80010a0b4d2f3834272f30272f3027126f787075623643706a4e3963563265535348767a4135313133705271443571615752685558533741427335417147696175334262416e57326678314a774545776e397567564167783676627058414b456a51624b6a59484850436a617848457779664c63557677786a6261424550526518010a8b010a0345544810011a81010a0c4d2f3434272f3630272f3027126f787075623643504a5942566639704b59754d52705437506841506f436e437768384b7959586e646e4647705150554d37757a42346e34675263646265737432743379466b797175505971527256593639645a36386b4a576a4773705a6577794e57387258473659595170705555693918010a8c010a0342434810011a82010a0d4d2f3434272f313435272f3027126f787075623643653758464a6b70386165617277396e68646f69756165756f6352395a75457a534d6e5235574b745539396662586d4d676e6f62584d6e33356b514a47614633324a5a444e53395574454a33736f6e535a364a564453754b4655327652443171626145327a794734563418010a8b010a044441534810011a80010a0b4d2f3434272f35272f3027126f78707562364364424c45536b4a77726a554d6f4d56365a33646f50544c5a534a396b6e567032326d476d6b4a6a483734684a34726955387a394a434a66394150775367574670426b433137417265395a38536b367169325861584e6e4855693941354262764a4c6142683557776b4618010a8a010a034c544310011a80010a0b4d2f3439272f32272f3027126f7870756236434153523954557644774450347533526948367836624e4d67707274777a394b61587a4754367739486e6f41655944674732526538386d5a6550786e63536d4e5372634c586543475733553863476d69547050696837506f5944706d6f6e4679676479784446715a5a5a18010a8d010a0454524f4e10011a82010a0d4d2f3434272f313935272f3027126f787075623643355541464d5979366f364375635068716e695a785a4574424b5953397859343333433944756e74745842774d39734e6d6e737543645a4641444c67555873536a6a54685559514c4b5777486e50726161634e4a4148527643653751337772574531776d77366476797018010a8c010a0358525010011a82010a0d4d2f3434272f313434272f3027126f7870756236426d5862774e4737776663457335565573424b4b3543774a6b3356663851464b4633775465757457317a57775374513771475a4a723957364b786442463570797a6a7531374872737573346b6a646a76596675335042343642735551593631575132643972503576376918010a88010a03444f5410011a7f0a0a2f2f706f6c6b61646f74126f787075623638774b5064454b6a625a346f316d534737654e36623762367648587053684d54784a4534656d5a39384662733933635078726b4b78526d4c6b713770693933377150796a7876586b345a6e326e554376755167367a625276715170567a585179614d386d697763733745180128e6f301320c6b657973746f6e652050726f",
            hex_unzip
        );
        // USE BASE MESSAGE TYPE , DONT USE SYNC TYPE
        let parse_result: crate::pb::protoc::Base = parse_protobuf(unzip_bytes).unwrap();
        let playload = parse_result.data.unwrap();
        let content = playload.content.unwrap();
        assert_eq!("1250B6BC".to_string(), playload.xfp);
        // Base { version: 1, description: "keystone qrcode", data: Some(Payload { r#type: Sync, xfp: "1250B6BC", content: Some(Sync(Sync { coins: [Coin { coin_code: "BTC", active: true, accounts: [Account { hd_path: "M/49'/0'/0'", x_pub: "xpub6BqcZUPst99NuHmP1TNcCXftBBUjf87UAgZYfGYLKnafbhSo9rjoQoVhXDYwcf4Bt7wv2CszAiBXfrhgx5yoJNo8qe1FVQPNyTEtsaWJp52", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "BTC_LEGACY", active: false, accounts: [Account { hd_path: "M/44'/0'/0'", x_pub: "xpub6CzHiXpZKvPYnCwnd6qq511yn8B63yB8rCtxsY4azeuCxPYJn6B7KNEa6sDCkNAj54H5AW3zYY8QLSzG6qe7SZBvKW3ighdPGTYQgDXxfHQ", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "BTC_NATIVE_SEGWIT", active: false, accounts: [Account { hd_path: "M/84'/0'/0'", x_pub: "xpub6CpjN9cV2eSSHvzA5113pRqD5qaWRhUXS7ABs5AqGiau3BbAnW2fx1JwEEwn9ugVAgx6vbpXAKEjQbKjYHHPCjaxHEwyfLcUvwxjbaBEPRe", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "ETH", active: true, accounts: [Account { hd_path: "M/44'/60'/0'", x_pub: "xpub6CPJYBVf9pKYuMRpT7PhAPoCnCwh8KyYXndnFGpQPUM7uzB4n4gRcdbest2t3yFkyquPYqRrVY69dZ68kJWjGspZewyNW8rXG6YYQppUUi9", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "BCH", active: true, accounts: [Account { hd_path: "M/44'/145'/0'", x_pub: "xpub6Ce7XFJkp8aearw9nhdoiuaeuocR9ZuEzSMnR5WKtU99fbXmMgnobXMn35kQJGaF32JZDNS9UtEJ3sonSZ6JVDSuKFU2vRD1qbaE2zyG4V4", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "DASH", active: true, accounts: [Account { hd_path: "M/44'/5'/0'", x_pub: "xpub6CdBLESkJwrjUMoMV6Z3doPTLZSJ9knVp22mGmkJjH74hJ4riU8z9JCJf9APwSgWFpBkC17Are9Z8Sk6qi2XaXNnHUi9A5BbvJLaBh5WwkF", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "LTC", active: true, accounts: [Account { hd_path: "M/49'/2'/0'", x_pub: "xpub6CASR9TUvDwDP4u3RiH6x6bNMgprtwz9KaXzGT6w9HnoAeYDgG2Re88mZePxncSmNSrcLXeCGW3U8cGmiTpPih7PoYDpmonFygdyxDFqZZZ", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "TRON", active: true, accounts: [Account { hd_path: "M/44'/195'/0'", x_pub: "xpub6C5UAFMYy6o6CucPhqniZxZEtBKYS9xY433C9DunttXBwM9sNmnsuCdZFADLgUXsSjjThUYQLKWwHnPraacNJAHRvCe7Q3wrWE1wmw6dvyp", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "XRP", active: true, accounts: [Account { hd_path: "M/44'/144'/0'", x_pub: "xpub6BmXbwNG7wfcEs5VUsBKK5CwJk3Vf8QFKF3wTeutW1zWwStQ7qGZJr9W6KxdBF5pyzju17Hrsus4kjdjvYfu3PB46BsUQY61WQ2d9rP5v7i", address_length: 1, is_multi_sign: false }] }, Coin { coin_code: "DOT", active: true, accounts: [Account { hd_path: "//polkadot", x_pub: "xpub68wKPdEKjbZ4o1mSG7eN6b7b6vHXpShMTxJE4emZ98Fbs93cPxrkKxRmLkq7pi937qPyjxvXk4Zn2nUCvuQg6zbRvqQpVzXQyaM8miwcs7E", address_length: 1, is_multi_sign: false }] }] })) }), device_type: "keystone Pro", content: Some(ColdVersion(31206)) }
    }
    #[test]
    fn test_parse_eth_sign_protobuffer_data_wallet3() {
        let hex = "1f8b0800000000000003558e314ac4401486cd2e6248e33265aa10849540c89bf766de4c96adb2eba282b282bd4c66269510582cacbd82780dcfe0312cbc8195373096fe7cc5d77d7f3a13a77787cd1862b13f8c4fa31f1ff3ef249d8954a2868ebb4df99564f38bfb4b71a6bdb15e81acb51f865a3199bab5926ac7d2f8185b05d0e78b9b46a965c3b06cfe68a0787bfffc79c173b17a4db20a9e87881e095dd8ee9835b5dd2ee820bba03af4be955bd216101d8a8584ffcb3363812d13b7b23cd6d6485b1d41b55aa742ccaf6eaff393e9e6c324e59489687bea097bf6a4b423350cacd8280696a842d0ce47ef088a8ffd2f48148a9904010000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_bytes = unzip(bytes).unwrap();
        let hex_unzip = hex::encode(unzip_bytes.clone()).to_lowercase();
        let parse_result: crate::pb::protoc::Base = parse_protobuf(unzip_bytes).unwrap();
        let payload = parse_result.data.unwrap();
        let content = payload.content.unwrap();
        assert_eq!("1250B6BC", payload.xfp);
    }

    #[test]
    fn test_parse_eth_sign_protobuffer_data_from_cold_wallet3() {
        let hex = "1f8b08000000000000003dd03daad4510c0570e655a308c294afb2b0b019c8c74d6e62396ec02de4de24cd03079f16ba427760e102dc801bf00f82ed297ee770cea7cbeba7fafee5ebfd53bdf9fcbcef598f3f1fce2f2e6724819bde3ebcfff1f0f2ede8ad1c48d79082eba0aaab97fa95362508ab96d0e506df9a66f2b6610e233256b50c84b0996db50d67c2008911730179b1e940459cdab612211f7f9d0ec434484d006d0b90ea365abcc047d751c81499ad2aecab5312578e457b3b268b015190a94433c28c010047a601d2de7b05b74f39a66d63e7d9089a30553c0e7f697475ad204636249845ce348d8663004ef4f1cfe98c604e921a216486c374614c8a25037ac7a60a37dfbcb07dc48a9cfceef79f13bdfafff6c7e7fb5f371772a97e010000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_bytes = unzip(bytes).unwrap();

        assert_eq!(
            "0801120f6b657973746f6e65207172636f64651ad6020809120831323530423642433ac7020a2434666336336131322d613565302d343265652d396536392d32633264303533363665353212423078663237643363383438393034616461626566353431306138376466386563383137643034303561346137623032396533383634313631313736663862643130641ada01307866383661323638353031386335303236366338326233623039346665326332333261646466363635333962666435643162643462326363393164333538303232613238363561663331303761343030303830323661303566396663626133663937356164616338333933376631303664303736353961623039623661666566656261323331333831323037653239333237383234393161303137313934303566396663666461613333643235653461353238383134383662316137326162353430666361633265613938396333623166393461626164373328e6f301320c6b657973746f6e652050726f".to_string(),
            hex::encode(&unzip_bytes)
        );
        let parse_result: crate::pb::protoc::Base = parse_protobuf(unzip_bytes).unwrap();
        println!("{:?}", parse_result);
        let payload = parse_result.data.unwrap();
        let content = payload.content.unwrap();
        assert_eq!("1250B6BC", payload.xfp);
    }

    #[test]
    fn test_decode_hex() {
        let proto_buffer_data_version2 = "1f8b08000000000000003d90314e43410c4415aa80909052a6a2a0a0f992d75e7bbd94e1025cc15e7b1b24220245382137a0e0005c800bf02bbad17bd24833dbcdeee6393fdede8f2f79fb7a1ac7c8fdd7c5f672b72dc87090c3e3c3e7c5d55d4e2f23a32edcc7586a515a2ca52f435057d522a0ec0e7086c1139371ce663007b2995874c81613045dc8a546234f290acd552699a96aa23060df7f6fe03c550c45198a8e158a0c4527875e67e240428b98224cdd6770148fea38462f41ac8068a8c236a940b30a002b630384c41803306b69605db2924fb11e8d7b387569a94ca563801097da9cb9ac4283d5fa34584b335aace3a8312a449f7516e90ea652416a65a86ab4a624600f73e156a8672b4ed2ee7f7e3778fdfff6d3e9f8078971e1577e010000";
        let bytes = Vec::from_hex(proto_buffer_data_version2).unwrap();
        println!("{:?}", bytes);
    }

    #[test]
    fn test_parse_eth_tx_protobuffer_data() {
        let hex = "1f8b0800000000000003e36012e20f2c72ce4f49550828ca2fc94fcecf915ac3c8c124c46168646ae064e6e4ac3497918bd935c44348253125c9c430d93c59d7d8c2c258d724cdc8503729d1385937c9d4c2c028d1222529d920554ac057dfc4445ddfcc405d1f84f40d1456cdea6f6d34d210b20ae5d232a8484b354a3632364a4c717133333335b674724b314d31744a3171324a4eb6347431061966946824c46f648002a4b80d4dcc4c2ccc4c4ccccdcd95584dcc80625a0c068a0a07020064c17efbc5000000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_bytes = unzip(bytes).unwrap();
        let hex_unzip = hex::encode(unzip_bytes.clone()).to_lowercase();
        assert_eq!(
            "0802120f5172436f64652050726f746f636f6c1aac01080212083132353042364243229d010a03455448122461646234316337632d333838332d346632312d626133632d6235383032613864626330651a104d2f3434272f3630272f30272f302f3020aa9a8f85813228123a550a2a307866653263323332616444463636353339424664356431426434423263633931443335383032326132120f3230303030303030303030303030301a0b3134363438363434373737220534363030302a00302120c050",
            hex_unzip
        );
        let parse_result: crate::pb::protoc::Base = parse_protobuf(unzip_bytes).unwrap();
        assert_eq!(2, parse_result.version);
        assert_eq!("QrCode Protocol", parse_result.description);
        let pay_load_data = parse_result.data.unwrap();
        assert_eq!("1250B6BC", pay_load_data.xfp);
        #[derive(Clone, Debug, Default)]
        pub struct EthSignRequest {
            request_id: Option<crate::types::Bytes>,
            sign_data: crate::types::Bytes,
            data_type: DataType,
            chain_id: Option<i128>,
            derivation_path: CryptoKeyPath,
            address: Option<crate::types::Bytes>,
            origin: Option<String>,
        }
        // Base { version: 2, description: "QrCode Protocol", data: Some(Payload { r#type: SignTx, xfp: "1250B6BC", content: Some(SignTx(SignTransaction { coin_code: "ETH", sign_id: "adb41c7c-3883-4f21-ba3c-b5802a8dbc0e", hd_path: "M/44'/60'/0'/0/0", timestamp: 1718266088746, decimal: 18, transaction: Some(EthTx(EthTx { to: "0xfe2c232adDF66539BFd5d1Bd4B2cc91D358022a2", value: "200000000000000", gas_price: "14648644777", gas_limit: "46000", memo: "", nonce: 33, r#override: None })) })) }), device_type: "", content: Some(HotVersion(10304)) }
    }

    #[test]
    fn test_gzip() {
        let hex = "1f8b08000000000000034dcc3b0ac2401405503f08a2a092325510410984bcc9bcf93c0b4193808da2900de45b09816061ed06dc803b105c8d0bb1b177ecbc5c6e718adbef58e36313d645e91c9afa5ce7f5c97eb48df615a8388e6438bdb707dd38d95a33519498a63af50a5e290f55c9bd8caad223d4012334ab337bb2f311e7be84b9ffab0fceebfdbc5d61612d93810b17f398150170bd965270220e40b990c4823043c1984a37c6226d8d18fcc71e724942078a10e4b487d298db8295f3d97f01e1098c51c4000000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_data = unzip(bytes).unwrap();
        let zip_data = zip(&unzip_data).unwrap();
        assert_eq!(hex, hex::encode(zip_data).to_lowercase());
    }
}
