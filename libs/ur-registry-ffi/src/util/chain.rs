pub fn map_coin_type(coin_type: u32) -> String {
    match coin_type {
        0 => "BTC",
        2 => "LTC",
        3 => "DOGE",
        5 => "DASH",
        60 => "ETH",
        118 => "ATOM",
        144 => "XRP",
        145 => "BCH",
        195 => "TRX",
        354 => "DOT",
        394 => "CRO",
        397 => "NEAR",
        434 => "KSM",
        459 => "KAVA",
        472 => "AR",
        501 => "SOL",
        637 => "APTOS",
        966 => "MATIC",
        996 => "OKT",
        1007 => "FTM",
        8217 => "KLAY",
        9000 => "AVAX",
        _ => ""
    }.to_string()
}
