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
//this is where this snippet came from
//https://v2.tauri.app/plugin/stronghold/
//
//
//

pub async fn request_account_data_access() -> Result<String,String>{
 
    let started_port = start_oauth_server().await;
    let request_confirmed = request_redirect(started_port.unwrap()).await;
    Ok("mewo".to_string())
}


async fn start_oauth_server() -> Result<u16, u16> {
    let app = Router::new().route("/callback", get(|| async { "You can close this window now :3" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

     println!("axum is starting the server at localhost:3000...");

     let port = listener
         .local_addr()
         .map_err(|_| 2u16)?
         .port();

    tokio::spawn(async move {
       if let Err(err) = axum::serve(listener, app).await {
           eprintln!("Server error: {err}");
       }
    });

   
    Ok(port)
}

async fn request_redirect(port: u16) -> Result<String, String> {
    
    let redirect_uri = "http://127.0.0.1/"; //, port);

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
       // .append_pair("client_id", "client_id_secret")
        //.append_pair("response_type", "code")
       // .append_pair("redirect_uri", &redirect_uri)
       // .append_pair("scope", "identity");
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
