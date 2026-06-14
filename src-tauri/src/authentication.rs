use axum::{
    routing::get, 
    Router,
    extract::Query,
    response::Html
};
use keyring::use_native_store;
use std::{
    thread,
    fs,
    sync::{ 
        Arc,
        Mutex
    },
    path::PathBuf
};
use keyring_core::{
    Entry,
    Result
};
use base64::{
    engine::general_purpose::STANDARD,
    Engine
};
use rand::RngExt;
use open::that;
use url::Url;
use serde::Deserialize;
use secrecy::{
    SecretString,
    ExposeSecret
};

const test: &str = env!("API_TEST");
pub async fn request_private_data_api_key(desired_scope: Vec<ApiKeyScope>) -> ApiKey{

    //this allows devs to get a key to access private user data

    let new_key = ApiKey{
        key: SecretString::new("some_key".into()),
        duration: 60,
        scope: desired_scope,
        token_info: format!("get the token info from the api endpoint")
    };

    new_key
}

fn get_or_set_oauth_password(){
let test_secret: QuagginKeyringSecurity = serde::from_str(raw_config).expect("failure"); 

}


fn get_arg_string(position: usize, default: &str) -> String {
    std::env::args()
        .nth(position)
        .unwrap_or_else(|| String::from(default))
}

#[derive(Debug, Deserialize)]
struct OAuthCallback{
    code: String,
}

#[derive(Deserialize, Clone)]
pub struct QuagginKeyringSecurity{
     keyring_entry_name: String,
     keyring_username: String,
     Oauth2Configuration: Oauth2Configuration,
}
#[derive(Deserialize, Clone)]
struct Oauth2Configuration{
     oauth2_link: String,
     redirect_routing_endpoint: String,
     redirect_uri: String,
     client_id: String,
     response_type: String,
     scope: String,
     prompt: String,
     include_granted_scopes: String,
}

impl QuagginKeyringSecurity{
    fn oauth2_configuaration(&self) -> &Oauth2Configuration{
        &self.Oauth2Configuration
    }

    fn create_keyring_entry(&self, secret_to_store: SecretString) -> Result<Entry>{

        //set the OS credential store based on the operating system
        use_native_store(true);

        println!("keyring entry name: {}", &self.keyring_entry_name);

        println!("keyring user name {}", &self.keyring_username);

        //prepare the new keyring entry in the system
        let keyring_entry = Entry::new(&self.keyring_entry_name, &self.keyring_username)?; 

        //create the password
        let mut bytes = [0u8; 32];
        rand::rng().fill(&mut bytes);
        let new_password = STANDARD.encode(&bytes);

        //set the secret
        keyring_entry.set_password(&new_password)?;
        keyring_entry.set_secret(&secret_to_store.expose_secret().as_bytes());


        Ok(keyring_entry)
    }

    fn check_keyring_entry(&self) -> Result<bool>{

        Ok(true)
    }
}
impl Oauth2Configuration {
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub fn response_type(&self) -> &str {
        &self.response_type
    }
    pub fn scope(&self) -> &str {
        &self.scope
    }
    pub fn prompt(&self) -> &str {
        &self.prompt
    }
    pub fn include_granted_scopes(&self) -> &str {
        &self.include_granted_scopes
    }
    pub async fn get_user_consent(&self) -> &str{
        let (tx, _rx) = oneshot::channel::<String>();

        let tx = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));

        let auth_code = "";

        let app = Router::new().route(
            &self.redirect_routing_endpoint, 
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
        //paramaterize the listener incase we ever want to change it 
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
        let request_url = Url::parse_with_params(&self.oauth2_link, 
            &[
                ("client_id", &self.client_id),
                ("response_type", &self.response_type),
                ("redirect_uri", &self.redirect_uri),
                ("scope", &self.scope),
                ("prompt", &self.prompt),
                ("include_granted_scopes", &self.include_granted_scopes)
                //,
                //("code_verifier", "PKCE challenge"),
            ]);
 
        let string_url = request_url.unwrap().to_string();
        println!("{}", string_url);
        that(string_url);
        &auth_code
    }
}

pub struct ApiKey{
    key: SecretString, 
    duration: i32,
    scope: Vec<ApiKeyScope>,
    //call the API to return information about the token
    token_info: String
}

impl ApiKey{
    pub fn Key(&self) -> &SecretString{
        &self.key
    }
    pub fn Duration(&self) -> &i32{
        &self.duration
    }
    pub fn Scope(&self) -> &Vec<ApiKeyScope>{
        &self.scope
    }

    pub fn TokenInfo(&self) -> &str{
       //return information from the /tokeninfo endpoint for the dev
        &self.token_info
    }
}

pub enum ApiKeyScope {
    //this is the enum to keep track of endpoints that require authentication 
    //Some are not included in this enum, see other comments marked with N/A

    // /account scopes
    Account(Vec<AccountScopes>),

    // /characters scopes
    Characters(Vec<CharacterScopes>),

    // /commerce scopes
    Commerce(Vec<CommerceScopes>),

    //N/A: we will not include CreateSubToken, we don't want devs trying to use that

    // /guild scopes
    Guild(Vec<GuildScopes>),

    // /pvp scopes
    Pvp(Vec<PvpScopes>),

    //N/A: Token info will be retrievable from the ApiKey struct we give the dev

}

pub enum AccountScopes{
    Account,
    Achievments,
    Bank,
    BuildStorage,
    DailyCrafting,
    Dungeons,
    Dyes,
    Emotes,
    Finishers,
    Gliders,
    Inventory,
    JadeBots,
    LegendaryArmory,
    Luck,
    Mail,
    MailCarriers,
    MapChests,
    Masteries,
    Materials,
    Minis,
    Novelties,
    Outfits,
    Progression,
    Raids,
    Recipes,
    Skiffs,
    Titles,
    Wallet,
    WorldBosses,
    WvW,
    WizardsVault(Vec<WizardsVaultScopes>),
    AccountPvp(Vec<AccountPvpScopes>),
    Mounts(Vec<MountScopes>),
    Home(Vec<HomeScopes>),
    Homestead(Vec<HomesteadScopes>),
    Mastery(Vec<MasteryScopes>),

}

pub enum WizardsVaultScopes{
    Daily,
    Listings,
    Special,
    Weekly
}

pub enum AccountPvpScopes{
    Heroes
}

pub enum MountScopes{
    Skins,
    Types,
}

pub enum HomeScopes{
    Cats,
    Nodes,
}

pub enum HomesteadScopes{
    Decorations,
    Glyphs,

}

pub enum MasteryScopes{
    Points,
}

pub enum CharacterScopes{
    Characters,
    Backstory,
    BuildTabs(Vec<BuildTabsScopes>),
    Core,
    Crafting,
    Dungons,
    Equipment,
    EquipmentTabs(Vec<EquipmentTabsScopes>),
    HeroPoints,
    Inventory,
    Quests,
    Recipes,
    Sab,
    Skills,
    Specializations,
    Training
}
pub enum BuildTabsScopes{
    BuildTabs,
    Active
}

pub enum EquipmentTabsScopes{
    EquipmentTabs,
    Active
}

pub enum CommerceScopes{
    Delivery,
    Transactions
}

pub enum GuildScopes{
    Guild,
    Log,
    Members,
    Ranks,
    Storage,
    Teams,
    Treasury,
    GuildSpecificUpgrades
    // this one is guild/:id/upgrades in the API
    // as opposed to /guild/upgrades which does not need authentication

}

pub enum PvpScopes{
    Games,
    Standings,
    Stats
}
