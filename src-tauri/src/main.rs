// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use iota_stronghold::Stronghold;

mod authentication;
fn main() {
    //tauri::Builder::default()
      //  .setup(authentication::setup)
        //.invoke_handler(tauri::generate_handler![
          //  store_secret,
            //get_secret
       // ])
        //.run(tauri::generate_context!())
        //.expect("Error running app! Quaggin sad D:");
    dotnettauritest_lib::run()
}
