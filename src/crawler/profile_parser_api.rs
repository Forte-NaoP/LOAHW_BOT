use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use lazy_static::lazy_static;
use reqwest::header::{HeaderMap, self};
use futures::{join, future::{join_all, FutureExt}};
use url::form_urlencoded::{byte_serialize, parse};

use crate::{
    user_info::*,
    loa_contents::LOA_CONTENTS,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CharacterJSON {
    pub ServerName: String,
    pub CharacterName: String,
    pub CharacterLevel: u64,
    pub CharacterClassName: String,
    pub ItemAvgLevel: String,
    pub ItemMaxLevel: String
}

static USER_AGENT: &str = "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36";

pub async fn get_character_list(name: String) -> Option<Vec<(String, CharacterData)>> {
    
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();

    let key = std::env::var("LOA_TOKEN").unwrap();

    let mut headers = HeaderMap::new();
    headers.insert(header::ACCEPT, "application/json".parse().unwrap());
    headers.insert(header::USER_AGENT, USER_AGENT.parse().unwrap());
    headers.insert(header::AUTHORIZATION, format!("bearer {}", key).parse().unwrap());

    let res = client
        .get(format!("https://developer-lostark.game.onstove.com/characters/{}/siblings", name))
        .headers(headers)
        .send()
        .await
        .unwrap();

    match res.json::<Vec<CharacterJSON>>().await {
        Ok(result) => {
            let mut characters = vec![];
            
            for character in result.iter() {
                let lv = character.ItemMaxLevel.replace(",", "").parse::<f64>().unwrap();
                characters.push((
                    character.CharacterName.to_owned(),
                    CharacterData::from(
                        character.CharacterClassName.to_owned(),
                        lv,
                        LOA_CONTENTS.get_hw(lv)
                    )
                ))
            }

            Some(characters)
        },
        Err(e) => None,
    }


}