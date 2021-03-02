use chrono::NaiveDateTime;
use validator::Validate;
use serde::{Serialize, Deserialize};

#[doc(hidden)]
#[derive(Validate, Serialize, Debug, Default, PartialEq, Clone)]
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
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AccessToken {
    pub token: String,
    pub account_id: i64,
    pub created: NaiveDateTime,
    pub valid_thru: NaiveDateTime,
}