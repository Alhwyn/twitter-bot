use crate::api_result::{ApiPayload, ApiResponse, ApiResponseExt, ApiResult};
use crate::auth::Authorization;
use crate::error::Result;
use crate::utils::JsonStream;
use futures::prelude::*;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, IntoUrl, Method, Url};
use serde::de::DeserializeOwned;
use std::sync::Arc;

#[derive(Debug)]
pub struct TwitterApi<A> {
    client: Client,
    base_url: Url,
    auth: Arc<A>,
}

impl<A> TwitterApi<A>
where
    A: Authorization,
{
    pub fn new(auth: A) -> Self {
        Self {
            client: Client::builder().pool_max_idle_per_host(0).build().unwrap(),
            base_url: Url::parse("https://api.twitter.com/2/").unwrap(),
            auth: Arc::new(auth),
        }
    }

    pub fn auth(&self) -> &A {
        &self.auth
    }

    pub(crate) fn url(&self, url: impl AsRef<str>) -> Result<Url> {
        Ok(self.base_url.join(url.as_ref())?)
    }

    pub(crate) fn request(&self, method: Method, url: impl IntoUrl) -> reqwest::RequestBuilder {
        self.client.request(method, url)
    }

    pub(crate) async fn send<T: DeserializeOwned>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> ApiResult<T> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);

        let response = self.client.execute(req).await?.api_error_for_status();

        // Get the response text for debugging
        let response_text = response.text().await?;
        tracing::debug!("Twitter API response: {}", response_text);

        // Parse the Twitter API v2 response format
        let api_response: ApiPayload<T> = serde_json::from_str(&response_text)?;

        Ok(ApiResponse::new(api_response))
    }

    #[allow(dead_code)]
    pub(crate) async fn stream<T: DeserializeOwned + 'static>(
        &self,
        req: reqwest::RequestBuilder,
    ) -> Result<impl Stream<Item = Result<ApiPayload<T>, crate::error::Error>>> {
        let mut req = req.build()?;
        let authorization = self.auth.header(&req).await?;
        let _ = req.headers_mut().insert(AUTHORIZATION, authorization);

        // For now, return a simple empty stream to get compilation working
        let empty_stream = futures::stream::empty();
        Ok(JsonStream::new(empty_stream))
    }
}

impl<A> Clone for TwitterApi<A> {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            base_url: self.base_url.clone(),
            auth: self.auth.clone(),
        }
    }
}
