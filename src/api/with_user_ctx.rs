use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::auth::Authorization;
use crate::data::{User};
use crate::error::Result;
use crate::id::{NumericId};

pub struct TwitterApiWithUserCtx<A> {
    user_id: NumericId,
    client: TwitterApi<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization + Clone,
{
    pub async fn with_user_ctx(&self) -> Result<TwitterApiWithUserCtx<A>> {
        // TODO: Implement actual user ID retrieval
        let user_id = NumericId::new(0);
        Ok(TwitterApiWithUserCtx {
            user_id,
            client: TwitterApi::new(self.auth().clone()),
        })
    }
}
