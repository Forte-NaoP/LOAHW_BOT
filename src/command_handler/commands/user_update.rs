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

struct UserUpdate;

#[async_trait]
impl CommandInterface for UserUpdate {
    async fn run(&self, ctx: &Context, msg: &Message, options: &[CommandDataOption]) -> CommandResult {
        
        let nickname = Option::<String>::from(DataWrapper::from(options, 0)).unwrap();
        let lv = Option::<f64>::from(DataWrapper::from(options, 0)).unwrap();

        let mut charinfo: CharInfo = CharInfo::new();
        
        let mut chardata = CharData::from(
            String::from(""),
            lv,
            ctx.data.read().await.get::<LoaContents>().unwrap().to_integer(&lv)
        );

        charinfo.insert(nickname, chardata);

        let userinfo = UserInfo::new(msg.author.tag(), charinfo);

        let result = database_handler::user_update(&ctx.data.read().await.get::<DBContainer>().unwrap(), userinfo).await;
        match result {
            Ok(()) => {
                msg.channel_id.say(&ctx.http, "성공").await?;
            },
            Err(why) => {
                msg.channel_id.say(&ctx.http, why.to_string()).await?;
            }
        }
        Ok(())
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("등록")
            .description("보유 캐릭터 등록/추가/수정")
            .create_option(|option| {
                option
                    .name("캐릭터명")
                    .kind(CommandOptionType::String)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("레벨")
                    .kind(CommandOptionType::Number)
                    .min_number_value(50.0)
                    .max_number_value(9999.0)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("클래스")
                    .kind(CommandOptionType::String)
            })
    }
}

