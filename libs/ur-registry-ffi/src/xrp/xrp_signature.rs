use hex;
use serde_json::json;
use ur_registry::bytes::Bytes;
use ur_registry::traits::From;

use crate::export;

export! {
    @Java_com_keystone_sdk_KeystoneNativeSDK_parseXrpSignature
    fn parse_xrp_signature(ur_type: &str, cbor_hex: &str) -> String {
        if "bytes" != ur_type {
            return json!({"error": "type not match"}).to_string();
        }

        let binary_encoded_transaction = Bytes::from_cbor(hex::decode(cbor_hex).unwrap_or_default()).unwrap().get_bytes();
        // TODO: parse XRP signed transaction and return signature
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_parse_xrp_account() {
        let xrp_account_cbor = "58bc12000022800000002404c49431201b04c531dc61400000000098968068400000000000000c73210263e0f578081132fd9e12829c67b9e68185d7f7a8bb37b78f98e976c3d9d163e67446304402202559efe68c5f5d61eccb5ddee82e24960a3b31a59038b0e5d9fdee1b31a05f0302204600823f7e48b01e05b95c127b03fd58e0e0f04620c00f592e064b18c64a12f581149c9a4355b51b024b70d9e7074e555c4ec5cae5a78314b4551ad4ad0273ea551703d51688f52cbe96f8c5";
        let expect_result = "{\"signature\":\"304402202559EFE68C5F5D61ECCB5DDEE82E24960A3B31A59038B0E5D9FDEE1B31A05F0302204600823F7E48B01E05B95C127B03FD58E0E0F04620C00F592E064B18C64A12F5\"}";

        assert_eq!(
            expect_result,
            parse_xrp_signature("bytes", xrp_account_cbor)
        );
    }
}
