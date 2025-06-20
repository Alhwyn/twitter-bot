mod api;

use crate::api::{TwitterField};


fn main() {
    let client = reqwest::Client::new();
    let config = TwitterField {
        client, 
        api_base_url: "https://api.twitter.com/2".to_string(),
        bearer_token: "YOUR_BEAR".to_string(),
    };

    match TwitterField::new(config) {
        Ok(twitter) => {
            println!("TwitterField initialized successfully.");
            // Use the twitter instance to call methods that read its fields
            twitter.print_base_url();

        }
        Err(e) => {
            eprintln!("Error initializing TwitterField: {}", e);
        }
    }


}