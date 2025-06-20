mod api;

use crate::api::{TwitterField, TwitterConfig};


fn main() {
    let client = reqwest::Client::new();
    let config = TwitterConfig {
        client, 
        api_base_url: "https://api.twitter.com/2".to_string(),
    };

    match TwitterField::new(config) {
        Ok(twitter) => {

        }
        Err(e) => {
            eprintln!("Error initializing TwitterField: {}", e);
        }
    }


}