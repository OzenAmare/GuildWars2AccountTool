use tauri::Manager;
use open;
use std::thread;
use tiny_http::{Response, Server};
use url::Url;

//this is where this snippet came from
//https://v2.tauri.app/plugin/stronghold/
pub fn run(){
    tauri::Builder::default()
        .setup(|app| {
            let salt_path = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path")
                .join("salt.txt");
            app.handle().plugin(tauri_plugin_stronghold::Builder::with_argon2(&salt_path).build())?;

        Ok(())
        })
        .run(tauri::generate_context!())
            .expect("rror while running tauri application");
}
pub async fn start_oauth_server() -> String {
    let redirect_uri = "http://localhost:3000".to_string();

    thread::spawn(move || {
        let server = Server::http("127.0.0.1:3000/").unwrap();

        for request in server.incoming_requests() {
            let url = Url::parse(&format!("http://localhost{}", request.url())).unwrap();
            if let Some(code) = url.query_pairs().find(|(k, _)| k == "code") {
                println!("oauth code recieved");
            }

            let response =
                Response::from_string("<html><body>you can close this now</body></html>");
            request.respond(response).unwrap();
        }
    });
    redirect_uri
    //this will handle
    //the sever for oauth2 and return the uri
}

pub async fn oauth2_authoization() -> Result<String, String> {
    //let url = format!("https://account.guildwars2.com/oauth2/authorize");
    let client_id = "security meow :3";
    //  let scopes = "account characters wallet";
    let redirect_uri = "http://localhost:3000";
    let auth_url = format!("https://gw2.me/oauth2/authorize?client_id={}&response_type=code&redirect_uri={}&scope=identify&prompt=consent&include_granted_scopes=true", client_id, redirect_uri);
    //let auth_url = format!("https://gw2.me/oauth2/authorize?client_id={}&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2F&scope=identify&prompt=consent&include_granted_scopes=true", client_id);
    //  println!("meow{}",auth_url);

    open::that(&auth_url).expect("Failed to open browser");
    //let client = reqwest::Client::new();
    //let resp = client
    //.post(url)
    //.headers();

    return Ok(auth_url);
}

pub async fn oauth_login() -> Result<String, String> {
    let res = "meow";
    return Ok(format!("meow {}", res));
    //oauth2_authoization();
    //return res.await;
}
