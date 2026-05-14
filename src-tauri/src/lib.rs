//use std::process::id;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::{Deserialize, Serialize};
//use std::collections::HashMap;
use std::fs;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Gw2Item {
    id: u32,
    chat_link: String,
    name: String,

    icon: Option<String>,
    description: Option<String>,

    #[serde(rename = "type")]
    item_type: String,

    rarity: String,
    level: u32,
    vendor_value: u32,

    default_skin: Option<u32>,

    flags: Vec<String>,

    #[serde(default)]
    game_types: Vec<String>,

    #[serde(default)]
    restrictions: Vec<String>,

    upgrades_into: Option<Vec<Gw2Upgrade>>,
    upgrades_from: Option<Vec<Gw2Upgrade>>,

    details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Gw2Upgrade {
    upgrade: String,
    item_id: u32,
}

#[derive(Deserialize)]
struct Gw2Character {
    name: String,
    profession: String,
    level: u32,
    race: String,
}


// cache file? 
const CACHE_FILE: &str = "items_cache.json";

fn load_items_from_file() -> Result<Vec<Gw2Item>, String> {
    let text = fs::read_to_string(CACHE_FILE)
        .map_err(|e| e.to_string())?;

    let items: Vec<Gw2Item> = serde_json::from_str(&text)
        .map_err(|e| e.to_string())?;

    Ok(items)
}

async fn fetch_all_items_from_api() -> Result<Vec<Gw2Item>, String> {
    let mut all_items: Vec<Gw2Item> = Vec::new();

    let ids = reqwest::get("https://api.guildwars2.com/v2/items")
        .await
        .map_err(|e| e.to_string())?
        .json::<Vec<u32>>()
        .await
        .map_err(|e| e.to_string())?;

    for chunk in ids.chunks(200) {
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

        all_items.extend(items);
    }

    Ok(all_items)
}


fn save_items_to_file(items: &Vec<Gw2Item>) -> Result<(), String> {
    let json = serde_json::to_string(items)
        .map_err(|e| e.to_string())?;

    fs::write(CACHE_FILE, json)
        .map_err(|e| e.to_string())?;

    Ok(())
}

async fn get_all_items_cached() -> Result<Vec<Gw2Item>, String> {
    if let Ok(items) = load_items_from_file() {
        println!("Loaded {} items from cache file", items.len());
        return Ok(items);
    }

    println!("No cache file found. Fetching all items from API...");

    let items = fetch_all_items_from_api().await?;

    save_items_to_file(&items)?;

    println!("Saved {} items to cache file", items.len());

    Ok(items)
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

    println!("Searching cache for '{}'", wanted);

    let items = get_all_items_cached().await?;

    for item in items {

        if item.name.to_lowercase() == wanted {

            println!(
                "Found {} ({})",
                item.name,
                item.id
            );

            return Ok(format!(
                "{}: {}",
                item.id,
                item.name
            ));
        }
    }

    Err(format!(
        "No item found named '{}'",
        value
    ))
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
        .invoke_handler(tauri::generate_handler![search_gw2])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
