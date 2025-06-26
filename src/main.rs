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
use tracing_subscriber::prelude::*;

use twitter_v2::authorization::{Oauth2Client, Oauth2Token, Scope};
use twitter_v2::oauth2::{AuthorizationCode, CsrfToken, PkceCodeChallenge, PkceCodeVerifier};
use twitter_v2::TwitterApi;

pub struct Oauth2Ctx {
    client: Oauth2Client,
    verfier: Option<PkceCodeVerifier>,
    state: Option<CsrfToken>,
    token: Option<Oauth2Token>,
}

async fn login(Extension(ctx): xtension<Arc<Mutex<Oauth2Ctx>>>) -> into IntoResponse {
    let mut ctx = ctx.lock().unwrap();

    // creater challenge 
    let (challenge, verifier) = PkceCodeChallenge::new_random_sha256();

}





#[tokio::main]
async fn main()  {
    // init tracing 
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG")
                .unwrap_or_else(|_| "oauth2_callback=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // serve on port 3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    // initialize Oauth2Client with id and secret the callback to this server
    let oauth_ctx = Oauth2Ctx {
        client: Oauth2CLient::new(
            std::env:var::("CLIENT_ID").expect("could not find CLIENT_ID"),
            std::env::var("CLIENT_SECRET").expect("could not find CLIENT_SECRET"),
            format!("http://{addr}/callback").parse().unwrap(),
            verifier: State,
            state: None,
            token: None,
        )
    };

    // init server
    let app = Router::new()
        .route("/login", get(login))

}