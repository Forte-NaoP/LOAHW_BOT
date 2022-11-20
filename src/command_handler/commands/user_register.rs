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

use reqwest::header::USER_AGENT;
use scraper::{Html, Selector};
use futures;
use std::sync::{Arc, Mutex};

use crate::{
    database_handler, DBContainer, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
        command_return::CommandReturn,
    },
    crawler::profile_parser::{self, get_character_list},
};

struct UserRegister;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(UserRegister)
}

#[async_trait]
impl CommandInterface for UserRegister {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        
        let name = Option::<String>::from(DataWrapper::from(options, 0)).unwrap();

        let mut character_list = vec![];

        match get_character_list(name).await {
            Some(mut s) => character_list.append(&mut s),
            None => return CommandReturn::String("캐릭터 없음".to_string()),
        }

        let mut charinfo: CharInfo = character_list.into_iter().collect();

        let userinfo = UserInfo::new(command.user.tag(), charinfo);

        let result = database_handler::user_update(&ctx.data.read().await.get::<DBContainer>().unwrap(), userinfo).await;
        match result {
            Ok(()) => {
                CommandReturn::String("성공".to_string())
            },
            Err(why) => {
                CommandReturn::String("실패".to_string())
            }
        }
        
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("등록")
            .description("보유 캐릭터 등록/추가/수정")
            .create_option(|option| {
                option
                    .name("캐릭터명")
                    .description("캐릭터명")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
    }
}

