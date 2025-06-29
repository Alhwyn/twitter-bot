#[cfg(feature = "oauth2")]
pub extern crate oauth2;

pub mod api;
pub mod api_result;
pub mod authorization;
pub mod data;
pub mod error;
pub mod id;
pub mod meta;
pub mod query;
pub mod requests;
mod utils;

pub use self::{
    api::{TwitterApi, TwitterApiWithUserCtx},
    api_result::{ApiError, ApiPayload, ApiResponse, ApiResult},
    authorization::Authorization,
    data::{Tweet, User},
    error::{Error, Result},
};

pub mod prelude {
    pub use crate::api_result::PaginableApiResponse;
    pub use crate::authorization::Authorization;
    pub use crate::id::{IntoNumericId, IntoStringId};
    pub use crate::meta::PaginationMeta;
}
