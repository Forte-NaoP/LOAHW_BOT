use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateActionRow, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption, CreateSelectMenuOptions},
    client::{Context},
    framework::standard::{
        CommandResult, Args,
    },
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        prelude::{
            Message,
            interaction::{application_command::{CommandDataOption, CommandDataOptionValue}, self},
            command::CommandOptionType,
        },
        user::User,
    },
};

use crate::{
    database_handler, DBContainer, loa_contents::{LOA_CONTENTS, CONTENTS_NAME}, user_info::*,
    command_handler::{
        command_handler::*,
        command_data::*,
        command_return::{CommandReturn, ControlInteraction},
    },
    utils::*,
};

use std::{any::Any, collections::HashMap};
use log::error;
use std::time::Duration;
use futures::StreamExt;

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
        
        let user_name = match Option::<User>::from(DataWrapper::from(options, 0)) {
            Some(user) => user.tag(),
            None => command.user.tag(),
        };

        match database_handler::user_query(
            &ctx.data.read().await.get::<DBContainer>().unwrap(),
            user_name,
        ).await {
            Ok(result) => {
                let pages = QueryResult::new(embeds_from_user_info(&result));
                CommandReturn::ControlInteraction(Box::new(pages))
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
            .create_option(|option| {
                option
                    .name("유저")
                    .description("디스코드 유저명 (Ex: @loahw_bot)")
                    .kind(CommandOptionType::User)
                    .required(false)
            })
    }
}

struct QueryResult {
    pub pages: HashMap<String, CreateEmbed>,
}

impl QueryResult {
    pub fn new(pages: HashMap<String, CreateEmbed>) -> QueryResult {
        QueryResult { pages }
    }
}

#[async_trait]
impl ControlInteraction for QueryResult {
    async fn control_interaction(
        &mut self,
        ctx: &Context, 
        interaction: ApplicationCommandInteraction, 
    ) -> Result<(), serenity::Error> {

        let mut names: Vec<(String, f64)> = vec![];

        for (name, embed) in self.pages.iter() {
            let mut lv = embed.0.get("description").unwrap().as_str().unwrap().split_whitespace();
            lv.next();
            names.push((
                name.to_owned(),
                lv.next().unwrap().parse::<f64>().unwrap()
            ));
        }
        names.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let mut names = names.iter().map(|(a, b)| a.to_owned()).collect::<Vec<_>>();

        if let Err(why) = interaction
            .edit_original_interaction_response(&ctx.http, |interaction| {
                interaction.set_embed(self.pages.get(&names[0]).unwrap().to_owned());
                interaction.components(|c| {
                    c.create_action_row(|r| {
                        r.add_select_menu(character_select_menu(&names, 0))
                    })
                })
            })
            .await
        {
            error!("an error occured while adding select menus.");
            return Err(why);
        }

        match interaction.get_interaction_response(&ctx.http).await {
            Ok(msg) => {
                let mut interaction_stream = msg
                    .await_component_interactions(&ctx)
                    .timeout(Duration::from_secs(60*5))
                    // .filter(move |f| {
                    //     f.message.id == msg.id && f.member.as_ref().unwrap().user.id == interaction.user.id
                    // })
                    .build();
                while let Some(select_option) = interaction_stream.next().await {
                    if let Err(why) = select_option
                        .create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|i| {
                                    let idx = select_option.data.values[0].parse::<usize>().unwrap();
                                    i.set_embed(
                                        self.pages.get(&names[idx]).unwrap().to_owned()
                                    )
                                    .components(|f| {
                                        f.create_action_row(|f| {
                                            f.add_select_menu(character_select_menu(&names, idx))
                                        })
                                    })
                                })
                        })
                        .await
                    {
                        error!("Couldn't set embed.");
                        return Err(why);
                    }
                }
            }
            Err(why) => {
                error!("Couldn't get message info from interaction.");
                return Err(why);
            }
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
