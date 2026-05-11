//use std::process::id;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;
use tauri::{Config, generate_context};
use tiny_http::{Server, Response};
use url::Url; 
use std::thread;
use open;


mod authentication;

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

#[tauri::command]
async fn authenticate_user(do_something: String) -> String{

    let cat = format!("meow");

    cat 
}

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
        .invoke_handler(tauri::generate_handler![search_gw2, authenticate_user])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
