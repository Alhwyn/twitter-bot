use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum ApiError {
    #[error("API Error: {message}")]
    General { message: String },
    #[error("Rate limit exceeded")]
    RateLimit,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiPayload<T> {
    pub data: Option<T>,
    pub meta: Option<serde_json::Value>,
    pub errors: Option<Vec<ApiError>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub payload: ApiPayload<T>,
}

impl<T> ApiResponse<T> {
    pub fn new(payload: ApiPayload<T>) -> Self {
        Self { payload }
    }
}

pub type ApiResult<T> = Result<ApiResponse<T>, crate::error::Error>;

pub trait ApiResponseExt {
    fn api_error_for_status(self) -> Self;
}

impl ApiResponseExt for reqwest::Response {
    fn api_error_for_status(self) -> Self {
        // For now, just return the response as is
        // This should be implemented to check for API errors
        self
    }
}

pub trait PaginableApiResponse<T> {
    fn into_data(self) -> Option<T>;
}
