use reqwest::{header, Client};
use std::error::Error;

pub struct TwitterField {
    pub client: Client,
    pub api_base_url: String,
    pub bearer_token: String,
}

impl TwitterField {

    pub fn print_base_url(&self) {
        println!("{}", self.api_base_url);
    }


    
    pub fn new(config: TwitterField) -> Result<Self, Box<dyn Error>> {

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
