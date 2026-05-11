//use std::process::id;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;
use tauri::{Config, generate_context};
use tiny_http::{Server, Response};
use url::Url; 
use std::thread;
use open;
#[derive(Deserialize)]
struct Gw2Item{
    name: String,
    id : u32
}

#[derive(Deserialize)]
struct Gw2Character {
    name: String,
    profession: String,
    level: u32,
    race: String,
}


//adding in comment to test repo and pipeline for rework

#[tauri::command]
async fn search_gw2(search_type: String, search_value: String) -> Result<String, String> {
    match search_type.as_str() {
        "item_id" => search_item_by_id(search_value).await,
        "item_name" => search_item_by_name(search_value).await,
        "character" => search_character(search_value).await,
        _ => Err("Unknown search type".to_string()),
    }
}

#[tauri::command]
fn start_oauth_server() -> String{
    let redirect_uri = "http://localhost:3000".to_string();

    thread::spawn(move || {
        let server = Server::http("127.0.0.1:3000/").unwrap();

        for request in server.incoming_requests(){
            let url = Url::parse(&format!("http://localhost{}", request.url())).unwrap();
            if let Some(code) = url.query_pairs().find(|(k, _)| k == "code"){
                println!("oauth code recieved");
            }

            let response = Response::from_string(
                "<html><body>you can close this now</body></html>"
                );
            request.respond(response).unwrap();
        }
    });
    redirect_uri
//this will handle
//the sever for oauth2 and return the uri
}

#[tauri::command]
async fn oauth2_authoization() -> Result<String, String>{

    let url = format!("https://account.guildwars2.com/oauth2/authorize");
    let client_id = "security meow :3";
    let scopes = "account characters wallet";
    let redirect_uri = start_oauth_server();

    //let auth_url = format!(
      //  "https://account.guildwars2.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri={}&scope=profile",
        //client_id, redirect_uri
    //);
let auth_url = format!("https://gw2.me/oauth2/authorize?client_id={}&response_type=code&redirect_uri={}&scope=identify&prompt=consent&include_granted_scopes=true", client_id, redirect_uri);
//let auth_url = format!("https://gw2.me/oauth2/authorize?client_id={}&response_type=code&redirect_uri=http%3A%2F%2Flocalhost%3A3000%2F&scope=identify&prompt=consent&include_granted_scopes=true", client_id);
    println!("meow{}",auth_url);

   open::that(&auth_url).expect("Failed to open browser");
    //let client = reqwest::Client::new();
    //let resp = client
        //.post(url)
        //.headers();

    return Ok(auth_url);
}

#[tauri::command]
async fn oauth_login() -> Result<String, String>{
   let res = oauth2_authoization();
    return res.await;
}

#[tauri::command]
async fn search_item_by_id(value: String) -> Result<String, String> {
    let id: u32 = value
        .parse()
        .map_err(|_| "Item ID must be a number".to_string())?;

    let url = format!("https://api.guildwars2.com/v2/items/{}", id);

    let item: Gw2Item = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json::<Gw2Item>()
        .await
        .map_err(|e| e.to_string())?;
    
        
    Ok(format!("{}: {}", id, item.name))
}

async fn search_item_by_name(value: String) -> Result<String, String> {

    let wanted = value.to_lowercase();

    println!("Starting item name search for: {}", wanted);

    let ids = reqwest::get("https://api.guildwars2.com/v2/items")
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<u32>>()
        .await
        .map_err(|e| e.to_string())?;

    println!("Fetched {} item IDs", ids.len());

    let mut chunk_index = 0;

    for chunk in ids.chunks(200) {

        chunk_index += 1;

        println!(
            "Processing chunk {} ({} ids)",
            chunk_index,
            chunk.len()
        );

        let ids_text = chunk
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<String>>()
            .join(",");

        let url = format!("https://api.guildwars2.com/v2/items?ids={}", ids_text);

        let items = reqwest::get(&url)
            .await
            .map_err(|e| e.to_string())?
            .json::<Vec<Gw2Item>>()
            .await
            .map_err(|e| e.to_string())?;

        println!(
            "Chunk {} returned {} items",
            chunk_index,
            items.len()
        );

        for item in items {

            println!("Checking item: {}", item.name);

            if item.name.to_lowercase() == wanted {

                println!(
                    "FOUND MATCH: {} ({})",
                    item.name,
                    item.id
                );

                return Ok(format!("{}: {}", item.id, item.name));
            }
        }
    }

    println!("No item found");

    Err(format!("No item found named '{}'", value))
}

async fn search_character(character_name: String) -> Result<String, String> {
    let api_key = "API_KEY";

    let encoded_name = character_name.replace(" ", "%20");

    let url = format!(
        "https://api.guildwars2.com/v2/characters/{}?access_token={}",
        encoded_name,
        api_key
    );

    let character = reqwest::get(&url)
        .await
        .map_err(|e| e.to_string())?
        .json::<Gw2Character>()
        .await
        .map_err(|e| e.to_string())?;

    Ok(format!(
        "{} is a level {} {} {}",
        character.name,
        character.level,
        character.race,
        character.profession
    ))
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![search_gw2, oauth_login, oauth2_authoization])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
