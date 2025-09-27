use reqwest::Client;
use bc_utils_lg::funcs::settings::settings_from_json;

use bc_exch_api_funcs::bybit::account::acc_info::*;


#[tokio::test]
async fn acc_info_req_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
    println!("{:#?}", acc_info_req(
        &Client::new(), 
        &sttngs.exch.api_key,
        &sttngs.exch.api_secret,
        &sttngs.exch.api_url,
    ).await.unwrap());
}

#[tokio::test]
async fn acc_info_a_lch_1() {
    let sttngs = settings_from_json("settings.json").unwrap();
println!("{:#?}", acc_info_a(
        &Client::new(), 
        &sttngs.exch.api_key,
        &sttngs.exch.api_secret,
        &sttngs.exch.api_url,
        &f64::INFINITY,
    ).await.unwrap());
}
