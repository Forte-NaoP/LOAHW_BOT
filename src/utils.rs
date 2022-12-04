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

pub fn select_menu_from_vec<D: ToString + Clone>(v: &Vec<D>, selected: usize) -> CreateSelectMenu {
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
    }).custom_id("character_list");

    menu
}

pub fn get_content_list(_charinfo: &CharData) -> Vec<(String, &str, bool)> {
    let contents_len = CONTENTS_NAME.len();
    let mut content_list = vec![];

    for i in (0..contents_len) {
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
