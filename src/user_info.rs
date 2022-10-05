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
    lv: u32,
    tot_hw: u32,
    done_hw: u32,
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
            result.push_str(format!("name: {}, class: {}, lv: {}, income: {}\n", 
                name, 
                charinfo.class(),
                charinfo.lv(),
                charinfo.tot_hw()
            ).as_str());
        }
        result
    }
}

impl CharData {
    pub fn new(class: String, lv: u32) -> CharData {
        CharData { 
            class, lv, 
            tot_hw: 0,
            done_hw: 0,
        }
    }
    pub fn class(&self) -> &str {
        &self.class
    }

    pub fn lv(&self) -> u32 {
        self.lv
    }

    pub fn tot_hw(&self) -> u32 {
        self.tot_hw
    }

    pub fn done_hw(&self) -> u32 {
        self.done_hw
    }

    pub fn set_done_hw(&mut self, hw: u32) {
        self.done_hw |= hw;
    }
}
