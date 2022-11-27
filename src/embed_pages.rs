use futures::StreamExt;
use rand::seq::SliceRandom;
use serenity::{
    async_trait,
    builder::{CreateActionRow, CreateEmbed, CreateSelectMenu, CreateSelectMenuOption, CreateSelectMenuOptions},
    client::Context,
    model::{
        application::interaction::{
            application_command::ApplicationCommandInteraction, InteractionResponseType,
        },
        id::GuildId,
        prelude::{
            Message,
            interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        },
    },
    framework::{
        standard::CommandResult,
    }
};

use crate::{
    database_handler, user_info::*, 
    command_handler::{commands, command_return::CommandReturn}, 
    DBContainer, 
    loa_contents::{LOA_CONTENTS, CONTENTS_NAME},
};

use std::{collections::HashMap, option};
use std::sync::Arc;
use std::time::Duration;
use lazy_static::lazy_static;
use log::error;

static EMPTY_STR: &str = "\u{200b}";

#[derive(Debug, Clone)]
pub struct EmbedPages {
    pub pages: Vec<CreateEmbed>,
    pub menu: CreateSelectMenu,
}

impl EmbedPages {
    pub fn from_user_info(userinfo: UserInfo) -> EmbedPages {
        let mut pages: Vec<(f64, CreateEmbed)> = vec![];
        let mut menu = CreateSelectMenu::default();
        menu.custom_id("character_list")
            .placeholder("캐릭터를 선택해주세요");

        let mut options = vec![];

        for (_name, _charinfo) in userinfo.user_character().iter() {
            let mut embed = CreateEmbed::default();
            embed
                .title(format!("```{}```", _name))
                .description(format!("{} {}", _charinfo.class, _charinfo.lv))
                .fields(
                    get_content_list(_charinfo)
                );
            pages.push((_charinfo.lv, embed));
            let mut option = CreateSelectMenuOption::default();
            option.label(_name);
            options.push((_charinfo.lv, option));
        }

        pages.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        options.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let pages = pages.iter().map(|(a, b)| b.to_owned()).collect::<Vec<_>>();
        let mut options = options.iter().map(|(a, b)| b.to_owned()).collect::<Vec<_>>();
        
        for (idx, option) in options.iter_mut().enumerate() {
            option.value(idx);
        }

        menu.options(move |f| {
            f.set_options(options)
        });
        println!("{:?}\n{:?}", pages, menu);
        EmbedPages { pages, menu }
    }
}

fn get_content_list(_charinfo: &CharData) -> Vec<(String, &str, bool)> {
    let contents_len = CONTENTS_NAME.len();
    let mut content_list = vec![];

    for i in (0..contents_len) {
        if (1 << i) & _charinfo.total_hw() != 0 {
            let mut s = String::new();
            if (1 << i) & _charinfo.done_hw() != 0 {
                s = format!("~~**{}**: **완료**~~", CONTENTS_NAME[i].2);
            } else {
                s = format!("**{}**: **가능**", CONTENTS_NAME[i].2);
            }
            content_list.push((CONTENTS_NAME[i].1, (s, EMPTY_STR, false)));
        }
    }
    content_list.sort_by_key(|a| -a.0);
    content_list.iter().map(|(a, b)| b.to_owned()).collect::<Vec<_>>()
}

pub async fn control_pages(
    ctx: &Context, 
    interaction: ApplicationCommandInteraction, 
    pages: EmbedPages
) -> Result<(), serenity::Error> {

    let embeds = pages.clone();

    if let Err(why) = interaction
        .edit_original_interaction_response(&ctx.http, |interaction| {
            interaction.components(|c| {
                c.create_action_row(|r| {
                    r.add_select_menu(pages.menu.clone())
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
                                    pages.pages[idx].clone()
                                )
                                .components(|f| {
                                    f.create_action_row(|f| {
                                        f.add_select_menu(embeds.menu.clone())
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