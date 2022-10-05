use std::collections::HashMap;
use crate::{database_handler, user_info::*, DBContainer};
use regex::Regex;

use rusqlite::{Result, params};
use tokio_rusqlite::Connection as Connection;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::prelude::{GatewayIntents, ClientError};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework, CommandError, Args
};
use serenity::model::{
    prelude::{Message, Reaction, ReactionType, Ready},
    application::component::{SelectMenu, ComponentType, SelectMenuOption},
    application::interaction::InteractionResponseType,
};


#[command]
#[aliases("초기화")]
pub async fn user_init(ctx: &Context, msg: &Message) -> CommandResult {
    database_handler::user_delete(ctx.data.write().await.get_mut::<DBContainer>().unwrap(), msg.author.tag()).await?;
    msg.channel_id.say(&ctx.http, format!("reset data of {}", msg.author.tag())).await?;
    Ok(())
}

// ~등록 character1 class lv  [character2 class lv] [--all]
#[command]
#[aliases("등록")]
pub async fn register(
    ctx: &Context,
    msg: &Message,
    #[delimiters(Delimiter::Single(' '))] 
    args: Args
) -> CommandResult {

    let str = args.raw().collect::<Vec<&str>>().join(" ");
    let user_info = user_info_from_msg(&msg.author.tag(), &str);

    let response = build_response(user_info.user_character());

    if response.is_empty() {
        msg.channel_id.say(&ctx.http, "Parsing Failed").await?;
    } else {
        let result = database_handler::user_register(&ctx.data.write().await.get_mut::<DBContainer>().unwrap(), user_info).await;
        match result {
            Ok(()) => {
                msg.channel_id.say(&ctx.http, response).await?;
            },
            Err(why) => {
                msg.channel_id.say(&ctx.http, "Register Failed").await?;
            }
        }
    }

    Ok(())
}

#[command]
#[aliases("수정", "갱신", "추가")]
pub async fn update(
    ctx: &Context,
    msg: &Message,
    #[delimiters(Delimiter::Single(' '))] 
    args: Args
) -> CommandResult {

    let str = args.raw().collect::<Vec<&str>>().join(" ");
    let user_info = user_info_from_msg(&msg.author.tag(), &str);

    let response = build_response(user_info.user_character());

    if response.is_empty() {
        msg.channel_id.say(&ctx.http, "Parsing Failed").await?;
    } else {
        let result = database_handler::user_update(&ctx.data.write().await.get_mut::<DBContainer>().unwrap(), user_info).await;
        match result {
            Ok(()) => {
                msg.channel_id.say(&ctx.http, response).await?;
            },
            Err(why) => {
                msg.channel_id.say(&ctx.http, "Update Failed").await?;
            }
        }
    }
    Ok(())
}

#[command]
#[aliases("삭제")]
pub async fn delete(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let name = args.raw().collect::<Vec<&str>>()[0];
    let result = database_handler::character_delete(
        &ctx.data.write().await.get_mut::<DBContainer>().unwrap(), 
        msg.author.tag().to_string(), 
        name.to_string()
    ).await;
    match result {
        Ok(()) => {
            msg.channel_id.say(&ctx.http, format!("delete character {} of {}", name, &msg.author.tag())).await?;
        },
        Err(why) => {
            msg.channel_id.say(&ctx.http, "Delete Failed").await?;
        }
    }
    Ok(())
}

#[command]
#[aliases("조회")]
pub async fn query(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let result = database_handler::user_query(
        &ctx.data.write().await.get_mut::<DBContainer>().unwrap(),
        msg.author.tag().to_string(),
    ).await?;

    msg.channel_id.say(&ctx.http, format!("{}\n{}", &msg.author.tag(), result)).await?;

    Ok(())
}

fn user_info_from_msg(name: &String, msg: &String) -> UserInfo {

    let re = Regex::new(r"((?P<name>\w+?)\s+?(?P<class>\w+?)\s+?(?P<lv>\d+))").unwrap();
    let mut charinfo: CharInfo = HashMap::new();

    for caps in re.captures_iter(msg) {
        charinfo.insert(caps["name"].to_string(),
        CharData::new(
            caps["class"].to_string(),
            caps["lv"].parse::<u32>().unwrap(),
        ));
    }

    UserInfo::new(name.to_string(), charinfo)
}

fn build_response(charinfo: &CharInfo) -> String {
    let mut response = String::new();
    for (k, v) in charinfo.iter() {
        response += &format!("name: {}, class: {}, lv: {}\n", k, v.class(), v.lv()).to_string();
    }
    response
}