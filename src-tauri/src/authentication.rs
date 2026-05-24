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
use std::sync::Arc;
use axum::response::Html;
use serde::Deserialize;
use iota_stronghold::Stronghold;
use anyhow::Result;
use std::path::PathBuf;
use once_cell::sync::OnceCell;
use tauri::AppHandle;
use getrandom;
use std::fs;
//this is where this snippet came from
//https://v2.tauri.app/plugin/stronghold/
//
//
//
//




async fn get_user_consent() -> Result<String, String>{

    //this function starts a webserver and returns their initial key for authorized access
    let (tx, rx) = oneshot::channel::<String>();

    let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

    let mut auth_code = "";

        let app = Router::new().route(
        "/callback", 
        get({
            let tx = tx.clone();

            move |Query(params): Query<OAuthCallback>|{
                let tx = tx.clone();

                async move {
                    println!("recieved the auth code {}", params.code);

                    if let Some(sender) = tx.lock().unwrap().take(){
                        let auth_code = sender.send(params.code.clone());
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
   
    //we need to add a randomly generated string to use as a code challenge
    //lets paramterize all of these seperately 
    let oauth2_link = "https://gw2.me/oauth2/authorize?";
    let request_url = Url::parse_with_params(oauth2_link, 
        &[
        ("client_id", "b185490f-b41b-40bc-9b1d-a5d7eb22ac68"),
        ("response_type", "code"),
        ("redirect_uri", "http://127.0.0.1:3000/callback"),
        ("scope", "identify"),
        ("prompt", "consent"),
        ("include_granted_scopes", "true")
        //,
        //("code_verifier", "PKCE challenge"),
        ]);
    
    let string_url = request_url.unwrap().to_string();
    println!("{}", string_url);
    open::that(string_url);

       
    Ok(auth_code.to_string())
}


pub struct QuagginSecurity{
    pub stronghold: Arc<tokio::sync::Mutex<Stronghold>>,
    pub authentication_configuration: AuthenticationConfiguration
    
}

 impl QuagginSecurity{
   pub async fn request_private_access(&self){
       let dog = get_user_consent().await;
       println!("This is the auth code: {:?}", dog);

    }
   async fn flip_access_token(&self){
       //this is where we'll get the proper access token and secure it in our framework
       //we need to hit this endpoint-> https://gw2.me/api/token
       //the following parameters are needed:
       //grant_type| "refresh_token" | Literally what we're asking for 
       //refresh_token| String |the refresh token itself 
       //client_id| String |our client id 
       //client_secret| String |our client secret
   }
}

#[derive(Deserialize)]
pub struct AuthenticationConfiguration{
    pub redirect_routing_endpoint: String,
    pub redirect_uri: String,
    pub oauth2_link: String,
    pub stronghold_snapshot_file: String,
    pub stronghold_storage_file: String,
    pub Oauth2Configuration: Oauth2Configuration,
}
#[derive(Deserialize)]
pub struct Oauth2Configuration{
    pub client_id: String,
    pub response_type: String,
    pub scope: String,
    pub prompt: String,
    pub include_granted_scopes: String,
}




#[derive(Debug, Deserialize)]
struct OAuthCallback {
    code: String,
}

