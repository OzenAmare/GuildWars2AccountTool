// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;

#[derive(Deserialize)]
struct Gw2Item{
    name: String,
}



#[tauri::command]
async fn greet(name: String) -> Result<String, String> {
    let id = 30704; 
    let url = format!("https://api.guildwars2.com/v2/items/{}", id);

    let item: Gw2Item = reqwest::get(url)
        .await
        .map_err(|e| e.to_string())?
        .json::<Gw2Item>()
        .await
        .map_err(|e| e.to_string())?;
    
        
        Ok(format!("Hello {}, item is {}", name, item.name))

}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
