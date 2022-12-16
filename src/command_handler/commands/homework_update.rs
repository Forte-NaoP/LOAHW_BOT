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

struct CharacterUpdate;

pub fn command() -> Box<dyn CommandInterface + Sync + Send> {
    Box::new(CharacterUpdate)
}

#[async_trait]
impl CommandInterface for CharacterUpdate {
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
                let pages = QueryResult::new(embeds_from_user_info(&result), result);
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
            .name("숙제관리")
            .description("캐릭터 별 숙제 현황 관리")
    }
}

struct QueryResult {
    pub pages: HashMap<String, CreateEmbed>,
    pub info: UserInfo,
}

impl QueryResult {
    pub fn new(pages: HashMap<String, CreateEmbed>, info: UserInfo) -> QueryResult {
        QueryResult { pages, info }
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
        let names = names.iter().map(|(a, b)| a.to_owned()).collect::<Vec<_>>();

        if let Err(why) = interaction
            .edit_original_interaction_response(&ctx.http, |interaction| {
                interaction.set_embed(self.pages.get(&names[0]).unwrap().to_owned());
                interaction.components(|c| {
                    c.create_action_row(|r| {
                        r.add_select_menu(character_select_menu(&names, 0))
                    });
                    c.create_action_row(|r| {
                        let name = names.get(0).unwrap();
                        r.add_select_menu(
                            contents_select_menu(
                                name,
                                self.info.user_character().get(name).unwrap()
                            )
                        )
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
                let mut update = false;
                while let Some(response) = interaction_stream.next().await {
                    if let Err(why) = response
                        .create_interaction_response(&ctx.http, |r| {
                            r.kind(InteractionResponseType::UpdateMessage)
                                .interaction_response_data(|i| {
                                    match response.data.custom_id.as_str() {
                                        "character_select" => {
                                            let idx = response.data.values[0].parse::<usize>().unwrap();
                                            i.set_embed(
                                                self.pages.get(&names[idx]).unwrap().to_owned()
                                            )
                                            .components(|f| {
                                                f.create_action_row(|r| {
                                                    r.add_select_menu(character_select_menu(&names, idx))
                                                });
                                                f.create_action_row(|r| {
                                                    let name = names.get(idx).unwrap();
                                                    r.add_select_menu(
                                                        contents_select_menu(
                                                            name,
                                                            self.info.user_character().get(name).unwrap()
                                                        )
                                                    )
                                                })
                                            })
                                        },
                                        name @ _ => {
                                            let mut hw = 0u64;
                                            for idx in response.data.values.iter() {
                                                hw |= 1 << idx.parse::<u64>().unwrap();
                                            }
                                            let charinfo = self.info.user_character_mut().get_mut(name).unwrap();
                                            charinfo.set_done_hw(hw);
                                            update = true;
                                            self.pages = embeds_from_user_info(&self.info);
                                            
                                            i.set_embed(
                                                self.pages.get(name).unwrap().to_owned()
                                            )
                                            .components(|f| {
                                                f.create_action_row(|r| {
                                                    let idx = names.iter().position(|r| r == name).unwrap();
                                                    r.add_select_menu(character_select_menu(&names, idx))
                                                });
                                                f.create_action_row(|r| {
                                                    let name = names.get(0).unwrap();
                                                    r.add_select_menu(
                                                        contents_select_menu(
                                                            name,
                                                            self.info.user_character().get(name).unwrap()
                                                        )
                                                    )
                                                })
                                            })
                                        },
                                    }
                                })
                        })
                        .await
                    {
                        error!("Couldn't set embed.");
                        return Err(why);
                    }
                    if update {
                        database_handler::user_update(&ctx.data.read().await.get::<DBContainer>().unwrap(), self.info.clone()).await.unwrap();
                        update = false;
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
