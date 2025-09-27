use reqwest::Client;
use bc_utils_lg::funcs::settings::settings_from_json;

use bc_exch_api_funcs::bybit::account::wallet_balance::*;


#[tokio::test]
async fn wallet_balance_req_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
    println!("{:#?}", wallet_balance_req(
        &Client::new(),
        &sttngs.exch.key,
        &sttngs.exch.secret,
        &sttngs.exch.url,
        "UNIFIED", 
        "USDT",
    ).await.unwrap());
}

#[tokio::test]
async fn wallet_balance_a_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
    println!("{:#?}", wallet_balance_a(
        &Client::new(),
        &sttngs.exch.key,
        &sttngs.exch.secret,
        &sttngs.exch.url,
        "UNIFIED", 
        "USDT",
        &f64::INFINITY,
    ).await.unwrap());
}
