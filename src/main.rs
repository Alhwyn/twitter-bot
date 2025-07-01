use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
use std::io::{self, Write};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::trace::TraceLayer;
use tracing;

use oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use tracing_subscriber::prelude::*;
use tweetterminal::auth::oauth2::{Oauth2Client, Oauth2Token, Scope};
use tweetterminal::TwitterApi;

pub struct Oauth2Ctx {
    client: Oauth2Client,
    verifier: Option<PkceCodeVerifier>,
    state: Option<CsrfToken>,
    token: Option<Oauth2Token>,
}

#[derive(Deserialize)]
struct CallbackParams {
    code: String,
    state: CsrfToken,
}

#[derive(Deserialize)]
struct TweetParams {
    text: Option<String>,
}

async fn login(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    let mut ctx = ctx.lock().unwrap();

    // create challenge
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

    // create the auth url
    let (url, state) = ctx.client.auth_url(
        challenge,
        [Scope::TweetRead, Scope::TweetWrite, Scope::UsersRead],
    );

    tracing::info!("Generated OAuth URL: {}", url);
    tracing::info!("Generated state: {}", state.secret());

    // set context for reference in callback
    ctx.verifier = Some(verifier);
    ctx.state = Some(state);

    // redirect to auth url
    Redirect::to(&url.to_string())
}

async fn callback(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Query(CallbackParams { code, state }): Query<CallbackParams>,
) -> impl IntoResponse {
    tracing::info!(
        "OAuth callback received with code: {}",
        &code[..10.min(code.len())]
    );

    let (client, verifier) = {
        let mut ctx = ctx.lock().unwrap();

        // get previous state from ctx (from login)
        let saved_state = ctx.state.take().ok_or_else(|| {
            tracing::error!("No previous state found in context");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No previous state found. Please start the login process again.".to_string(),
            )
        })?;

        // check state returned to see if it matches, or otherwise throw an error
        if state.secret() != saved_state.secret() {
            tracing::error!(
                "State mismatch: received {} vs saved {}",
                state.secret(),
                saved_state.secret()
            );
            return Err((
                StatusCode::BAD_REQUEST,
                "State does not match - possible CSRF attack".to_string(),
            ));
        }

        // get verifier from ctx
        let verifier = ctx.verifier.take().ok_or_else(|| {
            tracing::error!("No PKCE verifier found in context");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "No PKCE verifier found. Please start the login process again.".to_string(),
            )
        })?;
        let client = ctx.client.clone();
        (client, verifier)
    };

    // request oauth2 token
    tracing::info!("Exchanging authorization code for access token");
    let token = client
        .request_token(AuthorizationCode::new(code), verifier)
        .await
        .map_err(|e| {
            tracing::error!("Failed to exchange code for token: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("OAuth token exchange failed: {}", e),
            )
        })?;

    tracing::info!("Successfully obtained OAuth2 token");

    // set context for use with the twitter API
    ctx.lock().unwrap().token = Some(token);

    // Redirect to tweets endpoint (no automatic posting)
    Ok(Redirect::to("/tweets"))
}

async fn tweets(
    Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>,
    Query(params): Query<TweetParams>,
) -> impl IntoResponse {
    // get oaouth2 token

    let (mut oauth_token, oauth_client) = {
        let ctx = ctx.lock().unwrap();
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
            .clone();
        let client = ctx.client.clone();
        (token, client)
    };

    // refresh oauth  token if expired
    if oauth_client
        .refresh_token_if_expired(&mut oauth_token)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        // save oauth token if refreshed
        ctx.lock().unwrap().token = Some(oauth_token.clone());
    }

    let api = TwitterApi::new(oauth_token);

    // Only tweet if text parameter is provided
    let tweet_text = match params.text {
        Some(text) => {
            if text.trim().is_empty() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    "Tweet text cannot be empty".to_string(),
                ));
            }
            if text.len() > 280 {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Tweet too long! Maximum 280 characters, got {}", text.len()),
                ));
            }
            text
        }
        None => {
            // No text provided, return a message instead of posting a default tweet
            return Ok(Json(serde_json::json!({
                "message": "Ready to tweet! Provide a 'text' parameter to post a tweet.",
                "example": "http://127.0.0.1:3000/tweets?text=Hello%20World!",
                "authenticated": true
            })));
        }
    };

    let response = api
        .post_tweet()
        .text(tweet_text)
        .send()
        .await
        .map_err(|err| {
            tracing::error!("Failed to post tweet: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        })?;

    // Extract the tweet data from the API response
    let tweet_data = response.payload.data.ok_or_else(|| {
        tracing::error!("No tweet data in API response");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "No tweet data returned".to_string(),
        )
    })?;

    Ok::<_, (StatusCode, String)>(Json(serde_json::json!({
        "id": tweet_data.id.to_string(),
        "text": tweet_data.text,
        "success": true
    })))
}

async fn revoke(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
    // getting the oauth token
    let (oauth_token, oauth_client) = {
        let ctx = ctx.lock().unwrap();
        let token = ctx
            .token
            .as_ref()
            .ok_or_else(|| (StatusCode::UNAUTHORIZED, "User not logged in!".to_string()))?
            .clone();

        let client = ctx.client.clone();
        (token, client)
    };

    // revoke token
    oauth_client
        .revoke_token(oauth_token.revokable_token())
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;
    Ok::<_, (StatusCode, String)>("Token revoked successfully!".to_string())
}

#[allow(dead_code)]
async fn interactive_tweeting(ctx: Arc<Mutex<Oauth2Ctx>>) {
    println!("\nOAuth login successful! You can now tweet interactively.");

    loop {
        print!("\nEnter tweet: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tweet_text = input.trim();

                if tweet_text.is_empty() {
                    continue;
                }

                if tweet_text.eq_ignore_ascii_case("quit")
                    || tweet_text.eq_ignore_ascii_case("exit")
                {
                    println!("Goodbye!");
                    break;
                }

                // Get OAuth token and post tweet directly
                match post_tweet_direct(ctx.clone(), tweet_text.to_string()).await {
                    Ok(tweet_id) => {
                        println!("Tweet posted successfully! ID: {}", tweet_id);
                    }
                    Err(e) => {
                        println!("Failed to post tweet: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
}

async fn post_tweet_direct(
    ctx: Arc<Mutex<Oauth2Ctx>>,
    tweet_text: String,
) -> Result<String, String> {
    // Get OAuth token
    let (mut oauth_token, oauth_client) = {
        let ctx = ctx.lock().unwrap();
        let token = ctx.token.as_ref().ok_or("No OAuth token found")?.clone();
        let client = ctx.client.clone();
        (token, client)
    };

    // Refresh token if needed
    if oauth_client
        .refresh_token_if_expired(&mut oauth_token)
        .await
        .map_err(|e| format!("Token refresh failed: {}", e))?
    {
        // Save refreshed token
        ctx.lock().unwrap().token = Some(oauth_token.clone());
    }

    // Post tweet
    let api = TwitterApi::new(oauth_token);
    let response = api
        .post_tweet()
        .text(tweet_text)
        .send()
        .await
        .map_err(|e| format!("Tweet posting failed: {}", e))?;

    // Extract tweet ID
    let tweet_data = response.payload.data.ok_or("No tweet data in response")?;

    Ok(tweet_data.id.to_string())
}

async fn interactive_tweeting_background(ctx: Arc<Mutex<Oauth2Ctx>>) {
    // Wait for OAuth login to complete
    loop {
        let has_token = {
            let ctx = ctx.lock().unwrap();
            ctx.token.is_some()
        };

        if has_token {
            break;
        }

        // Wait a bit before checking again
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    }

    println!("\nlogin finish");

    loop {
        print!("\nenter tweet: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let tweet_text = input.trim();

                if tweet_text.is_empty() {
                    continue;
                }

                if tweet_text.eq_ignore_ascii_case("quit")
                    || tweet_text.eq_ignore_ascii_case("exit")
                {
                    println!("Goodbye! (Server will keep running)");
                    break;
                }

                if tweet_text.len() > 280 {
                    println!("Tweet too long! ({} characters, max 280)", tweet_text.len());
                    continue;
                }

                // Post tweet directly using the API
                match post_tweet_direct(ctx.clone(), tweet_text.to_string()).await {
                    Ok(tweet_id) => {
                        println!("Tweet posted successfully! ID: {}", tweet_id);
                    }
                    Err(e) => {
                        println!("Failed to post tweet: {}", e);
                    }
                }
            }
            Err(e) => {
                println!("Error reading input: {}", e);
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // load environment variables from .env file
    dotenv::dotenv().ok();

    // init tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "tweetterminal=info,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // serve on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // initialize Oauth2Client with id and secret the callback to this server
    let oauth_ctx = Oauth2Ctx {
        client: Oauth2Client::new(
            std::env::var("CLIENT_ID").expect("could not find CLIENT_ID"),
            std::env::var("CLIENT_SECRET").expect("could not find CLIENT_SECRET"),
            format!("http://{addr}/callback").parse().unwrap(),
        ),
        verifier: None,
        state: None,
        token: None,
    };

    let ctx = Arc::new(Mutex::new(oauth_ctx));

    // init server
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/tweets", get(tweets))
        .route("/revoke", get(revoke))
        .layer(Extension(ctx.clone()))
        .layer(TraceLayer::new_for_http());

    println!("\nTwitter Bot Server Starting...");
    println!("Open http://{}/login in your browser to authenticate", addr);
    println!("Waiting for OAuth login...");

    tracing::debug!("Serving at {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Start interactive tweeting in the background
    let ctx_clone = ctx.clone();
    tokio::spawn(async move {
        // Wait a bit for the server to start
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        interactive_tweeting_background(ctx_clone).await;
    });

    // Run server
    let server = axum::serve(listener, app);

    tokio::select! {
        _ = server => {},
        _ = tokio::signal::ctrl_c() => {
            println!("\nShutting down server...");
        }
    }
}
