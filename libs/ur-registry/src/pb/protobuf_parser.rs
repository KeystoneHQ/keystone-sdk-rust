use alloc::string::ToString;
use alloc::vec::Vec;
use compression::prelude::{Action, DecodeExt, EncodeExt, GZipDecoder, GZipEncoder};
use prost::bytes::Bytes;
use prost::Message;
use crate::error::URError;

pub fn parse_protobuf<T>(bytes: Vec<u8>) -> Result<T, URError>
    where T: Message + Default {
    Message::decode(Bytes::from(bytes)).map_err(|e| URError::ProtobufDecodeError(e.to_string()))
}

pub fn serialize_protobuf<T>(data: T) -> Vec<u8> where T: Message + Default {
    data.encode_to_vec()
}


pub fn unzip(bytes: Vec<u8>) -> Result<Vec<u8>, URError> {
    let decompressed = bytes
        .iter()
        .cloned()
        .decode(&mut GZipDecoder::new())
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| URError::GzipDecodeError(e.to_string()))?;
    Ok(decompressed)
}

pub fn zip(bytes: &Vec<u8>) -> Result<Vec<u8>, URError> {
    let compressed = bytes
        .iter()
        .cloned()
        .encode(&mut GZipEncoder::new(), Action::Finish)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| URError::GzipEncodeError(e.to_string()))?;
    Ok(compressed)

}

#[cfg(test)]
mod tests {
    use alloc::vec::Vec;
    use hex::FromHex;
    use crate::pb::protobuf_parser::{parse_protobuf, serialize_protobuf, unzip, zip};
    use crate::pb::protoc::Base;


    #[test]
    fn test_protobuf() {
        let hex = "0802120f5172436f64652050726f746f636f6c1aa901080212083730374545443643229a010a03455448122435646534616138612d643366372d343765332d623966652d3934383231393434383238621a104d2f3434272f3630272f30272f302f3020d4f0ae8f823028123a520a2a307844364362643230333841363635333939333030396335363931324362343531313761423933304438120e31303030303030303030303030301a0b333639353832373934303622053436303030303e20f54e";
        let bytes = Vec::from_hex(hex).unwrap();
        let base: Base = parse_protobuf(bytes).unwrap();
        let encode_result = serialize_protobuf(base.clone());
        assert_eq!(hex, hex::encode(encode_result).to_lowercase());
    }

    #[test]
    fn test_gzip() {
        let hex = "1f8b08000000000000ffe36012e20f2c72ce4f49550828ca2fc94fcecf915acd0814e53037307775753173569ac3c8c5ec1ae221a4629a926a92986891a89b629c66ae6b629e6aac9b649996aa6b69626164686902242d92a4047cf54d4cd4f5cd0cd4f54148df40e1ca8775fd4d061a4256215c5a0615401393528c0c8c2d1ccdcc4c8d2d2d8d0d0c2c934dcd2c0d8d9c934c4c0d0dcd139d80622e16427c8606c8408adbd8ccd2d4c2c8dcd2c4c04c89d5c40c28a6c56060a7f0d50f00e1098c51c4000000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_data = unzip(bytes.clone()).unwrap();
        let zip_data = zip(&unzip_data).unwrap();
        assert_eq!(hex, hex::encode(zip_data).to_lowercase())
    }
}