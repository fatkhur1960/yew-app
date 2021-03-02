mod field;
mod form;
mod form_state;
mod button;
pub use field::Field;
pub use form::Form;
pub use button::Button;

use validator::Validate;
use std::{collections::HashMap, str::FromStr};

pub(crate) struct FormField {
    pub field_name: String,
    pub field_value: String,
    pub message: String,
    pub dirty: bool,
    pub valid: bool,
}

impl FormField {
    pub fn new(field_name: &str, field_value: &str) -> Self {
        FormField {
            field_name: String::from(field_name),
            field_value: String::from(field_value),
            message: String::new(),
            dirty: false,
            valid: true,
        }
    }
}

pub trait FormValue {
    fn fields(&self, prefix: &str, fields: &mut Vec<String>) {
        // By default, announce the value to be a scalar
        fields.push(String::from(prefix));
    }
    fn value(&self, field_path: &str) -> String;
    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String>;
}

pub trait Model: FormValue + Default + Validate + PartialEq + Clone + 'static {
    fn new() -> Self;
}

pub fn split_field_path(field_path: &str) -> (&str, &str) {
    if let Some(index) = field_path.find(".") {
        (&field_path[0..index], &field_path[index+1..])
    } else {
        (field_path, "")
    }
}

impl<T: ToString + FromStr> FormValue for T {
    fn value(&self, field_path: &str) -> String {
        debug_assert!(field_path == "");

        self.to_string()
    }

    fn set_value(&mut self, field_path: &str, value: &str) -> Result<(), String> {
        debug_assert!(field_path == "");

        if let Ok(v) = value.parse::<T>() {
            *self = v;
            Ok(())
        } else {
            Err(String::from("Could not convert"))
        }
    }
}