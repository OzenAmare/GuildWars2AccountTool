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
use rand::{
    RngExt,
    //distributions::Alphanumeric,
    Rng 
};
use sha2::{
    Sha256,
    Digest
};
use open::that;
use url::Url;
use serde::Deserialize;
use secrecy::{
    SecretBox,
    SecretString,
    ExposeSecret,
    zeroize::Zeroize
};
use std::sync::OnceLock;

pub async fn request_private_data_api_key(desired_scope: Vec<ApiKeyScope>) -> ApiKey{

    //this allows devs to get a key to access private user data
let test = std::env::var("API_TEST")
    .expect("API_TEST not set");
    println!("this is the variable {}", test);
    let new_key = ApiKey{
        key: SecretString::new("some_key".into()),
        duration: 60,
        scope: desired_scope,
        token_info: format!("get the token info from the api endpoint")
    };

    new_key
}

static QUAGGIN_IS_HERE: OnceLock<bool> = OnceLock::new();
static QUAGGIN_KEYRING_ENTRY: OnceLock<Entry> = OnceLock::new();
static QUAGGIN_SECURITY_CONFIGURATION: OnceLock<QuagginKeyringSecurity> = OnceLock::new();

pub fn load_security_configuration() {
    QUAGGIN_IS_HERE.get_or_init(|| {

        //load our security information
        QUAGGIN_SECURITY_CONFIGURATION.get_or_init(||{
            QuagginKeyringSecurity::load().expect("could not load security")
        });
        let secret_set = QUAGGIN_SECURITY_CONFIGURATION
            .get()
            .expect("NOOOO! QUAGGIN COULDN'T FIND SECRET!")
            .check_keyring_entry().expect("keyring failed");

        if secret_set{
            println!("the secret has been set, quaggin!");
        }else{
            println!("Oh no quaggin! The secret hasn't been set!");
            //QUAGGIN_SECURITY_CONFIGURATION.create_keyring_entry();
        }

        true
    });
}

pub async fn request_user_data(){

    let quaggin_security = QUAGGIN_SECURITY_CONFIGURATION.get()
        .expect("bad din't work");

    let dogs = quaggin_security.get_user_consent().await;


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
struct QuagginKeyringSecurity{
    keyring_entry_name: String,
    keyring_username: String,
    authorization_token_configuration: AuthoizationTokenConfiguration,
    access_token_configuration: AccessTokenConfiguration
}
impl QuagginKeyringSecurity{
    fn load() -> Result<Self>{

        //use whatever is the native OS credential store 
        use_native_store(true);

        //read the security configuration 
        let quaggin_security_config = std::fs::read_to_string("configuration/quaggin_keyring_security.toml").expect("could not read security.toml");
        let quaggin_security_struct: QuagginKeyringSecurity = toml::from_str(&quaggin_security_config).expect("could not parse security.toml");
       
        //set the entry in memory
        QUAGGIN_KEYRING_ENTRY.get_or_init(||{
            Entry::new(
                &quaggin_security_struct.keyring_entry_name, 
                &quaggin_security_struct.keyring_username
                ).expect("entry could not be intialized, that's very bad, quaggin.")
        });

        Ok(Self{
            keyring_entry_name: quaggin_security_struct.keyring_entry_name,
            keyring_username: quaggin_security_struct.keyring_username,
            authorization_token_configuration: quaggin_security_struct.authorization_token_configuration,
            access_token_configuration: quaggin_security_struct.access_token_configuration
        })
    }
    async fn get_user_consent(&self) -> Result<String>{

        let auth_token = &self.authorization_token_configuration.get_authoization_token()
            .await
            .expect("could not get auth token");

        let access_token: &SecretString = &self.access_token_configuration.get_access_token(auth_token)
            .await
            .expect("could not get access token");

        let access_token_open = access_token.expose_secret();
        println!("this is the access token: {}", access_token_open);
        Ok(access_token_open.to_string())

    }
    fn create_keyring_entry(&self) -> Result<&Entry>{

        //get keyring entry information
        let keyring_entry = self.keyring_entry();

        //create the password
        let mut bytes = [0u8; 32];
        rand::rng().fill(&mut bytes);
        let new_password = STANDARD.encode(&bytes);

        //set the secret
        keyring_entry.set_password(&new_password)?;

        Ok(keyring_entry)
    }
    fn check_keyring_entry(&self) -> Result<bool>{
        let keyring_entry = self.keyring_entry();
        let entry_exists = match keyring_entry.get_password(){
            Ok(entry) => !entry.is_empty(),
            Err(_) => false,
        };

        Ok(entry_exists)
    }
    fn set_keyring_entry(&self, secret: SecretString) -> Result<&str>{

        let keyring_entry = self.keyring_entry();
        let entry_status = match keyring_entry.set_password(secret.expose_secret()){
            Ok(_) => "password changed",
            Err(_) => "failed to change password",
        };

        Ok(&entry_status)

    }
    fn keyring_entry_name(&self) -> &str{
        &self.keyring_entry_name
    }
    fn keyring_username(&self) -> &str{
        &self.keyring_username
    }
    fn keyring_entry(&self) -> &Entry{
        QUAGGIN_KEYRING_ENTRY.get().expect("could not get QUAGGIN_KEYRING_ENTRY")
    }
  }

#[derive(Deserialize, Clone)]
struct AuthoizationTokenConfiguration{
     oauth2_link: String,
     redirect_routing_endpoint: String,
     redirect_uri: String,
     client_id: String,
     response_type: String,
     scope: String,
     prompt: String,
     include_granted_scopes: bool,
}
impl AuthoizationTokenConfiguration{
       async fn get_authoization_token(&self) -> Result<SecretString>{

           //let's start implementing PKCE flow. 
           //

           //let hasher = Sha256::new();

           let random_u128 = rand::rng().random::<u128>();
           let pkce_code_verifier = random_u128.to_string();
           let random_u128_hash = Sha256::digest(pkce_code_verifier);

           //let random_u128_hash_string = String::from(random_u128_hash);
           //Generate the random code verifier. 
            let hashed_pkce_code_challenge = SecretBox::new(
                Box::new(    
                    //let hasher = Sha256::new();
                    //Sha256::digest(rand::rng().random::<u128>())
                    random_u128_hash.to_string()
                    //rand::rng().random::<u128>()
                    //
                    //random_u128_hash
               )
            ); 
         
        //we need to add a randomly generated string to use as a code challenge
        //lets paramterize all of these seperately 
        let request_url = Url::parse_with_params(&self.oauth2_link, 
            &[
                ("client_id", &self.client_id),
                ("response_type", &self.response_type),
                ("redirect_uri", &self.redirect_uri),
                ("scope", &self.scope),
                ("prompt", &self.prompt),
                ("include_granted_scopes", &self.include_granted_scopes.to_string())
                ("code_verifier", hashed_pkce_code_challenge.expose_secret()),
                ("code_challenge_method", "S256")
            ]);
 
        //let string_url = request_url.unwrap().to_string();
        //println!("{}", string_url);
        //that(string_url);
        //let secret_auth_code = SecretString::new(String::from(auth_code).into_boxed_str());
       // println!("this is the auth code: {}", auth_code);
       // auth_code.to_owned().zeroize();
        Ok(secret_auth_code)

           }

    async fn get_refresh_token() -> Result<SecretString>{

        let cheese = SecretString::new(String::from("cheese").into_boxed_str());

        Ok(cheese)
    }

}

#[derive(Deserialize, Clone)]
struct AccessTokenConfiguration{

    //docs here https://gw2.me/dev/docs/access-tokens

    //endpoint to get an access token
    endpoint_url: String,

    //static paramaters to pass to the post request
    grant_type: String,
    client_id: String,
    redirect_uri: String, 
}
impl AccessTokenConfiguration{
    async fn get_access_token(&self, permission_code: &SecretString) -> Result<SecretString>{

        let client = reqwest::Client::new();
          let mut url = Url::parse(&self.endpoint_url).expect("could not parse access token url");

            
        let mut exposed_code = String::from(permission_code.expose_secret());
        let access_token_request_params = [
            ("grant_type", &self.grant_type),
            ("code", &exposed_code),
            ("client_id", &self.client_id),
            ("redirect_uri", &self.redirect_uri)
        ];
       
        let res = client.post(&self.endpoint_url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .form(&access_token_request_params)
            .send()
            .await
            .expect("could not send post request for access token");

        println!("This is the access token response: {:?}", res);
         exposed_code.zeroize();

        let cheese = SecretString::new(String::from("cheese").into_boxed_str());
        Ok(cheese)
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
