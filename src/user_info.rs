use serde_json::{self, json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    user_name: String,
    user_character: HashMap<String, u32>,
}

impl UserInfo {
    pub fn new(user_name: String, user_character: HashMap<String, u32>) -> UserInfo {
        UserInfo { user_name, user_character }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn to_json(&self) -> String {
        json!({
            "user_name": self.user_name,
            "user_character": self.user_character,
        }).to_string()
    }
}