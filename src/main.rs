use egg_mode::{KeyPair, Token, tweet::DraftTweet};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let consumer_key = env::var("CLIENT_ID")?;
    let consumer_secret = env::var("CLIENT_SECRET")?;
    let access_token = env::var("ACCESS_TOKEN")?;
    let access_token_secret = env::var("ACCESS_TOKEN_SECRET")?;

    let consumer_token = KeyPair::new(consumer_key, consumer_secret);
    let access_token = KeyPair::new(access_token, access_token_secret);
    let token = Token::Access {
        consumer: consumer_token,
        access: access_token,
    };

    let draft = DraftTweet::new("Hello, world from egg-mode!");
    let tweet = draft.send(&token).await?;

    println!("Tweet posted: {:?}", tweet);

    Ok(())
}