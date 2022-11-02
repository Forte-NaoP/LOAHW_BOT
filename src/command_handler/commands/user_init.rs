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
            interaction::application_command::CommandDataOption,
        },
        user::User,
    },
};

use crate::{
    database_handler, DBContainer, LoaContents, user_info::*,
    command_handler::command_handler::*,
};

struct UserInit;

#[async_trait]
impl CommandInterface for UserInit {
    async fn run(&self, ctx: &Context, msg: &Message, options: &[CommandDataOption]) -> CommandResult {
        database_handler::user_delete(ctx.data.read().await.get::<DBContainer>().unwrap(), msg.author.tag()).await.unwrap();
        msg.channel_id.say(&ctx.http, format!("reset data of {}", msg.author.tag())).await.unwrap();
        Ok(())
    }

    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
        command
            .name("사용자초기화")
            .description("등록된 사용자 정보 초기화")
    }
}
