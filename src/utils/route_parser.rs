use serde::de::{self, Deserialize, Deserializer};
use std::{fmt::Display, str::FromStr};
use crate::JsonValue;

use yew_router::matcher::RouteMatcher;
use yew_router_route_parser::{CaptureVariant, MatcherToken};

#[derive(Debug)]
struct RouteToken(String);

impl From<Vec<MatcherToken>> for RouteToken {
    fn from(tokens: Vec<MatcherToken>) -> Self {
        let token: Vec<String> = tokens
            .into_iter()
            .map(|t| match t {
                MatcherToken::Exact(value) => value,
                MatcherToken::Capture(cap) => match cap {
                    CaptureVariant::Unnamed => String::from("{}"),
                    CaptureVariant::ManyUnnamed => String::from("{*}"),
                    CaptureVariant::NumberedUnnamed { sections } => format!("{{{}}}", sections),
                    CaptureVariant::Named(name) => format!("{{{}}}", name),
                    CaptureVariant::ManyNamed(name) => format!("{{*:{}}}", name),
                    CaptureVariant::NumberedNamed { sections, name } => {
                        format!("{{{}:{}}}", sections, name)
                    }
                },
                MatcherToken::End => String::new(),
            })
            .collect();

        Self(token.join(""))
    }
}

impl ToString for RouteToken {
    fn to_string(&self) -> String {
        self.0.clone()
    }
}

pub fn from_str<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromStr,
    T::Err: Display,
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    T::from_str(&s).map_err(de::Error::custom)
}

pub fn parse(path: &str, routes: Vec<RouteMatcher>) -> Option<(String, JsonValue)> {
    routes.iter().find_map(|m| {
        let matcher = m.clone();
        if let Ok(r) = m.capture_route_into_map(path) {
            let token = RouteToken::from(matcher.tokens);
            let result = (token.to_string(), serde_json::to_value(r.1).unwrap());
            return Some(result);
        }
        return None;
    })
}