use serde_json::{json};
use ur_registry::pb::protobuf_parser::{serialize_protobuf, zip};
use ur_registry::pb::protoc::{Base, Payload, payload, SignTransaction, BchTx, DashTx, LtcTx};
use ur_registry::pb::protoc::payload::Type::SignTx;
use ur_registry::pb::protoc::sign_transaction::Transaction;
use crate::utxo::supported_coins::SupportedChains;

pub fn construct_tx(coin_type: SupportedChains, request_id: &str, sign_data: &str, xfp: &str, timestamp: i64) -> Result<Vec<u8>, String> {
    let transaction = match adapt_transaction(coin_type.clone(), sign_data) {
        Ok(tx) => tx,
        Err(err_msg) => return Err(err_msg),
    };

    let base = Base {
        version: 2,
        description: "QrCode Protocol".to_string(),
        content: None,
        device_type: "".to_string(),
        data: Some(
            Payload {
                r#type: SignTx as i32,
                xfp: xfp.to_string(),
                content: Some(payload::Content::SignTx(
                    SignTransaction {
                        coin_code: coin_type.clone().into(),
                        sign_id: request_id.to_string(),
                        hd_path: "".to_string(),
                        timestamp,
                        decimal: 8,
                        transaction: Some(transaction),
                    }
                )),
            }
        ),
    };
    let data = serialize_protobuf(base);
    zip(&data).map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
}

fn adapt_transaction(coin_type: SupportedChains, sign_data: &str) -> Result<Transaction, String> {
    match coin_type {
        SupportedChains::LTC => {
            serde_json::from_str::<LtcTx>(sign_data)
                .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
                .map(|tx| Transaction::LtcTx(tx))
        }
        SupportedChains::BCH => {
            serde_json::from_str::<BchTx>(sign_data)
                .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
                .map(|tx| Transaction::BchTx(tx))
        }
        SupportedChains::DASH => {
            serde_json::from_str::<DashTx>(sign_data)
                .map_err(|_| json!({"error": "transaction data is invalid"}).to_string())
                .map(|tx| Transaction::DashTx(tx))
        }
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
        let expect_result: Vec<u8> = [31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 85, 142, 205, 74, 35, 65, 20, 133, 73, 220, 52, 189, 73, 204, 74, 92, 73, 35, 68, 2, 77, 170, 110, 253, 116, 21, 184, 48, 70, 51, 51, 49, 45, 81, 227, 198, 93, 85, 245, 237, 132, 248, 211, 73, 163, 24, 124, 138, 60, 194, 188, 128, 184, 159, 7, 152, 1, 193, 157, 184, 118, 59, 34, 238, 221, 89, 91, 225, 112, 56, 247, 240, 193, 185, 65, 181, 81, 59, 42, 187, 69, 134, 27, 195, 178, 184, 46, 92, 113, 177, 254, 82, 245, 109, 208, 3, 214, 211, 189, 61, 136, 254, 85, 195, 149, 193, 168, 219, 216, 116, 78, 115, 105, 17, 98, 133, 220, 197, 28, 44, 198, 134, 1, 141, 133, 52, 130, 25, 229, 114, 65, 229, 198, 195, 199, 235, 159, 79, 178, 21, 216, 101, 53, 120, 92, 173, 63, 183, 162, 223, 149, 112, 199, 8, 109, 157, 53, 198, 32, 165, 214, 31, 76, 113, 198, 17, 50, 239, 10, 56, 67, 129, 76, 131, 16, 148, 10, 233, 184, 97, 168, 20, 38, 54, 115, 196, 130, 81, 185, 148, 172, 94, 89, 255, 21, 238, 18, 38, 164, 226, 25, 16, 130, 148, 88, 71, 61, 8, 54, 75, 50, 161, 81, 40, 67, 146, 28, 114, 170, 49, 215, 82, 37, 32, 144, 170, 156, 98, 38, 125, 96, 76, 75, 99, 253, 243, 114, 237, 239, 253, 91, 16, 213, 46, 219, 92, 55, 219, 208, 108, 19, 175, 54, 105, 53, 195, 40, 61, 205, 175, 204, 201, 252, 108, 58, 30, 143, 202, 159, 29, 40, 77, 183, 175, 207, 23, 179, 33, 76, 82, 121, 135, 119, 7, 183, 245, 229, 97, 107, 219, 131, 7, 218, 140, 112, 49, 158, 217, 227, 155, 116, 56, 255, 49, 75, 247, 143, 221, 180, 175, 38, 131, 190, 237, 156, 48, 122, 184, 168, 191, 63, 253, 15, 214, 42, 81, 45, 253, 62, 243, 5, 122, 90, 13, 84, 108, 1, 0, 0].to_vec();

        let result = construct_tx(SupportedChains::LTC, request_id, &sign_data, "F23F9FD2", 1681871353647).unwrap();
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
        let expect_result: Vec<u8> = [31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 85, 141, 191, 74, 43, 65, 28, 133, 73, 176, 88, 210, 100, 77, 37, 86, 97, 17, 34, 11, 75, 102, 126, 243, 119, 193, 66, 205, 146, 4, 174, 134, 92, 53, 32, 233, 102, 102, 127, 75, 136, 198, 141, 139, 108, 192, 135, 16, 159, 224, 190, 193, 197, 222, 7, 72, 33, 220, 78, 123, 91, 69, 236, 237, 238, 90, 10, 135, 3, 223, 225, 192, 231, 213, 91, 205, 223, 69, 47, 79, 177, 61, 46, 242, 155, 220, 229, 151, 219, 47, 245, 106, 245, 250, 192, 250, 113, 63, 129, 96, 93, 111, 108, 36, 7, 167, 195, 214, 142, 115, 49, 151, 22, 33, 210, 200, 93, 196, 193, 98, 100, 24, 208, 72, 72, 35, 152, 209, 46, 19, 84, 182, 31, 62, 95, 31, 191, 200, 174, 55, 189, 171, 123, 79, 155, 254, 115, 24, 252, 169, 53, 246, 141, 136, 173, 179, 198, 24, 164, 212, 86, 192, 52, 103, 28, 33, 173, 90, 3, 103, 40, 144, 197, 32, 4, 165, 66, 58, 110, 24, 106, 141, 202, 166, 142, 88, 48, 58, 147, 146, 249, 181, 173, 245, 223, 119, 47, 56, 36, 236, 91, 99, 8, 242, 44, 6, 137, 130, 80, 165, 82, 102, 164, 4, 180, 194, 57, 166, 21, 104, 13, 218, 161, 3, 30, 99, 198, 65, 104, 64, 165, 80, 8, 194, 164, 18, 140, 242, 176, 185, 232, 114, 222, 233, 138, 78, 151, 84, 233, 146, 176, 211, 8, 206, 151, 179, 229, 0, 111, 39, 44, 153, 252, 26, 94, 104, 85, 66, 146, 31, 241, 66, 13, 102, 211, 73, 217, 43, 75, 187, 240, 239, 71, 225, 94, 117, 204, 22, 232, 86, 131, 149, 27, 31, 158, 168, 229, 249, 217, 117, 113, 5, 114, 126, 54, 31, 161, 54, 60, 43, 221, 233, 145, 255, 241, 239, 205, 219, 170, 5, 205, 227, 159, 154, 255, 20, 60, 180, 223, 107, 1, 0, 0].to_vec();

        let result = construct_tx(SupportedChains::DASH, request_id, &sign_data, "F23F9FD2", 1681871353647).unwrap();
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
                        "address":"bitcoincash:qzrxqxsx0lfzyk4ht60a5hwwtr2xjvtxmu0qhkusnx",
                        "value":10000,
                        "is_change":false,
                        "change_address_path":""
                    },
                    {
                        "address":"bitcoincash:qpgw8p85ysnjutpsk6u490ytydmgdlmc6vzxu680su",
                        "value":18507500,
                        "is_change":true,
                        "change_address_path":"M/44'/145'/0'/0/0"
                    }
                ]
            }
        "#;
        let request_id = "cc946be2-8e4c-42be-a321-56a53a8cf516";
        let expect_result: Vec<u8> = [31, 139, 8, 0, 0, 0, 0, 0, 0, 255, 93, 142, 61, 106, 27, 81, 28, 196, 145, 212, 44, 106, 178, 114, 37, 92, 137, 37, 224, 32, 16, 122, 31, 255, 247, 246, 189, 164, 9, 118, 16, 110, 28, 108, 223, 224, 125, 106, 101, 105, 181, 210, 126, 88, 43, 85, 57, 66, 142, 144, 11, 24, 247, 58, 128, 11, 67, 186, 28, 32, 173, 141, 49, 184, 116, 231, 173, 5, 195, 192, 12, 12, 243, 11, 218, 71, 159, 174, 242, 179, 204, 186, 193, 101, 158, 149, 153, 201, 22, 199, 191, 58, 77, 27, 76, 8, 157, 200, 201, 15, 18, 189, 181, 187, 157, 211, 179, 243, 163, 207, 198, 72, 224, 218, 145, 145, 112, 96, 70, 64, 180, 27, 41, 74, 240, 136, 113, 197, 168, 18, 198, 51, 204, 7, 247, 175, 255, 247, 239, 232, 75, 112, 189, 111, 7, 143, 189, 240, 223, 48, 250, 211, 234, 126, 87, 76, 106, 163, 149, 82, 14, 99, 221, 4, 42, 128, 130, 35, 182, 113, 65, 128, 58, 230, 168, 36, 140, 97, 204, 184, 1, 69, 157, 16, 46, 214, 214, 32, 77, 148, 240, 156, 211, 176, 213, 127, 184, 123, 14, 162, 83, 68, 152, 178, 32, 69, 44, 141, 17, 30, 123, 137, 21, 193, 200, 112, 69, 98, 78, 188, 3, 137, 192, 121, 100, 193, 227, 216, 91, 76, 64, 55, 127, 130, 99, 202, 26, 100, 45, 49, 80, 59, 236, 165, 99, 128, 147, 49, 6, 118, 50, 70, 141, 198, 104, 248, 173, 203, 245, 172, 52, 217, 108, 105, 84, 145, 124, 93, 239, 242, 122, 93, 23, 53, 90, 248, 221, 118, 14, 73, 201, 145, 98, 201, 102, 83, 230, 164, 190, 185, 45, 235, 180, 66, 235, 100, 94, 21, 203, 58, 252, 253, 115, 120, 125, 48, 94, 77, 55, 98, 37, 216, 182, 88, 222, 84, 229, 170, 152, 243, 170, 161, 218, 150, 91, 155, 78, 237, 34, 53, 252, 118, 87, 87, 92, 160, 162, 10, 95, 254, 62, 5, 253, 86, 212, 187, 56, 4, 250, 0, 217, 21, 182, 93, 150, 1, 0, 0].to_vec();

        let result = construct_tx(SupportedChains::BCH, request_id, &sign_data, "F23F9FD2", 1681871353647).unwrap();
        assert_eq!(expect_result, result);
    }
}
