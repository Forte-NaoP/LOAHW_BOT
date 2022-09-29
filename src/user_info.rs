use serde_json::{self, json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type CharInfo = HashMap<String, (String, u32, u32)>;
#[derive(Serialize, Deserialize, Debug)]
pub struct UserInfo {
    user_name: String,
    user_character: CharInfo,
}

impl UserInfo {
    pub fn new(user_name: String, user_character: CharInfo) -> UserInfo {
        UserInfo { user_name, user_character }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn user_character(&self) -> &CharInfo {
        &self.user_character
    }

    pub fn user_character_mut(&mut self) -> &mut CharInfo {
        &mut self.user_character
    }

    pub fn to_json(&self) -> String {
        json!({
            "user_name": self.user_name,
            "user_character": self.user_character,
        }).to_string()
    }
}