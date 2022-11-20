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


struct CharacterDelete;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(CharacterDelete)
}

#[async_trait]
impl CommandInterface for CharacterDelete {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        let nickname = Option::<String>::from(DataWrapper::from(options, 0)).unwrap();
        database_handler::character_delete(ctx.data.read().await.get::<DBContainer>().unwrap(), command.user.tag(), nickname).await.unwrap();
        // 보유 캐릭터 현황 보여주기
        command.channel_id.say(&ctx.http, format!("뿅")).await.unwrap();
        CommandReturn::None
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
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
