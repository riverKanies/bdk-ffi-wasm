//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use web_sys::console;
use js_sys::Date;


use bdk_ffi_wasm::{Descriptor, WalletWasm, EsploraClientWrapper, UpdateWrapper};
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

fn new_test_wallet() -> Result<WalletWasm, String> {

    let esplora_url = "https://mutinynet.com/api";

    let descriptor = Descriptor::new("tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/0/*)#z3x5097m".into(), "signet".into()).expect("descriptor");
    console::log_1(&format!("descriptor parsed: {}", descriptor.to_string_with_secret()).into());

    let change_descriptor = Descriptor::new("tr([12071a7c/86'/1'/0']tpubDCaLkqfh67Qr7ZuRrUNrCYQ54sMjHfsJ4yQSGb3aBr1yqt3yXpamRBUwnGSnyNnxQYu7rqeBiPfw3mjBcFNX4ky2vhjj9bDrGstkfUbLB9T/1/*)#n9r4jswr".into(), "signet".into()).expect("descriptor");
    console::log_1(&format!("change_descriptor parsed: {}", change_descriptor.to_string_with_secret()).into());

    WalletWasm::new(
        descriptor.to_string_with_secret(),
        change_descriptor.to_string_with_secret(),
        "signet".into()
    )
}

#[wasm_bindgen_test]
async fn test_wallet() {



    let wallet = new_test_wallet().expect("wallet");
    let scan_request = wallet.start_full_scan();

    let esplora_client = EsploraClientWrapper::new("https://mutinynet.com/api".into());

    let update = esplora_client.full_scan(scan_request, 5, 1).await.expect("full scan");

    let now = (Date::now() / 1000.0) as u64;
    wallet.apply_update_at(update, now).expect("apply update");
    
    // wallet.sync(5).await.expect("sync");

    let first_address = wallet.peek_address("external".into(), 0);
    console::log_1(&format!("first_address: {}", first_address).into());

    assert_eq!(
        first_address,
        "tb1pkar3gerekw8f9gef9vn9xz0qypytgacp9wa5saelpksdgct33qdqs257jl".to_string()
    );

    let balance = wallet.balance();
    console::log_1(&format!("balance: {}", balance).into());
    assert_ne!(balance, 0);

    // let new_address = wallet.get_new_address();
    // console::log_1(&format!("new_address: {}", new_address).into());
}
