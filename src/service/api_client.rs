use super::{ApiError, ApiResult, Result};

use reqwest_wasm::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};

#[derive(Debug, Clone)]
pub struct ApiClient {
    client: Client,
    scope: Option<String>,
    headers: HeaderMap,
}

impl ApiClient {
    pub fn new() -> Self {
        ApiClient {
            scope: None,
            client: Self::build_client(),
            headers: Self::default_headers(),
        }
    }

    pub fn custom_headers(&self, headers: HeaderMap) -> Self {
        let client = ClientBuilder::new()
            .default_headers(headers.clone())
            .build()
            .expect("Cannot rebuild Client");

        ApiClient {
            client,
            scope: self.scope.to_owned(),
            headers,
        }
    }

    fn build_client() -> Client {
        ClientBuilder::new().build().expect("cannot build client")
    }

    pub fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/json"),
        );
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_static("application/json;utf-8"),
        );
        headers
    }

    fn add_header(&mut self, key: &'static str, val: HeaderValue) -> HeaderMap {
        self.headers.insert(key, val);
        self.headers.clone()
    }

    pub fn public(&mut self) -> Self {
        self.scope = Some(crate::PUBLIC_URL.to_string());
        self.clone()
    }

    pub fn private(&mut self) -> Self {
        self.scope = Some(crate::PRIVATE_URL.to_string());
        self.clone()
    }

    pub async fn get<T>(&self, path: &str) -> Result<ApiResult<T>>
    where
        T: DeserializeOwned,
    {
        let base_url = self
            .scope
            .as_ref()
            .map(|s| format!("{}{}", s.clone(), path))
            .unwrap_or(format!("{}", path));

        let res = self
            .client
            .get(&base_url)
            .headers(self.headers.clone())
            .send()
            .await?;

        if res.status() == StatusCode::OK {
            let json = res.json::<ApiResult<T>>().await?;
            Ok(json)
        } else {
            Err(ApiError::new(&format!("Get Error Code: {}", res.status())))
        }
    }

    pub async fn post<B, T>(&self, path: &str, body: &B) -> Result<ApiResult<T>>
    where
        T: DeserializeOwned,
        B: Serialize,
    {
        let base_url = self
            .scope
            .as_ref()
            .map(|s| format!("{}{}", s.clone(), path))
            .unwrap_or(format!("{}{}", crate::PUBLIC_URL.to_string(), path));

        let res = self
            .client
            .post(&base_url)
            .headers(self.headers.clone())
            .json(body)
            .send()
            .await?;

        if res.status() == StatusCode::OK {
            let json = res.json::<ApiResult<T>>().await?;
            Ok(json)
        } else {
            Err(ApiError::new(&format!("Get Error Code: {}", res.status())))
        }
    }
}
