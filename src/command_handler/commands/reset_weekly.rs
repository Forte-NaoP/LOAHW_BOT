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


struct ResetWeekly;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(ResetWeekly)
}


#[async_trait]
impl CommandInterface for ResetWeekly {
    async fn run(
        &self, 
        ctx: &Context, 
        command: &ApplicationCommandInteraction, 
        options: &[CommandDataOption]
    ) -> CommandReturn {
        
        database_handler::reset_manually_one(&ctx.data.read().await.get::<DBContainer>().unwrap(), command.user.tag()).await;
        command.channel_id.say(&ctx.http, String::from("짜잔")).await.unwrap();
        
        CommandReturn::None
    }

    fn register<'a: 'b, 'b>(
        &'a self,
        command: &'a mut CreateApplicationCommand
    ) -> &'b mut CreateApplicationCommand {
        command
            .name("숙제초기화")
            .description("주간 숙제 수동 초기화")
    }
}
