
use std::{env, vec};
use std::{time::Duration, sync::Arc};

use rusqlite::{Result, params};
use tokio_rusqlite::Connection as Connection;

use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::prelude::*;
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework, CommandError, Args
};
use serenity::model::{
    prelude::{Message, Reaction, ReactionType, Ready},
    application::component::{SelectMenu, ComponentType, SelectMenuOption},
    application::interaction::InteractionResponseType,
};

mod user_info;
mod database_handler;
mod commands;
mod loa_contents;
mod event_handler;
mod command_handler;

use commands::*;


#[group]
#[commands(user_init, register, update, delete, query, homework, reset_weekly)]

struct General;

struct DBContainer;
impl TypeMapKey for DBContainer {
    type Value = Connection;
}

struct LoaContents;
impl TypeMapKey for LoaContents {
    type Value = loa_contents::LoaContents;
}

#[tokio::main]
async fn main() -> Result<()>{

    let conn = Connection::open("user.db").await?;
    database_handler::initialize(&conn).await?;

    const END_LV: u32 = 99999;
    let mut loacontents = loa_contents::LoaContents::new(vec![
        (1600, END_LV, 6500), (1580, 1600, 5500),
        (1560, END_LV, 3000), (1550, END_LV, 2000), (1540, END_LV, 5500),
        (1520, 1560, 2500), (1500, 1550, 1500), (1490, 1540, 4500),
        (1475, END_LV, 4500), (1460, END_LV, 4500), (1445, END_LV, 4500),
        (1430, 1460, 2500), (1415, 1445, 2500)
    ]);

    let framework = StandardFramework::new().configure(|c| c.prefix("~")).group(&GENERAL_GROUP);

    let token = std::env::var("DISCORD_TOKEN").unwrap();
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::MESSAGE_CONTENT
    ;

    let mut client = Client::builder(
        token, 
        intents
    )
    .event_handler(event_handler::event_handler::DiscordEventHandler)
    .framework(framework)
    .await
    .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<DBContainer>(conn);
        data.insert::<LoaContents>(loacontents);
    }
    
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}