use std::{fmt::format, collections::HashMap};

use crate::{Context, Error, database_handler, user_info::{UserInfo, CharInfo, self}};
use poise::serenity_prelude::{self as serenity, User};
use regex::Regex;

#[poise::command(prefix_command, owners_only, hide_in_help)]
pub async fn shutdown(ctx: Context<'_>) -> Result<(), Error> {
    ctx.framework()
        .shard_manager()
        .lock()
        .await
        .shutdown_all()
        .await;
    Ok(())
}

/// ~등록 character1 class lv  [character2 class lv] [--all]
#[poise::command(prefix_command)]
pub async fn 등록(
    ctx: Context<'_>,
    msg: Vec<String>,
) -> Result<(), Error> {
    
    let str = msg.join(" ");
    let user_info: UserInfo = user_info_from_msg(&ctx.author().name, &str);

    let mut response = String::new();
    for (k, v) in user_info.user_character().iter() {
        response += &format!("name: {}, class: {}, lv: {}\n", k, v.0, v.1).to_string();
    }

    if response.is_empty() {
        ctx.say("Parsing Failed").await?;
    } else {
        ctx.say(response).await?;
    }

    database_handler::user_register(&ctx.data().conn, user_info).await?;

    Ok(())
}

fn user_info_from_msg(name: &String, msg: &String) -> UserInfo {

    let re = Regex::new(r"((?P<name>\w+?)\s+?(?P<class>\w+?)\s+?(?P<lv>\d+))").unwrap();
    let mut charinfo: user_info::CharInfo = HashMap::new();

    for caps in re.captures_iter(msg) {
        charinfo.insert(caps["name"].to_string(), (caps["class"].to_string(), caps["lv"].parse::<u32>().unwrap(), 0));
    }

    UserInfo::new(name.to_string(), charinfo)
}