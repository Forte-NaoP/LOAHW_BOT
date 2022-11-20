use std::collections::HashMap;

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

struct UserUpdate;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(UserUpdate)
}

#[async_trait]
impl CommandInterface for UserUpdate {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        
        // 나중에 embed page 및 select box로 구현 
        CommandReturn::String("미구현".to_owned())
        
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("수정")
            .description("보유 캐릭터 정보 수정")
    }
}

