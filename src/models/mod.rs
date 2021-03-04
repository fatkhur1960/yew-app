use serde::{Deserialize, Serialize};
use validator::Validate;

#[doc(hidden)]
#[derive(Serialize, Debug, Clone)]
pub struct IdPayload {
    id: i64,
}

#[doc(hidden)]
#[derive(Model, Validate, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Login {
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    #[validate(length(min = 6, message = "Password is required"))]
    pub password: String,
}

#[doc(hidden)]
#[derive(Model, Validate, Serialize, Debug, Default, PartialEq, Clone)]
pub struct Register {
    #[validate(length(min = 3, max = 30, message = "Full Name is required"))]
    pub full_name: String,
    #[validate(length(min = 6, max = 15, message = "Nick Name is required"))]
    pub nickname: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
    pub phone_num: String,
    #[validate(length(min = 6, message = "Password is required"))]
    pub password: String,
    #[validate(must_match(other = "password", message = "Password not match"))]
    pub confirm_password: String,
}

#[doc(hidden)]
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub account_id: i64,
    pub created: String,
    pub valid_thru: String,
}
