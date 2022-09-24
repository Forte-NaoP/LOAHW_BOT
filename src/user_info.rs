use serde_json::{self, json};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    user_name: String,
    user_character: HashMap<String, u32>,
}

#[derive(Serialize, Deserialize)]
pub struct CharInfo {
    char_name: String,
    char_lv: u32,
}

impl UserInfo {
    pub fn new(user_name: String, user_character: Vec<CharInfo>) -> UserInfo {
        let user_character:HashMap<String, u32> = user_character.iter().map(|character| {
            (character.char_name.to_string(), character.char_lv)
        }).collect();
        UserInfo { user_name, user_character }
    }

    pub fn user_name(&self) -> &str {
        &self.user_name
    }

    pub fn to_json(&self) -> String {
        let char_info_slice:Vec<String> = self.user_character.iter().map(
            |char_info| {
                format!("{}:{}", char_info.0, char_info.1)
            }
        ).collect();
        
        json!({
            "user_name": self.user_name,
            "user_character": char_info_slice,
        }).to_string()
    }
}

impl CharInfo {
    pub fn new(char_name: String, char_lv: u32) -> CharInfo {
        CharInfo { char_name, char_lv }
    }
    pub fn char_name(&self) -> &str {
        &self.char_name
    }

    pub fn char_name_mut(&mut self) -> &mut String {
        &mut self.char_name
    }

    pub fn char_lv(&self) -> u32 {
        self.char_lv
    }
    
    pub fn set_char_lv(&mut self, lv: u32) {
        self.char_lv = lv;
    }
}