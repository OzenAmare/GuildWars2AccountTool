use axum::{
    routing::get, 
    Router,
    serve::Serve
};
use iota_stronghold::Stronghold;
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
//this is where this snippet came from
//https://v2.tauri.app/plugin/stronghold/
//
//
//

pub async fn request_account_data_access() -> Result<String,String>{
 
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


struct Vault {
    stronghold: Stronghold,
    snapshot_path: String,
}

struct StrongholdState {
    stronghold: Stronghold,
    storage: std::sync::Mutex<String>,
}

#[derive(Debug, Deserialize)]
struct OAuthCallback {
    code: String,
}
