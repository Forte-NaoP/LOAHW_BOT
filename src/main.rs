
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
mod gold_calc;

use commands::*;


#[group]
#[commands(user_init, register, update, delete, query)]

struct General;

struct DBContainer;

impl TypeMapKey for DBContainer {
    type Value = Connection;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() -> Result<()>{

    let conn = Connection::open("user.db").await?;

    database_handler::initialize(&conn).await?;

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
    .event_handler(Handler)
    .framework(framework)
    .await
    .expect("Error creating client");
    {
        let mut data = client.data.write().await;
        data.insert::<DBContainer>(conn);
    }
    
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }

    Ok(())
}