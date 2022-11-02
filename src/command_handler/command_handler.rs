use serenity::{
    async_trait,
    builder::CreateApplicationCommand,
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        },
    },
    framework::{
        standard::CommandResult,
    }
};

#[async_trait]
pub trait CommandInterface {
    async fn run(&self, ctx: &Context, msg: &Message, options: &[CommandDataOption]) -> CommandResult;
    fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand;
}
