use std::collections::HashMap;

use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::{Context},
    framework::standard::{
        CommandResult, Args,
    },
    model::{
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
        },
        user::User,
    },
};

use crate::{
    database_handler, DBContainer, LoaContents, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
    }
};

struct CharacterDelete;

#[async_trait]
impl CommandInterface for CharacterDelete {
    async fn run(&self, ctx: &Context, msg: &Message, options: &[CommandDataOption]) -> CommandResult {
        
        let nickname = Option::<String>::from(DataWrapper::from(options, 0)).unwrap();
        database_handler::character_delete(ctx.data.read().await.get::<DBContainer>().unwrap(), msg.author.tag(), nickname).await.unwrap();
        // 보유 캐릭터 현황 보여주기
        msg.channel_id.say(&ctx.http, format!("뿅")).await.unwrap();
        Ok(())
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("삭제")
            .description("등록된 캐릭터 삭제")
            .create_option(|option| {
                option
                    .name("캐릭터명")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }
}
