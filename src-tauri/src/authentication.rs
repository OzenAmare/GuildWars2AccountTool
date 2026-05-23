use axum::{
    routing::get, 
    Router,
    serve::Serve
};
use open;
use std::sync::Mutex;
use std::thread;
use tauri::Manager;
use tiny_http::{Response, Server};
use url::Url;
use tokio;
use axum::extract::Query;
use axum::response::Html;
use serde::Deserialize;
use tauri_plugin_stronghold::stronghold::Stronghold;
use anyhow::Result;
use std::path::PathBuf;
//this is where this snippet came from
//https://v2.tauri.app/plugin/stronghold/
//
//
//
//

// async fn initialize_stronghold_vault() -> Result<String, String>{
//
//     //creates the local stronghold vault to be operated with
//     tauri::Builder::default()
//         .setup(|app| {
//             let salt_path = app
//                 .path()
//                 .app_local_data_dir()
//                 .expect("could not resolve app local data path")
//                 .join("quaggin.vault");
//
//             let vault_path = app
//                 .path()
//                 .app_local_data_dir()
//                 .expect("could not resolve the path this time")
//                 .join("quaggin.hold");
//
//
//
//             app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;
//
//             let stronghold = app.handle().stronghold();
//
//             stronghold.save(&vault_path).map_err(|e| anyhow::anyhow!(e.to_string))?;
//
//
//             Ok(())
//         });
//
//
//
//
//
//     Ok("meow".to_string())
// }

// async fn stornghold_save_and_commit() -> Result<String, String>{
//
//     let stronghold = Stronghold::default();
//
//     stronghold.commit(&vault_path).map_err(|e| anyhow::anyhow!(e))?;
//
//     stronghold.save().map_err(|e| anyhow::anyhow!(e))?;
// }


impl QuagginStronghold{
    pub fn new(app: &tauri::AppHandle) -> Result<Self, String>{

        let app_dir = app
            .path()
            .app_local_data_dir()
            .map_err(|e| e.to_string())?;

        let vault_path = app_dir.join("quaggin.hold");

        //need to derive key eventually from local user system or something
        let password = b"need-to-derive-key".to_vec();

        let stronghold = Stronghold::new(&vault_path, password)
            .map_err(|e| e.to_string())?;

        Ok(Self{
            stronghold,
            vault_path
        })

    }
}

impl QuagginStronghold {
    pub fn save(&self) -> Result<(), String>{
        self.stronghold
            .save()
            .map_err(|e| e.to_string())
    }
}
pub async fn request_account_data_access() -> Result<String,String>{

    //let stronghold_start = initialize_stronghold_vault().await;
 
   // let started_port = start_oauth_server().await;
    let request_confirmed = request_redirect().await;
    Ok("mewo".to_string())
}


async fn request_redirect() -> Result<String, String>{

    let (tx, rx) = oneshot::channel::<String>();

    let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

        let app = Router::new().route(
        "/callback", 
        get({
            let tx = tx.clone();

            move |Query(params): Query<OAuthCallback>|{
                let tx = tx.clone();

                async move {
                    println!("recieved the auth code {}", params.code);

                    if let Some(sender) = tx.lock().unwrap().take(){
                        let _ = sender.send(params.code.clone());
                    }
           
                 Html("You can close this window now :3")

               }
            }
        }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

     println!("axum is starting the server at localhost:3000...");

     
    tokio::spawn(async move {
       if let Err(err) = axum::serve(listener, app).await {
           eprintln!("Server error: {err}");
       }   
    });
    
    //lets paramterize all of these seperately 
    let auth_link = "https://gw2.me/oauth2/authorize?";
    let request_url = Url::parse_with_params(auth_link, 
        &[
        ("client_id", "b185490f-b41b-40bc-9b1d-a5d7eb22ac68"),
        ("response_type", "code"),
        ("redirect_uri", "http://127.0.0.1:3000/callback"),
        ("scope", "identify"),
        ("prompt", "consent"),
        ("include_granted_scopes", "true")
        ]);
    
    let string_url = request_url.unwrap().to_string();
    println!("{}", string_url);
    open::that(string_url);

       
    Ok("meow".to_string())
}


struct QuagginStronghold{
    stronghold: Stronghold,
    vault_path: PathBuf
}

#[derive(Debug, Deserialize)]
struct OAuthCallback {
    code: String,
}
