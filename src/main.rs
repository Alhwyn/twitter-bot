use axum::{
    extract::{Extension, Query},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::get,
    Json, Router,
};
use serde::Deserialize;
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
    ctx.lock().unwrap().token = Some(token.clone());

    // Automatically post a welcome tweet after successful login
    tracing::info!("Posting welcome tweet...");
    let api = TwitterApi::new(token);

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let welcome_text = format!(
        "🎉 Successfully logged in to Twitter! Ready to tweet! #{}",
        timestamp
    );

    match api.post_tweet().text(welcome_text).send().await {
        Ok(response) => {
            if let Some(tweet_data) = response.payload.data {
                tracing::info!("Welcome tweet posted successfully! ID: {}", tweet_data.id);
                return Ok(Redirect::to(&format!(
                    "/tweets?welcome_tweet={}",
                    tweet_data.id
                )));
            }
        }
        Err(e) => {
            tracing::error!("Failed to post welcome tweet: {}", e);
            // Don't fail the login process if welcome tweet fails
        }
    }

    Ok(Redirect::to("/tweets"))
}

async fn tweets(Extension(ctx): Extension<Arc<Mutex<Oauth2Ctx>>>) -> impl IntoResponse {
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

    // Use current timestamp to make each tweet unique
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let tweet_text = format!("Hello from my Twitter bot! 🤖 #{}", timestamp);

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

    Ok::<_, (StatusCode, String)>(Json(tweet_data))
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

    // init server
    let app = Router::new()
        .route("/login", get(login))
        .route("/callback", get(callback))
        .route("/tweets", get(tweets))
        .route("/revoke", get(revoke))
        .layer(Extension(Arc::new(Mutex::new(oauth_ctx))))
        .layer(TraceLayer::new_for_http());

    println!("\nOpen http://{}/login in your browser\n", addr);
    tracing::debug!("Serving at {}", addr);

    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
