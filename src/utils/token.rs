use std::sync::RwLock;
use yew_services::{storage::Area, StorageService};
use super::bindings::encode_uri;

const TOKEN_KEY: &str = "access_token";

lazy_static! {
    /// Jwt token read from local storage.
    pub static ref TOKEN: RwLock<Option<String>> = {
        let storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
        if let Ok(token) = storage.restore(TOKEN_KEY) {
            RwLock::new(Some(token))
        } else {
            RwLock::new(None)
        }
    };
}

/// Set access token to local storage.
pub fn set_token(token: Option<String>) {
    let mut storage = StorageService::new(Area::Local).expect("storage was disabled by the user");
    if let Some(t) = token.clone() {
        storage.store(TOKEN_KEY, Ok(t));
    } else {
        storage.remove(TOKEN_KEY);
    }
    let mut token_lock = TOKEN.write().expect("cannot write token");
    *token_lock = token;
}

/// Get access token from lazy static.
pub fn get_token() -> Option<String> {
    let token_lock = TOKEN.read().expect("cannot read token");
    token_lock.clone()
}

/// Check if current user is authenticated.
pub fn is_authenticated() -> bool {
    get_token().is_some()
}

/// Check if current user is authenticated
/// and then redirect to target if not
pub fn auth_middleware(auth: bool, redirect: &str, next: Option<String>) {
    if auth && !is_authenticated() {
        let target = next
            .map(|t| {
                let encoded = encode_uri(&t)
                    .as_string()
                    .unwrap_or(String::new());
                format!("{}?next={}", redirect, encoded)
            })
            .unwrap_or(redirect.to_string());

        redirect!(&target);
    }
}
