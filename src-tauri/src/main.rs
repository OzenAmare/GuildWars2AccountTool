// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri::Builder;
fn main() {

    // let quaggin_security_setup = r#"
    // "security_configuration":{
    //   "keyring_entry_name": "QUAGGIN_STRONGHOLD_PASSWORD",
    //   "keyring_username": "quaggin",
    //   "Oauth2Configuration": {
    //     "oauth2_link": "https://gw2.me/oaut2/authorize?",
    //     "redirect_routing_endpoint": "/callback",
    //     "redirect_uri": "127.0.0.1:3000",
    //     "client_id": "b185490f-b41b-40bc-9b1d-a5d7eb22ac68",
    //     "response_type": "code",
    //     "scope": "identify",
    //    "prompt": "consent",
    //    "include_granted_scopes": "true"
    //   }
    //  }"#;

    dotnettauritest_lib::run()
} 
