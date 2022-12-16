use std::{collections::HashMap, hash::Hash};

use regex::internal::Char;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateActionRow, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption, CreateSelectMenuOptions},
    client::{Context},
    framework::standard::{CommandResult, Args},
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{
            Message,
            interaction::{application_command::{CommandDataOption, CommandDataOptionValue}, self},
            command::CommandOptionType,
        },
        user::User,
    },
};

use crate::user_info::*;
use crate::loa_contents::*;

pub static EMPTY_STR: &str = "\u{200b}";

pub fn embeds_from_user_info(userinfo: &UserInfo) -> HashMap<String, CreateEmbed> {
    let mut pages: HashMap<String, CreateEmbed> = HashMap::new();

    for (_name, _charinfo) in userinfo.user_character().iter() {
        let mut embed = CreateEmbed::default();
        if _charinfo.lv >= 1302.0 {
            embed
                .title(format!("```{}```", _name))
                .description(format!("{} {}", _charinfo.class, _charinfo.lv))
                .fields(
                    create_fields_from_chardata(_charinfo)
                );
            pages.insert(_name.to_owned(), embed);
        }
    }
    pages
}

pub fn create_fields_from_chardata(_charinfo: &CharData) -> Vec<(String, &str, bool)> {
    let contents_len = CONTENTS_NAME.len();
    let mut content_list = vec![];

    for i in 0..contents_len {
        if (1 << i) & _charinfo.total_hw() != 0 {
            let mut s = String::new();
            if (1 << i) & _charinfo.done_hw() != 0 {
                s = format!("~~**{}**: **완료**~~", CONTENTS_NAME[i].2);
            } else {
                s = format!("**{}**: **가능**", CONTENTS_NAME[i].2);
            }
            content_list.push((CONTENTS_NAME[i].1, (s, EMPTY_STR, false)));
        }
    }
    content_list.sort_by_key(|a| -a.0);
    content_list.iter().map(|(a, b)| b.to_owned()).collect::<Vec<_>>()
}

pub fn character_select_menu<D: ToString + Clone>(v: &Vec<D>, selected: usize) -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();
    let mut options: Vec<CreateSelectMenuOption> = vec![];

    for (idx, val) in v.iter().enumerate() {
        let mut option = CreateSelectMenuOption::default();
        option.label(val.clone()).value(idx);
        if idx == selected {
            option.default_selection(true);
        }
        options.push(option);
    }

    menu.options(|f| {
        f.set_options(options)
    }).custom_id("character_select");

    menu
}

pub fn contents_select_menu(_char_name: &str, _char_info: &CharData) -> CreateSelectMenu {
    let mut menu = CreateSelectMenu::default();
    let mut options: Vec<CreateSelectMenuOption> = vec![];

    let contents_list = get_contents_list(_char_info);

    for (idx, contents, done) in contents_list.iter() {
        let mut option = CreateSelectMenuOption::default();
        option.label(contents).value(idx);
        if *done {
            option.default_selection(true);
        }
        options.push(option);
    }

    menu.options(|f| {
            f.set_options(options)
        })
        .custom_id(_char_name)
        .min_values(0)
        .max_values(contents_list.len() as u64);

    menu
}

pub fn get_contents_list(_char_info: &CharData) -> Vec<(i32, String, bool)> {
    let mut contents_list = vec![];
    let contents_len = CONTENTS_NAME.len();

    let total_hw = _char_info.total_hw();
    let done_hw = _char_info.done_hw();

    for i in 0..contents_len {
        if (1 << i) & total_hw != 0 {
            contents_list.push((CONTENTS_NAME[i].0, CONTENTS_NAME[i].1, CONTENTS_NAME[i].2.to_owned(), (1 << i) & done_hw != 0));
        }
    }
    contents_list.sort_by_key(|a| -a.1);
    contents_list.iter().map(|(a, b, c, d)| (a.to_owned(), c.to_owned(), d.to_owned())).collect::<Vec<_>>()
}