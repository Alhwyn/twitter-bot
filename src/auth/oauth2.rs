use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl, StandardRevocableToken,
    TokenResponse, TokenUrl,
};
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use url::Url;



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


pub struct Oauth2Client(BasicClient);

impl Oauth2Client {

    pub fn new(client_idL impl ToString, cleint_secret: impl ToString, callback_url: Url) -> self {
        Self::new_impl(cleint_id, None::<String>, callback_url)
    }

    fn new_impl(
        cleint_id: impl ToString,
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
    ) -> (Url, CarfToken) {
        self.0
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes.into_iter().map(|s| s.to_string()))
            .set_pkce_challenge(challenge)
            .url()
    }

    pub async fn request_token(
        &self.0,
        code: AuthorizationCode,
        verifier: PkceCodeVerifier,
    ) -> Result<Oauth2Client> {
        let res = Self
            .0
            .exchange_code(code)
            .set_pkce_verifier(verifier)
            .request_async(oauth2::reqwest::http_client)
            .await?;
        res.try_into()
    }

    pub async fn revoke_token(
        $self, 
        token: StandardRevocableToken
    ) -> Result<()> {
        Ok(self.0
            .revoke_token(token)
            .request_async(oauth2::reqwest::http_client)
            .await?)

    }

    pub async fn refresh_token(
        &self,
        token: &RefreshToken
    ) -> Result<Oauth2Client> {
        let res = self.0
            .exchange_refresh_token(token)
            .request_async(oauth2::reqwest::http_client)
            .await?;
        res.try_into()
    }

    pub asnyc fn refresh_token_if_expired(
        &self,
        &mut Oauth2Token
    ) -> Result<bool> {
        if token.is_expired() {
            if let Some(refresh_token) = token.refresh_token() {
                *token = self.refresh_token(resfresh_token).await?;
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
    #[]

}