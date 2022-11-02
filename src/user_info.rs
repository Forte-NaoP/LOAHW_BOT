use serde_json::{self, json};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Result};

pub type CharInfo = HashMap<String, CharData>;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserInfo {
    user_name: String,
    user_character: CharInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharData {
    class: String,
    lv: f64,
    total_hw: u64,
    done_hw: u64,
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

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        for (name, charinfo) in self.user_character().iter() {
            result.push_str(format!("name: {}, class: {}, lv: {}", 
                name, 
                charinfo.class(),
                charinfo.lv(),
            ).as_str());
        }
        result
    }
}

impl CharData {
    pub fn from(class: String, lv: f64, total_hw: u64) -> CharData {
        CharData { 
            class, lv, total_hw,
            done_hw: 0,
        }
    }

    pub fn new() -> CharData {
        CharData {
            class: String::from(""), 
            lv: 0.0, 
            total_hw: 0,
            done_hw: 0,
        }
    }

    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn set_class(&mut self, class: String) {
        self.class = class;
    }

    pub fn lv(&self) -> f64 {
        self.lv
    }

    pub fn set_lv(&mut self, lv: f64) {
        self.lv = lv;
    }

    pub fn total_hw(&self) -> u64 {
        self.total_hw
    }

    pub fn done_hw(&self) -> u64 {
        self.done_hw
    }

    pub fn set_done_hw(&mut self, hw: u64) {
        self.done_hw |= hw;
    }

    pub fn reset_done_hw(&mut self) {
        self.done_hw = 0;
    }
}
