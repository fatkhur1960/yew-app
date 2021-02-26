extern crate regex;
use regex::Regex;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::JsonValue;

#[derive(Deserialize, Serialize)]
#[serde(untagged)]
enum ArgValue {
    Int(i32),
    Str(String),
}

pub fn parse_get_key(keys: Vec<String>, path: String) -> Option<(String, JsonValue)> {
    keys.into_iter().find_map(|raw| {
        let m = parse(&raw, &path);
        if m.is_some() {
            Some((raw, m.unwrap()))
        } else {
            None
        }
    })
}

pub fn parse(pattern: &str, input: &str) -> Option<JsonValue> {
    let mut args: Vec<(String, String)> = Vec::new();
    let mut keys: Vec<(String, String)> = Vec::new();
    let mut result: HashMap<String, ArgValue> = HashMap::new();
    
    let mut input_pat = pattern.to_string();
    
    let re = Regex::new(r"<(?P<arg>\w+):(?P<type>\w+)>").unwrap();
    for cap in re.captures_iter(&input_pat) {
        let key = cap.get(0).unwrap().as_str().to_string();
        let arg = cap.name("arg").unwrap().as_str();
        let raw_ty = cap.name("type").unwrap().as_str();
        
        let ty = match raw_ty {
            "int" => "[0-9]",
            "string" => "[0-9A-Za-z_-]",
            _ => "[0-9A-Za-z_-]"
        };
        
        let input_pat = format!("(?P<{}>{}+)", arg, ty);
        args.push((key, input_pat));
        keys.push((arg.to_string(), raw_ty.to_string()));
    }
    
    if args.is_empty() || keys.is_empty() {
        return None;
    }
    
    for arg in args.iter() {
        input_pat = input_pat.replace(&arg.0, &arg.1);
    }
    
    let re2 = Regex::new(&input_pat).unwrap();
    let caps = re2.captures(input);
    
    if caps.is_some() {
        let cap = caps.unwrap();
        for key in &keys {
            let raw_val = cap.name(&key.0).unwrap().as_str();
            let mut val = ArgValue::Str(raw_val.to_string());
            
            if key.1 == "int" {
                match raw_val.parse::<i32>() {
                    Ok(i) => val = ArgValue::Int(i),
                    Err(_) => return None,
                }
            }
            
            result.insert(key.0.to_string(), val);
        }
    }
    
    if !result.is_empty() {
        serde_json::to_value(&result).ok()
    } else {
        None
    }
}