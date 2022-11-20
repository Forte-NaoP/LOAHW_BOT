use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::{Context},
    framework::standard::{
        CommandResult, Args,
    },
    model::{
        application::interaction::application_command::ApplicationCommandInteraction,
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
            command::CommandOptionType,
        },
        user::User,
    },
};

use crate::{
    database_handler, DBContainer, loa_contents::LOA_CONTENTS, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
        command_return::CommandReturn,
    }
};


struct CharacterQuery;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(CharacterQuery)
}

#[async_trait]
impl CommandInterface for CharacterQuery {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        
        let result = database_handler::user_query(
            &ctx.data.read().await.get::<DBContainer>().unwrap(),
            command.user.tag(),
        ).await;

        command.channel_id.say(&ctx.http, format!("{}", msg_from_user_info(ctx, &result.unwrap()).await)).await;
        
        CommandReturn::None
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("조회")
            .description("보유 캐릭터 조회")
    }
}