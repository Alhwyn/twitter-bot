#[cfg(feature = "oauth2")]
pub extern crate oauth2;

pub mod api;
pub mod api_result;
pub mod auth;
pub mod data;
pub mod error;
pub mod id;
pub mod requests;
pub mod utils;

pub use self::{
    api::{TwitterApi, TwitterApiWithUserCtx},
    api_result::{ApiError, ApiPayload, ApiResponse, ApiResult},
    auth::Authorization,
    data::{TweetPublicMetrics, User, UserPublicMetrics, Withheld, WithheldScope},
    error::{Error, Result},
};

pub mod prelude {
    pub use crate::auth::Authorization;
    pub use crate::id::{IntoNumericId, IntoStringId};
}
