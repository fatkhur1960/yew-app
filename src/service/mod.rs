use serde::{Deserialize, Serialize};
use std::fmt;
use yew::format::{Json, Text};
use yew_services::{storage::Area, StorageService};

mod api_client;
mod auth_service;
mod account;
pub use self::{api_client::ApiClient, auth_service::AuthService, account::AccountService};

pub type Result<T> = std::result::Result<T, ApiError>;

#[derive(Debug)]
pub struct LocalStorage {
    storage: StorageService,
}

impl LocalStorage {
    pub fn new() -> Self {
        LocalStorage {
            storage: StorageService::new(Area::Local).expect("Storage disabled by user"),
        }
    }

    pub fn set_item(&mut self, key: &str, value: String) {
        self.storage.store(key, Json(&value));
    }

    pub fn get_item(&self, key: &str) -> Text {
        self.storage.restore(key)
    }

    pub fn remove_item(&mut self, key: &str) {
        self.storage.remove(key);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResult<T> {
    /// Error code untuk memberikan informasi hasil pengembalian,
    /// apabila tidak ada error terjadi maka code harus berisi 0.
    pub code: i32,

    /// Status bisa berisi: "success" atau "error".
    pub status: String,

    /// Deskripsi error apabila terjadi error.
    pub description: String,

    /// Result data.
    pub result: Option<T>,
}

#[derive(Debug)]
pub struct ApiError {
    pub msg: String,
}

impl ApiError {
    fn new(msg: &str) -> ApiError {
        ApiError {
            msg: msg.to_string(),
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl std::error::Error for ApiError {
    fn description(&self) -> &str {
        &self.msg
    }
}

impl From<reqwest_wasm::Error> for ApiError {
    fn from(err: reqwest_wasm::Error) -> Self {
        ApiError::new(&err.to_string())
    }
}
