use serde_json::json;

fn okx_chain_id_map(coin_type: u32) -> u32 {
    match coin_type {
        60 => 1,
        _ => coin_type,
    }
}

pub fn gen_extra_data(coin_type: u32) -> String {
    let okx_chain_id = okx_chain_id_map(coin_type);

    return json!({
        "okx": {
            "chainId": okx_chain_id
        }
    }).to_string()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_coin_type_as_default() {
        let coin_type = 0;
        let expect_extra_data = r#"{"okx":{"chainId":0}}"#;

        assert_eq!(
            expect_extra_data,
            gen_extra_data(coin_type)
        );
    }


    #[test]
    fn test_map_eth_chain_id_to_1() {
        let coin_type = 60;
        let expect_extra_data = r#"{"okx":{"chainId":1}}"#;

        assert_eq!(
            expect_extra_data,
            gen_extra_data(coin_type)
        );
    }
}
