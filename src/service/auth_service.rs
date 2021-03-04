use reqwest_wasm::header::{HeaderMap, HeaderValue};

use crate::{
    models::{AccessToken, IdPayload, Login},
    utils,
};

use super::{ApiClient, ApiResult, Result};

#[derive(Debug)]
pub struct AuthService {
    client: ApiClient,
    base_endpoint: String,
}

impl AuthService {
    pub fn new() -> Self {
        let client = ApiClient::new().custom_headers(Self::init_headers());

        AuthService {
            client,
            base_endpoint: String::from("/auth/v1"),
        }
    }

    fn init_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        let token = utils::token::get_token().unwrap_or(String::new());
        headers.insert(
            "X-Access-Token",
            HeaderValue::from_str(&token).expect("Cannot set X-Access-Token"),
        );

        headers
    }

    fn endp(&self, path: &str) -> String {
        format!("{}{}", self.base_endpoint, path)
    }

    pub async fn authorize(&mut self, body: &Login) -> Result<ApiResult<AccessToken>> {
        self.client
            .public()
            .post::<Login, AccessToken>(&self.endp("/authorize"), body)
            .await
    }

    pub async fn unauthorize(&mut self, body: &IdPayload) -> Result<ApiResult<()>> {
        self.client
            .public()
            .post::<IdPayload, ()>(&self.endp("/unauthorize"), body)
            .await
    }
}
