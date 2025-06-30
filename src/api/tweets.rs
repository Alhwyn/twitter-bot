use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::auth::Authorization;
use crate::id::IntoNumericId;
use crate::requests::TweetBuilder;
use reqwest::Method;

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn post_tweet(&self) -> TweetBuilder<A> {
        TweetBuilder::new(self, self.url("tweets").unwrap())
    }

    pub async fn delete_tweet(&self, id: impl IntoNumericId) -> ApiResult<()> {
        self.send(self.request(
            Method::DELETE,
            self.url(format!("tweets/{}", id.into_id()))?,
        ))
        .await
    }
}
