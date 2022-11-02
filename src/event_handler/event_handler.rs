use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::standard::{
        macros::{command, group},
        CommandResult, StandardFramework, CommandError, Args
    },
    model::{
        prelude::{Message, Reaction, ReactionType, Ready},
        application::{
            component::{SelectMenu, ComponentType, SelectMenuOption},
            interaction::{Interaction, InteractionResponseType}
        },
        id::GuildId,
    },
};

use std::env;
use log::{error, info, warn};

pub struct DiscordEventHandler;

#[async_trait]
impl EventHandler for DiscordEventHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.tag());

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        
    }

}