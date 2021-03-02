use super::{Result, ApiError};
use chrono::NaiveDateTime;
use reqwest_wasm::{
    header::{self, HeaderMap, HeaderValue},
    Client, ClientBuilder, StatusCode,
};
use serde::Deserialize;
use yew::Callback;

#[derive(Debug, Deserialize)]
pub struct Repsitory {
    pub full_name: String,
    pub name: String,
    pub language: Option<String>,
    pub description: Option<String>,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub pushed_at: String,
}

#[derive(Clone)]
pub struct GithubService {
    pub client: Client,
    pub base_url: String,
    pub user: String,
}

impl GithubService {
    pub fn new() -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            header::ACCEPT,
            HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        headers.insert(header::USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10.16; rv:85.0) Gecko/20100101 Firefox/85.0").unwrap());
        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .unwrap();

        GithubService {
            base_url: String::from("https://api.github.com"),
            user: String::from("fatkhur1960"),
            client,
        }
    }

    pub async fn get_repos(&self, page: i32, per_page: i32) -> Result<Vec<Repsitory>> {
        let url = format!(
            "{}/users/{}/repos?per_page={}&page={}&sort=updated",
            self.base_url, self.user, per_page, page
        );

        match self.client.get(&url).send().await {
            Ok(res) => {
                if res.status() == StatusCode::OK {
                    match res.json::<Vec<Repsitory>>().await {
                        Ok(json) => Ok(json),
                        Err(e) => Err(ApiError::new(&e.to_string())),
                    }
                } else {
                    Err(ApiError::new(&format!(
                        "Response Code: {}",
                        res.status()
                    )))
                }
            }
            Err(e) => Err(ApiError::new(&e.to_string())),
        }
    }

    pub async fn get_repo(&self, name: String) -> Result<Repsitory> {
        let url = format!("{}/repos/{}/{}", self.base_url, self.user, name);
        let res = self.client.get(&url).send().await?;

        if res.status() == StatusCode::OK {
            let json = res.json::<Repsitory>().await?;
            Ok(json)
        } else {
            Err(ApiError::new(&format!("Get Code: {}", res.status())))
        }
    }
}
