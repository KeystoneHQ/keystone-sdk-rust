use crate::keystone::supported_coins::SupportedChains;
use serde_json::json;
use ur_registry::pb::protobuf_parser::{serialize_protobuf, zip};
use ur_registry::pb::protoc::payload::Type::SignTx;
use ur_registry::pb::protoc::sign_transaction::Transaction;
use ur_registry::pb::protoc::{payload, Base, BchTx, DashTx, LtcTx, Payload, SignTransaction};

pub fn construct_tx(
    coin_type: SupportedChains,
    request_id: &str,
    sign_data: &str,
    xfp: &str,
    timestamp: i64,
) -> Result<Vec<u8>, String> {
    let transaction = match adapt_transaction(coin_type.clone(), sign_data) {
        Ok(tx) => tx,
        Err(err_msg) => return Err(err_msg),
    };

    let base = Base {
        version: 2,
        description: "QrCode Protocol".to_string(),
        content: None,
        device_type: "".to_string(),
        data: Some(Payload {
            r#type: SignTx as i32,
            xfp: xfp.to_string(),
            content: Some(payload::Content::SignTx(SignTransaction {
                coin_code: coin_type.clone().into(),
                sign_id: request_id.to_string(),
                hd_path: "".to_string(),
                timestamp,
                decimal: 8,
                transaction: Some(transaction),
            })),
        }),
    };
    let data = serialize_protobuf(base);
    zip(&data).map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
}

fn adapt_transaction(coin_type: SupportedChains, sign_data: &str) -> Result<Transaction, String> {
    match coin_type {
        SupportedChains::LTC => serde_json::from_str::<LtcTx>(sign_data)
            .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
            .map(|tx| Transaction::LtcTx(tx)),
        SupportedChains::BCH => serde_json::from_str::<BchTx>(sign_data)
            .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
            .map(|tx| Transaction::BchTx(tx)),
        SupportedChains::DASH => serde_json::from_str::<DashTx>(sign_data)
            .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
            .map(|tx| Transaction::DashTx(tx)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_ltc_tx() {
        let sign_data = r#"
            {
                "fee": 2250,
                "dust_threshold":5460,
                "memo":"",
                "inputs":[
                    {
                        "hash":"a59bcbaaae11ba5938434e2d4348243e5e392551156c4a3e88e7bdc0b2a8f663",
                        "index":1,
                        "owner_key_path":"m/49'/2'/0'/0/0",
                        "utxo":{
                            "public_key":"035684d200e10bc1a3e2bd7d59e58a07f2f19ef968725e18f1ed65e13396ab9466",
                            "value":18519750,
                            "script": ""
                        }
                    }
                ],
                "outputs":[
                    {
                        "address":"MUfnaSqZjggTrHA2raCJ9kxpP2hM6zezKw",
                        "value":10000,
                        "is_change":false,
                        "change_address_path":""
                    },
                    {
                        "address":"MK9aTexgpbRuMPqGpMERcjJ8hLJbAS31Nx",
                        "value":18507500,
                        "is_change":true,
                        "change_address_path": "M/49'/2'/0'/0/0"
                    }
                ]
            }
        "#;
        let request_id = "cc946be2-8e4c-42be-a321-56a53a8cf516";
        let expect_result: Vec<u8> = [
            31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 85, 78, 79, 75, 2, 81, 28, 68, 187, 44, 94, 52, 79,
            226, 73, 150, 192, 16, 150, 125, 255, 247, 61, 232, 144, 89, 86, 234, 138, 169, 93,
            186, 189, 247, 246, 183, 138, 253, 89, 93, 138, 196, 79, 225, 71, 232, 11, 68, 247, 62,
            64, 65, 208, 45, 58, 119, 45, 162, 123, 183, 246, 26, 12, 195, 204, 48, 48, 227, 228,
            203, 197, 147, 180, 149, 68, 80, 27, 164, 201, 117, 98, 147, 139, 234, 123, 62, 75,
            157, 54, 161, 109, 213, 222, 39, 238, 115, 190, 176, 209, 27, 183, 202, 91, 214, 42,
            38, 12, 16, 79, 2, 179, 30, 35, 6, 60, 77, 9, 246, 184, 208, 156, 106, 105, 99, 142,
            69, 237, 225, 231, 227, 241, 23, 109, 59, 102, 157, 119, 94, 54, 75, 111, 13, 247, 46,
            87, 216, 213, 92, 25, 107, 180, 214, 128, 177, 201, 12, 149, 140, 50, 32, 81, 198, 146,
            48, 10, 28, 168, 34, 156, 99, 204, 133, 101, 154, 130, 148, 16, 152, 200, 34, 67, 180,
            140, 133, 160, 165, 92, 245, 184, 176, 135, 40, 23, 146, 69, 4, 33, 192, 200, 88, 156,
            21, 137, 137, 130, 136, 43, 224, 82, 163, 32, 38, 49, 86, 16, 43, 33, 3, 194, 1, 203,
            24, 67, 36, 50, 65, 169, 18, 218, 100, 231, 69, 229, 233, 254, 203, 113, 139, 151, 62,
            83, 117, 159, 212, 125, 148, 193, 71, 141, 122, 193, 13, 79, 227, 43, 61, 90, 156, 205,
            38, 147, 113, 122, 212, 36, 169, 110, 117, 212, 249, 114, 62, 32, 211, 80, 172, 96,
            213, 189, 45, 173, 251, 141, 157, 172, 216, 85, 122, 12, 203, 201, 220, 12, 111, 194,
            193, 226, 112, 30, 30, 12, 237, 172, 35, 167, 189, 142, 105, 142, 40, 238, 47, 75, 223,
            175, 159, 78, 37, 231, 22, 195, 255, 51, 127, 122, 90, 13, 84, 108, 1, 0, 0,
        ]
        .to_vec();

        let result = construct_tx(
            SupportedChains::LTC,
            request_id,
            &sign_data,
            "F23F9FD2",
            1681871353647,
        )
        .unwrap();
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_construct_dash_tx() {
        let sign_data = r#"
            {
                "fee": 2250,
                "dust_threshold": 5460,
                "memo":"",
                "inputs":[
                    {
                        "hash": "a59bcbaaae11ba5938434e2d4348243e5e392551156c4a3e88e7bdc0b2a8f663",
                        "index": 1,
                        "value": 18519750,
                        "pubkey": "03cf51a0e4f926e50177d3a662eb5cc38728828cec249ef42582e77e5503675314",
                        "owner_key_path": "m/44'/5'/0'/0/0"
                    }
                ],
                "outputs":[
                    {
                        "address":"XphpGezU3DUKHk87v2DoL4r7GhZUvCvvbm",
                        "value":10000,
                        "is_change":false,
                        "change_address_path":""
                    },
                    {
                        "address":"XfmecwGwcPBR7pXTqrn26jTjNe8a4fvcSL",
                        "value":18507500,
                        "is_change":true,
                        "change_address_path":"M/44'/5'/0'/0/0"
                    }
                ]
            }
        "#;
        let request_id = "cc946be2-8e4c-42be-a321-56a53a8cf516";
        let expect_result: Vec<u8> = [
            31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 85, 141, 191, 74, 195, 80, 24, 197, 105, 113, 8, 93,
            90, 157, 138, 83, 9, 66, 37, 16, 146, 124, 247, 111, 192, 65, 219, 208, 22, 172, 165,
            218, 22, 74, 183, 123, 111, 190, 80, 170, 53, 53, 72, 10, 62, 132, 248, 4, 190, 129,
            184, 251, 0, 29, 4, 55, 221, 93, 21, 113, 119, 51, 142, 194, 225, 192, 57, 252, 224,
            103, 149, 119, 170, 167, 89, 59, 141, 177, 49, 204, 210, 235, 212, 164, 23, 187, 111,
            229, 226, 181, 58, 64, 58, 97, 39, 2, 123, 83, 174, 108, 69, 71, 163, 222, 206, 158,
            49, 33, 229, 26, 193, 149, 72, 141, 75, 65, 163, 171, 8, 4, 46, 227, 138, 17, 37, 77,
            194, 2, 222, 120, 252, 126, 127, 250, 241, 247, 173, 217, 109, 217, 122, 222, 174, 189,
            58, 246, 125, 169, 114, 168, 88, 168, 141, 86, 74, 97, 16, 232, 98, 16, 73, 9, 69, 136,
            139, 150, 64, 9, 50, 36, 33, 48, 22, 4, 140, 27, 170, 8, 74, 137, 66, 199, 198, 215,
            160, 100, 194, 57, 169, 149, 234, 155, 135, 79, 203, 110, 249, 228, 79, 163, 124, 164,
            73, 8, 28, 153, 31, 8, 17, 19, 197, 57, 160, 102, 198, 16, 41, 64, 74, 144, 6, 13, 208,
            16, 19, 10, 76, 2, 10, 129, 140, 249, 132, 11, 70, 2, 234, 84, 151, 30, 165, 77, 143,
            53, 61, 191, 136, 231, 59, 205, 138, 61, 93, 205, 87, 93, 188, 153, 144, 104, 114, 220,
            59, 151, 34, 135, 40, 237, 211, 76, 116, 231, 179, 73, 222, 206, 115, 189, 172, 221,
            13, 156, 131, 2, 76, 150, 104, 214, 221, 181, 25, 182, 206, 196, 106, 58, 190, 202, 46,
            129, 47, 198, 139, 1, 74, 69, 147, 220, 140, 250, 181, 175, 151, 15, 171, 94, 178, 171,
            39, 255, 53, 191, 20, 60, 180, 223, 107, 1, 0, 0,
        ]
        .to_vec();

        let result = construct_tx(
            SupportedChains::DASH,
            request_id,
            &sign_data,
            "F23F9FD2",
            1681871353647,
        )
        .unwrap();
        assert_eq!(expect_result, result);
    }

    #[test]
    fn test_construct_bch_tx() {
        let sign_data = r#"
            {
                "fee": 2250,
                "dust_threshold": 5460,
                "memo":"",
                "inputs":[
                    {
                        "hash": "a59bcbaaae11ba5938434e2d4348243e5e392551156c4a3e88e7bdc0b2a8f663",
                        "index": 1,
                        "value": 18519750,
                        "pubkey": "025ad49879cc8f1f91a210c6a2762fe4904ef0d4f17fd124b11b86135e4cb9143d",
                        "owner_key_path": "m/44'/145'/0'/0/0"
                    }
                ],
                "outputs":[
                    {
                        "address":"qzrxqxsx0lfzyk4ht60a5hwwtr2xjvtxmu0qhkusnx",
                        "value":10000,
                        "is_change":false,
                        "change_address_path":""
                    },
                    {
                        "address":"qpgw8p85ysnjutpsk6u490ytydmgdlmc6vzxu680su",
                        "value":18507500,
                        "is_change":true,
                        "change_address_path":"M/44'/145'/0'/0/0"
                    }
                ]
            }
        "#;
        let request_id = "cc946be2-8e4c-42be-a321-56a53a8cf516";
        let expect_result: Vec<u8> = [
            31, 139, 8, 0, 0, 0, 0, 0, 0, 3, 93, 142, 187, 74, 3, 65, 24, 133, 73, 108, 150, 52,
            38, 86, 193, 42, 4, 33, 178, 16, 118, 46, 255, 76, 102, 58, 73, 36, 216, 40, 234, 27,
            204, 53, 33, 247, 236, 37, 217, 205, 83, 248, 8, 98, 47, 246, 62, 128, 133, 96, 103,
            99, 103, 169, 34, 246, 118, 110, 45, 28, 14, 156, 3, 135, 239, 4, 213, 131, 253, 171,
            120, 176, 180, 174, 117, 25, 47, 211, 165, 89, 206, 14, 63, 170, 101, 27, 12, 9, 29,
            202, 225, 41, 105, 191, 85, 107, 123, 253, 193, 217, 193, 145, 49, 18, 184, 118, 164,
            43, 28, 152, 46, 16, 237, 186, 138, 18, 220, 101, 92, 49, 170, 132, 241, 12, 243, 214,
            195, 207, 251, 227, 47, 58, 14, 174, 239, 170, 193, 115, 163, 254, 26, 182, 111, 43,
            181, 19, 197, 164, 54, 90, 41, 229, 48, 214, 101, 160, 2, 40, 56, 98, 75, 23, 4, 168,
            99, 142, 74, 194, 24, 198, 140, 27, 80, 212, 9, 225, 122, 218, 26, 164, 137, 18, 158,
            115, 90, 175, 52, 159, 238, 191, 130, 118, 31, 17, 166, 44, 72, 209, 147, 198, 8, 143,
            189, 196, 138, 96, 100, 184, 34, 61, 78, 188, 3, 137, 192, 121, 100, 193, 227, 158,
            183, 152, 128, 46, 121, 130, 99, 202, 202, 203, 90, 98, 160, 54, 108, 204, 35, 128, 78,
            132, 129, 117, 34, 84, 42, 66, 97, 84, 11, 215, 187, 56, 95, 231, 73, 142, 102, 126,
            87, 76, 97, 156, 114, 164, 216, 120, 187, 77, 99, 146, 79, 54, 105, 62, 207, 208, 122,
            60, 205, 146, 69, 94, 191, 185, 8, 135, 229, 96, 53, 218, 138, 149, 96, 69, 178, 152,
            100, 233, 42, 153, 242, 172, 164, 23, 105, 97, 231, 35, 59, 155, 27, 190, 217, 229, 25,
            23, 40, 201, 234, 223, 47, 159, 65, 179, 210, 110, 156, 255, 7, 255, 1, 78, 194, 53,
            238, 126, 1, 0, 0,
        ]
        .to_vec();

        let result = construct_tx(
            SupportedChains::BCH,
            request_id,
            &sign_data,
            "F23F9FD2",
            1681871353647,
        )
        .unwrap();
        assert_eq!(expect_result, result);
    }
}
