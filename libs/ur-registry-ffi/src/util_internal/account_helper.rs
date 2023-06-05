use crate::sync::crypto_hd_key::{AccountExtra, OkxExtra};

fn okx_chain_id_map(coin_type: u32) -> u32 {
    match coin_type {
        60 => 1,
        _ => coin_type,
    }
}

pub fn gen_extra_data(coin_type: u32) -> AccountExtra {
    let okx_chain_id = okx_chain_id_map(coin_type);
    return AccountExtra {
        okx: OkxExtra { chain_id: okx_chain_id },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keep_coin_type_as_default() {
        let coin_type = 0;
        let expect_chain_id = 0;

        assert_eq!(expect_chain_id, gen_extra_data(coin_type).okx.chain_id);
    }


    #[test]
    fn test_map_eth_chain_id_to_1() {
        let coin_type = 60;
        let expect_chain_id = 1;

        assert_eq!(expect_chain_id, gen_extra_data(coin_type).okx.chain_id);
    }
}
