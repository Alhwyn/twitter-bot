use super::TwitterApi;
use crate::api_result::ApiResult;
use crate::authorization::Authorization;
use crate::data::{Tweet, User};
use crate::error::Result;
use crate::id::{IntoNumericId, NumericId};

pub struct TwitterApiWithUserCtx<A> {
    user_id: NumbericId,
    client: TwitterApi<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub async fn with_user_ctx(&self) -> Result<TwitterApiWithUserCtx<A>> {
        let user_id = self.get_users_me().send().await?.into_data().unwrap().id;
    }
}
