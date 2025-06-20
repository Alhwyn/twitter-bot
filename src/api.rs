use std::env;
use reqwest::{header, Client};
use std::error::Error;

pub struct TwitterConfig {
    pub client: Client,
    pub api_base_url: String,
    pub bearer_token: String,
}

pub struct TwitterAuth {
    pub signature_method: String,
    pub consumer_key: String,
    pub consumer_key_secret: String,
    pub access_token: String,
    pub token_secret: String,
}

pub struct TwitterField {
    pub client: Client,
    pub api_base_url: String,
    pub bearer_token: String,
}

impl TwitterField {
    pub fn new(config: TwitterConfig) -> Result<Self, Box<dyn Error>> {

        // set up the headers 
        let mut headers = header::HeaderMap::new();

        // now build the header tod the resoet 
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.bearer_token))?,
        );
        Ok(TwitterField {
            client: config.client,
            api_base_url: config.api_base_url,
            bearer_token: config.bearer_token,
        })
    }
}
