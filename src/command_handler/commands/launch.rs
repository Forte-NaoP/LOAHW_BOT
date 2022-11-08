use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::{Context},
    framework::standard::{
        CommandResult, Args,
    },
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        id::GuildId,
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
        },
        user::User,
    },
};

use log::error;

use crate::{
    database_handler, DBContainer, LoaContents, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
        command_return::CommandReturn,
    }
};


pub fn register(
    command: &mut CreateApplicationCommand
) -> &mut CreateApplicationCommand {
    command
        .name("launch")
        .description("서버에 명령어 등록")
}

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
) {
    command.defer(&ctx.http).await.unwrap();

    let guild_id = command.guild_id.unwrap();
    COMMAND_LIST.register(guild_id, ctx).await;
    
    if let Err(why) = command
        .edit_original_interaction_response(&ctx.http, |msg| msg.content("등록 완료"))
        .await
    {
        println!("{:#?}", why);
    }
}
