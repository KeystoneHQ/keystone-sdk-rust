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
    use alloc::vec::Vec;
    use hex::FromHex;

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
        let hex = "1f8b08000000000000034dcc3b0ac2401405503f08a2a092325510410984bcc9bcf93c0b4193808da2900de45b09816061ed06dc803b105c8d0bb1b177ecbc5c6e718adbef58e36313d645e91c9afa5ce7f5c97eb48df615a8388e6438bdb707dd38d95a33519498a63af50a5e290f55c9bd8caad223d4012334ab337bb2f311e7be84b9ffab0fceebfdbc5d61612d93810b17f398150170bd965270220e40b990c4823043c1984a37c6226d8d18fcc71e724942078a10e4b487d298db8295f3d97f01e1098c51c4000000";
        let bytes = Vec::from_hex(hex).unwrap();
        let unzip_data = unzip(bytes.clone()).unwrap();
        let zip_data = zip(&unzip_data).unwrap();
        assert_eq!(hex, hex::encode(zip_data).to_lowercase());
    }
}
