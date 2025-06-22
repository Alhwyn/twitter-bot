use reqwest::{header, Client};
use std::error::Error;

pub struct TwitterField {
    pub client: Client,
    pub api_base_url: String,
    pub bearer_token: String,
}

impl TwitterField {

    pub fn get_field(&self) -> String{

        let output = format!("{} {} {}",
            self.api_base_url,
            self.bearer_token
        );

        return output
    }

    pub fn new(config: TwitterField) -> Result<Self, Box<dyn Error>> {

        let mut headers = header::HeaderMap::new();

        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.bearer_token))?,
        );

        println!("Bearer token set for authentication.");


        Ok(TwitterField {
            client: config.client,
            api_base_url: config.api_base_url,
            bearer_token: config.bearer_token,
        })
    }
}
