use super::TwitterApi;
use crate::auth::Authorization;

use crate::id::IntoNumericId;
use crate::query::GetTweetsRequestBuilder;
use crate::requests::{StreamRuleBuilder, TweetBuilder, TweetId};

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn get_tweets(
        &self,
        ids: impl IntoIteratir<Item = impl IntoNumericId>,
    ) -> GetTweetsRequestBuilder<A, Vec<Tweet>, ()> {
        let mut url = self.url("tweets").unwrap();

        url.append_query_seq("ids", ids);
        GetTweetsRequestBuilder::new(self, url)
    }

    pub fn get_tweet(&self, id: impl IntoNumericId) -> GetTweetsRequestBuilder<A, Tweet, ()> {
        GetTweetsRequestBuilder::new(self, self.url(format!("tweets/{id}")).unwrap())
    }

    pub fn post_tweet(&self) -> TweetBuilder<A> {
        TweetBuilder::new(self, self.url("tweets").unwrap())
    }

    pub async fn delete_tweet(&self, id: impl IntoNumericId) -> ApiResult<A, Deleted, ()> {
        self.send(self.request(Method::DELETE, self.url(format!("tweets/{id}"))?))
            .await
    }

    pub fn get_user_tweets(
        &self,
        user_id: impl IntoNumericId,
    ) -> GetTimelineRequestBuilder<A, Vec<Tweet>, TweetsMeta> {
        GetTimelineRequestBuilder::new(self, self.url(format!("users/{user_id}/tweets")).unwrap())
    }
}
