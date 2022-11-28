use crate::embed_pages;
use serenity::builder::CreateEmbed;

pub enum CommandReturn {
    String(String),
    SingleEmbed(CreateEmbed),
    EmbedPages(embed_pages::EmbedPages),
    None,
}