mod api;
use std::env;
use crate::api::{TwitterField};


fn main() {
    let client = reqwest::Client::new();

    let bearer_token = env::var("BEARER_TOKEN").expect("TWITTER_BEARER_TOKEN must be set");


    let config = TwitterField {
        client, 
        api_base_url: "https://api.twitter.com/2".to_string(),
        bearer_token: format!("Bearer {}", bearer_token),
    };

    match TwitterField::new(config) {
        Ok(twitter) => {
            println!("TwitterField initialized successfully.");
            
            // Use the twitter instance to call methods that read its fields
            let twitter_field: String = twitter.get_field();


            println!("Twitter Field: {}", twitter_field);

        }
        Err(e) => {
            eprintln!("Error initializing TwitterField: {}", e);
        }
    }
}