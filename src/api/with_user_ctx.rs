use super::TwitterApi;
use crate::auth::Authorization;
use crate::error::Result;
use crate::id::NumericId;

pub struct TwitterApiWithUserCtx<A> {
    #[allow(dead_code)]
    user_id: NumericId,
    #[allow(dead_code)]
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
