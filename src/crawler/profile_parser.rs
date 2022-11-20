use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use scraper::{Html, Selector};
use lazy_static::lazy_static;
use reqwest::header;
use futures::{join, future::{join_all, FutureExt}};
use url::form_urlencoded::{byte_serialize, parse};

use crate::{
    user_info::*,
    loa_contents::LOA_CONTENTS,
};

pub struct CssSelector {
    pub selector: HashMap<&'static str, Selector>,
}

lazy_static! {
    pub static ref CSS_SELECTOR: CssSelector = CssSelector {
        selector: HashMap::from([
            ("not_exist_selector", Selector::parse(".profile-attention").unwrap()),
            ("profile", Selector::parse("ul.profile-character-list__char").unwrap()),
            ("character", Selector::parse("li > span > button").unwrap()),
            ("class", Selector::parse("img.profile-character-info__img").unwrap()),
            ("item_lv", Selector::parse("div.level-info2__item").unwrap()),
            ("nickname", Selector::parse("span.profile-character-info__name").unwrap()),
        ])
    };
}

static BASE_URL: &str = "https://lostark.game.onstove.com/Profile/Character/";
static USER_AGENT: &str = "User-Agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/89.0.4389.114 Safari/537.36";

pub async fn get_character_list(name: String) -> Option<Vec<(String, CharData)>> {

    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    
    let url_encoded: String = byte_serialize(name.as_bytes()).collect();
    
    let res = client
        .get(BASE_URL.to_owned()+url_encoded.as_str())
        .header(header::USER_AGENT, USER_AGENT)
        .send()
        .await
        .unwrap();
    println!("{}", BASE_URL.to_owned()+url_encoded.as_str());
    let body = res.text().await.unwrap();
    
    let mut info_fut = vec![];
    // let mut result: Vec<(String, CharData)> = vec![];
    {
        let page = Html::parse_document(&body);

        let not_exist_selector = CSS_SELECTOR.selector.get("not_exist_selector").unwrap();
        
        if page.select(not_exist_selector).count() != 0 {
            return None;
        }

        let profile_selector = CSS_SELECTOR.selector.get("profile").unwrap();
        let character_selector = CSS_SELECTOR.selector.get("character").unwrap();

        for ul in page.select(profile_selector) {
            for character in ul.select(character_selector) {
                let url = character.value().attr("onclick").unwrap().rsplit_once('/').unwrap().1.replace("\'", "");
                info_fut.push(get_character_info(BASE_URL.to_owned()+url.as_str()));
            }
        }
    }

    Some(join_all(info_fut).await)

}

async fn get_character_info(url: String) -> (String, CharData) {
    let client = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let res = client.get(url).header(header::USER_AGENT, USER_AGENT).send().await.unwrap();
    let body = res.text().await.unwrap();
    let page = Html::parse_document(&body);

    let (name, class, lv) = (get_name(&page), get_class(&page), get_item_lv(&page));

    (name, CharData::from(class, lv, LOA_CONTENTS.get_hw(&lv)))
}

fn get_name(page: &Html) -> String {
    let selector = CSS_SELECTOR.selector.get("nickname").unwrap();
    let page_element = page.select(&selector).next().unwrap();
    page_element.value().attr("title").unwrap().to_owned()
}

fn get_item_lv(page: &Html) -> f64 {
    let selector = CSS_SELECTOR.selector.get("item_lv").unwrap();
    let page_element = page.select(&selector).next().unwrap();
    page_element.text().collect::<Vec<_>>()[2..].join("").replace(",", "").parse::<f64>().unwrap()
}

fn get_class(page: &Html) -> String {
    let selector = CSS_SELECTOR.selector.get("class").unwrap();
    let page_element = page.select(&selector).next().unwrap();
    page_element.value().attr("alt").unwrap().to_owned()
}