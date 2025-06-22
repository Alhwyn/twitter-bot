use oauth2::{
    AccessToken, AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge,
    PkceCodeVerifier, RedirectUrl, RefreshToken, RevocationUrl, StandardRevocableToken,
    TokenResponse, TokenUrl,
};
use oauth2::basic::{BasicClient, BasicRequestTokenError, BasicTokenResponse};
use url::Url;


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

}