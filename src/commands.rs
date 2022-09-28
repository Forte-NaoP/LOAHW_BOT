use std::fmt::format;

use crate::{Context, Error};
use poise::serenity_prelude as serenity;
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

    println!("{:?}", msg);

    let parsed_msg = parse_msg(&str);
    let mut response = String::new();

    for parsed in parsed_msg.iter() {
        let s = format!("name:{}, class: {}, lv: {}\n", parsed[0], parsed[1], parsed[2]);
        println!("{}", s);
        response.push_str(s.as_str());
    }

    if response.is_empty() {
        ctx.say("Parsing Failed").await?;
    } else {
        ctx.say(response).await?;
    }

    Ok(())
}

fn parse_msg(msg: &String) -> Vec<Vec<&str>> {

    let re = Regex::new(r"(\w+?\s+?\w+?\s+?\d+)").unwrap();
    let parsed_msg: Vec<Vec<&str>> = re.find_iter(msg).map(|mat| {
        mat.as_str().split(" ").collect::<Vec<&str>>()
    }).collect();

    parsed_msg
}