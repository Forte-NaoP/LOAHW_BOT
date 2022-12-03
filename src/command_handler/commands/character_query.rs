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
        command_return::{CommandReturn, BuildActionRow},
    },
    embed_pages,
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
        
        match database_handler::user_query(
            &ctx.data.read().await.get::<DBContainer>().unwrap(),
            command.user.tag(),
        ).await {
            Ok(result) => {
                let pages = embed_pages::EmbedPages::from_user_info(result);
                CommandReturn::EmbedPages(pages)
            },
            Err(e) => CommandReturn::String("등록되지 않음".to_owned())
        }
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

