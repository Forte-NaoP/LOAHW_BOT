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
    database_handler, DBContainer, LoaContents, user_info::*,
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
        
        let nickname = Option::<String>::from(DataWrapper::from(options, 0)).unwrap();
        let lv = Option::<f64>::from(DataWrapper::from(options, 1)).unwrap();

        let mut charinfo: CharInfo = CharInfo::new();
        
        let mut chardata = CharData::from(
            String::from(""),
            lv,
            ctx.data.read().await.get::<LoaContents>().unwrap().to_integer(&lv)
        );

        charinfo.insert(nickname, chardata);

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
            .create_option(|option| {
                option
                    .name("레벨")
                    .description("레벨")
                    .kind(CommandOptionType::Number)
                    .min_number_value(50.0)
                    .max_number_value(9999.0)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("클래스")
                    .description("클래스")
                    .kind(CommandOptionType::String)
            })
    }
}

