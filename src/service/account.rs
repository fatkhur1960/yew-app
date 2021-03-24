use reqwest_wasm::header::{HeaderMap, HeaderValue};

use crate::{models::Account, utils};

use super::{ApiClient, ApiResult, Result};

#[derive(Debug, Clone)]
pub struct AccountService {
    client: ApiClient,
    base_endpoint: String,
}

impl AccountService {
    pub fn new() -> Self {
        let client = ApiClient::new().custom_headers(Self::init_headers());

        AccountService {
            client,
            base_endpoint: String::from("/account/v1"),
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

    pub async fn get_profile(&mut self) -> Result<ApiResult<Account>> {
        self.client
            .public()
            .get::<Account>(&self.endp("/me/info"))
            .await
    }
}
