use std::env;
use reqwest::{header, Client};

pub struct TwitterConfig {
    client: Cleint,
    api_base_url: String,
}

pub struct TwitterAuth {
    pub signatre_method: String,
    pub consumer_key: String,
    pub consumer_key_secret: String,
    pub access_token: String,
    pub token_secret: String,
}



impl TwitterField {
    pub fn new(config: TwitterConfig) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.bearer_token))?,
        );

    }


}


























/* pub async fn create_post(_tweet: &str) -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok(); 

    let auth_url  = env::var("AUTH_URL").expect("AUTH_URL must be set");

    
    println!("Twitter authentication logic goes here. {}", auth_url);


    let client = reqwest::Client::new();


    
    let _res = client
        .post(auth_url)
        .body("the exact body that is sent")
        .send()
        .await?;

    println!("Authentication request sent successfully. {}", _res.status());

    Ok(())
} */