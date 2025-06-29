use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl, StandardRevocableToken,
    TokenResponse, TokenUrl,
};
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use url::Url;
use time::OffsetDateTime;
use reqwest::header::HeaderValue;
use reqwest::Request;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use std::sync::Arc;
use tokio::sync::RwLock;
use futures::Future;
use async_trait::async_trait;
use crate::error::{Error, Result};
use crate::auth::Authorization;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString, Serialize, Deserialize)]

pub enum Scope {
    #[strum(serialize = "tweet.read")]
    #[serde(rename = "tweet.read")]
    TweetRead,
    #[strum(serialize = "tweet.write")]
    #[serde(rename = "tweet.write")]
    TweetWrite,
    #[strum(serialize = "tweet.moderate.write")]
    #[serde(rename = "tweet.moderate.write")]
    TweetModerateWrite,
    #[strum(serialize = "users.read")]
    #[serde(rename = "users.read")]
    UsersRead,
    #[strum(serialize = "follows.read")]
    #[serde(rename = "follows.read")]
    FollowsRead,
    #[strum(serialize = "follows.write")]
    #[serde(rename = "follows.write")]
    FollowsWrite,
    #[strum(serialize = "offline.access")]
    #[serde(rename = "offline.access")]
    OfflineAccess,
    #[strum(serialize = "space.read")]
    #[serde(rename = "space.read")]
    SpaceRead,
    #[strum(serialize = "mute.read")]
    #[serde(rename = "mute.read")]
    MuteRead,
    #[strum(serialize = "mute.write")]
    #[serde(rename = "mute.write")]
    MuteWrite,
    #[strum(serialize = "like.read")]
    #[serde(rename = "like.read")]
    LikeRead,
    #[strum(serialize = "like.write")]
    #[serde(rename = "like.write")]
    LikeWrite,
    #[strum(serialize = "list.read")]
    #[serde(rename = "list.read")]
    ListRead,
    #[strum(serialize = "list.write")]
    #[serde(rename = "list.write")]
    ListWrite,
    #[strum(serialize = "block.read")]
    #[serde(rename = "block.read")]
    BlockRead,
    #[strum(serialize = "block.write")]
    #[serde(rename = "block.write")]
    BlockWrite,
    #[strum(serialize = "bookmark.read")]
    #[serde(rename = "bookmark.read")]
    BookmarkRead,
    #[strum(serialize = "bookmark.write")]
    #[serde(rename = "bookmark.write")]
    BookmarkWrite,
}

/// Oauth2Client is a wrapper around the oauth2::BasicClient for handling OAuth 2.0 authentication with Twitter.
///
/// This struct provides methods to:
/// - Initialize a new OAuth2 client with Twitter's endpoints.
/// - Generate an authorization URL for user consent (`auth_url`).
/// - Exchange an authorization code for an access token (`request_token`).
/// - Revoke an access or refresh token (`revoke_token`).
/// - Refresh an access token if expired (`refresh_token`, `refresh_token_if_expired`).
///
/// It abstracts the details of the OAuth2 protocol, making it easier to authenticate and manage tokens
/// when interacting with the Twitter API. The client is tailored for Twitter's OAuth2 endpoints and handles
/// token management, including refreshing and revoking tokens.
///
#[derive(Clone, Debug)]
pub struct Oauth2Client(BasicClient);

impl Oauth2Client {

    pub fn new(client_id: impl ToString, client_secret: impl ToString, callback_url: Url) -> Self {
        Self::new_impl(client_id, Some(client_secret), callback_url)
    }

    fn new_impl(
        client_id: impl ToString,
        client_secret: Option<impl ToString>,
        callback_url: Url,
    ) -> Self {
        Self(
            BasicClient::new(
                ClientId::new(client_id.to_string()),
                client_secret.map(|client_secret| ClientSecret::new(client_secret.to_string())),
                AuthUrl::from_url("https://twitter.com/i/oauth2/authorize".parse().unwrap()),
                Some(TokenUrl::from_url(
                    "https://api.twitter.com/2/oauth2/token".parse().unwrap(),
                )),
            )
            .set_revocation_uri(RevocationUrl::from_url(
                "https://api.twitter.com/2/oauth2/revoke".parse().unwrap(),
            ))
            .set_redirect_uri(RedirectUrl::from_url(callback_url)),
        )
    }

    pub fn auth_url(
        &self,
        challenge: PkceCodeChallenge,
        scopes: impl IntoIterator<Item = Scope>,
    ) -> (Url, CsrfToken) {
        self.0
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter().map(|s| oauth2::Scope::new(s.to_string())))
            .set_pkce_challenge(challenge)
            .url()
    }

    pub async fn request_token(
        &self,
        code: AuthorizationCode,
        verifier: PkceCodeVerifier,
    ) -> Result<Oauth2Token> {
        let res = self
            .0
            .exchange_code(code)
            .set_pkce_verifier(verifier)
            .request_async(oauth2::reqwest::async_http_client)
            .await?;
        res.try_into()
    }

    pub async fn revoke_token(
        &self, 
        token: StandardRevocableToken
    ) -> Result<()> {
        self.0
            .revoke_token(token)?
            .request_async(oauth2::reqwest::async_http_client)
            .await?;
        Ok(())
    }

    pub async fn refresh_token(
        &self,
        token: &RefreshToken
    ) -> Result<Oauth2Token> {
        let res = self.0
            .exchange_refresh_token(token)
            .request_async(oauth2::reqwest::async_http_client)
            .await?;
        res.try_into()
    }

    pub async fn refresh_token_if_expired(
        &self,
        token: &mut Oauth2Token
    ) -> Result<bool> {
        if token.is_expired() {
            if let Some(refresh_token) = token.refresh_token() {
                *token = self.refresh_token(refresh_token).await?;
                Ok(true)
            } else {
                Err(Error::NoRefreshToken)
            }
        } else {
            Ok(false)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Oauth2Token {
    access_token: AccessToken,
    refresh_token: Option<RefreshToken>,
    #[serde(with = "time::serde::rfc3339")]
    expires: OffsetDateTime,
    scopes: Vec<Scope>,
}

impl Oauth2Token {
    pub fn access_token(&self) -> &AccessToken {
        &self.access_token
    }

    pub fn refresh_token(&self) -> Option<&RefreshToken> {
        self.refresh_token.as_ref()
    }

    pub fn expires(&self) -> OffsetDateTime {
        self.expires
    }

    pub fn is_expired(&self) -> bool {
        self.expires < OffsetDateTime::now_utc()
    }
    pub fn scopes(&self) -> &[Scope] {
        &self.scopes
    }
    pub fn revokable_token(&self) -> StandardRevocableToken {
        if let Some(refresh_token) = self.refresh_token.as_ref() {
            StandardRevocableToken::RefreshToken(refresh_token.clone())
        } else {    
            StandardRevocableToken::AccessToken(self.access_token.clone())
        }
    }
}

impl TryFrom<BasicTokenResponse> for Oauth2Token {
    type Error = Error;

    fn try_from(token: BasicTokenResponse) -> Result<Self, Self::Error> {
        Ok(Self {
            access_token: token.access_token().clone(),
            refresh_token: token.refresh_token().cloned(),
            expires: OffsetDateTime::now_utc()
                + token.expires_in().ok_or_else(|| {
                    Error::Oauth2TokenError(BasicRequestTokenError::Other(
                        "Missing expiration".to_string(),
                    ))
                })?,
            scopes: token
                .scopes()
                .ok_or_else(|| {
                    Error::Oauth2TokenError(BasicRequestTokenError::Other(
                        "Missing scopes".to_string(),
                    ))
                })?
                .iter()
                .map(|s| {
                    s.parse::<Scope>().map_err(|_| {
                        Error::Oauth2TokenError(BasicRequestTokenError::Other(
                            "Invalid scope".to_string(),
                        ))
                    })
                })
                .collect::<Result<Vec<_>>>()?,
        })
    }
}

#[async_trait]
impl Authorization for Oauth2Token {
    async fn header(&self, _request: &Request) -> Result<HeaderValue> {
        format!("Bearer {}", self.access_token().secret())
            .parse()
            .map_err(Error::InvalidAuthorizationHeader)
    }

}

fn no_op(_: Oauth2Token) -> futures::future::Ready<Result<()>> {
    futures::future::ready(Ok(()))
}

pub type NoCallback = fn(Oauth2Token) -> futures::future::Ready<Result<()>>;

#[derive(Clone, Debug)]
pub struct RefreshableOauth2Token<C> {
    oauth_client: Oauth2Client,
    token: Arc<RwLock<Oauth2Token>>,
    callback: C,
}


impl RefreshableOauth2Token<NoCallback> {
    pub fn new(oauth_client: Oauth2Client, token: Oauth2Token) -> Self {
        Self {
            oauth_client,
            token: Arc::new(RwLock::new(token)),
            callback: no_op,
        }
    }
}

impl<C> RefreshableOauth2Token<C> {
    pub fn with_callback<T>(&self, callback: T) -> RefreshableOauth2Token<T> {
        RefreshableOauth2Token {
            oauth_client: self.oauth_client.clone(),
            token: self.token.clone(),
            callback,
        }
    }

    pub async fn token(&self) -> tokio::sync::RwLockReadGuard<'_, Oauth2Token> {
        self.token.read().await
    }

    pub async fn revoke(&self) -> Result<()> {
        self.oauth_client
            .revoke_token(self.token.read().await.revokable_token())
            .await
    }
}

impl<C, F> RefreshableOauth2Token<C>
where
    C: Fn(Oauth2Token) -> F + Send + Sync,
    F: Future<Output = Result<()>>,
{
    pub async fn refresh(&self) -> Result<()> {
        let mut token = self.token.write().await;
        *token = self
            .oauth_client
            .refresh_token(token.refresh_token.as_ref().ok_or(Error::NoRefreshToken)?)
            .await?;
        (self.callback)(token.clone()).await?;
        Ok(())
    }
}

#[async_trait]
impl<C, F> Authorization for RefreshableOauth2Token<C>
where 
    C: Fn(Oauth2Token) -> F + Send + Sync,
    F: Future<Output = Result<()>> + Send,
{
    async fn header(&self, request: &Request) -> Result<HeaderValue> {
        let mut token = self.token.write().await;
        if self 
            .oauth_client
            .refresh_token_if_expired(&mut token)
            .await?
        {
            (self.callback)(token.clone()).await?;
        }
        token.header(request).await
    }
}
    