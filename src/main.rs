use std::env;
use reqwest::{header, Client};
use serde::Serialize;
use std::error::Error;

#[derive(Debug)]
pub struct TwitterConfig {
    pub bearer_token: String,
    pub api_base_url: Option<String>,
}

pub struct TwitterIntegration {
    client: Client,
    api_base_url: String,
}

#[derive(Debug, Serialize)]
struct TweetBody<'a> {
    text: &'a str,
}

impl TwitterIntegration {
    pub fn new(config: TwitterConfig) -> Result<Self, Box<dyn Error>> {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(&format!("Bearer {}", config.bearer_token))?,
        );
        headers.insert(header::CONTENT_TYPE, header::HeaderValue::from_static("application/json"));

        let client = Client::builder().default_headers(headers).build()?;

        Ok(Self {
            client,
            api_base_url: config.api_base_url.unwrap_or_else(|| "https://api.twitter.com/2".to_string()),
        })
    }

    pub async fn create_post(&self, tweet: &str) -> Result<(), Box<dyn Error>> {
        let url = format!("{}/tweets", self.api_base_url);

        let body = TweetBody { text: tweet };

        let res = self.client.post(&url).json(&body).send().await?;

        if !res.status().is_success() {
            let err_text = res.text().await?;
            return Err(format!("Failed to send tweet: {} - {}", res.status(), err_text).into());
        }

        println!("Tweet posted successfully.");
        Ok(())
    }
}



