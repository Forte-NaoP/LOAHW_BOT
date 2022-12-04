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

use std::any::Any;
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
        
        match database_handler::user_query(
            &ctx.data.read().await.get::<DBContainer>().unwrap(),
            command.user.tag(),
        ).await {
            Ok(result) => {
                let pages = QueryResult::from_user_info(result);
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
    }
}

struct QueryResult {
    pub pages: Vec<CreateEmbed>,
}

impl QueryResult {
    pub fn from_user_info(userinfo: UserInfo) -> QueryResult {
        let mut pages: Vec<(f64, CreateEmbed)> = vec![];

        for (_name, _charinfo) in userinfo.user_character().iter() {
            let mut embed = CreateEmbed::default();
            if _charinfo.lv >= 1302.0 {
                embed
                    .title(format!("```{}```", _name))
                    .description(format!("{} {}", _charinfo.class, _charinfo.lv))
                    .fields(
                        get_content_list(_charinfo)
                    );
                pages.push((_charinfo.lv, embed));
            }
        }

        pages.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        let pages = pages.iter().map(|(_, b)| b.to_owned()).collect::<Vec<_>>();

        QueryResult { pages }
    }
}

#[async_trait]
impl ControlInteraction for QueryResult {
    async fn control_interaction(
        &self,
        ctx: &Context, 
        interaction: ApplicationCommandInteraction, 
    ) -> Result<(), serenity::Error> {

        let mut names = vec![];

        for embed in self.pages.iter() {
            names.push(embed.0.get("title").unwrap().as_str().unwrap().replace("`", ""));
        }

        if let Err(why) = interaction
            .edit_original_interaction_response(&ctx.http, |interaction| {
                interaction.set_embed(self.pages[0].to_owned());
                interaction.components(|c| {
                    c.create_action_row(|r| {
                        r.add_select_menu(select_menu_from_vec(&names, 0))
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
                    .filter(move |f| {
                        f.message.id == msg.id && f.member.as_ref().unwrap().user.id == interaction.user.id
                    })
                    .build();
    
                while let Some(select_option) = interaction_stream.next().await {
                    if let Err(why) = select_option
                        .create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|i| {
                                    let idx = select_option.data.values[0].parse::<usize>().unwrap();
                                    i.set_embed(
                                        self.pages[idx].to_owned()
                                    )
                                    .components(|f| {
                                        f.create_action_row(|f| {
                                            f.add_select_menu(select_menu_from_vec(&names, idx))
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
