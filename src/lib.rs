use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serenity::{
    all::{
        Colour, Command, Context, CreateCommand, CreateEmbed, CreateEmbedAuthor,
        CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler, Interaction,
        Ready,
    },
    async_trait,
};
use url::Url;

use crate::commands::GuildlyCommand;

pub mod database;

pub mod commands;

pub struct GuildlyHandler {
    database: database::Database,
    link_finder: linkify::LinkFinder,
    commands: HashMap<&'static str, Box<dyn GuildlyCommand>>,
}

fn url_filter<'a>(link: linkify::Link<'a>) -> Option<u64> {
    let url = Url::parse(link.as_str()).unwrap();
    if matches!(
        url.host(),
        Some(url::Host::Domain("discord.com" | "discordapp.com"))
    ) {
        let mut segments = url.path_segments().unwrap();
        if Some("channels") == segments.next()
            && let Some(raw_guild_id) = segments.next()
            && let Ok(guild_id) = raw_guild_id.parse::<u64>()
        {
            return Some(guild_id);
        }
    }
    None
}

impl GuildlyHandler {
    pub fn new(database: database::Database) -> Self {
        let mut link_finder = linkify::LinkFinder::new();
        link_finder.kinds(&[linkify::LinkKind::Url]);
        link_finder.url_must_have_scheme(false);
        Self {
            database,
            link_finder,
            commands: HashMap::new(),
        }
    }
    pub fn register(&mut self, command: Box<dyn GuildlyCommand>) {
        self.commands.insert(command.name(), command);
    }
    pub fn guild_link_finder(&self, text: &str) -> impl Iterator<Item = u64> {
        self.link_finder.links(text).filter_map(url_filter)
    }
}

#[async_trait]
impl EventHandler for GuildlyHandler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            if let Some(guildly_command) = self.commands.get(command.data.name.as_str()) {
                let embed = guildly_command.execute(self, &ctx, &command).await;

                let _ = command
                    .create_response(
                        &ctx.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new().add_embed(embed),
                        ),
                    )
                    .await
                    .unwrap();
            } else {
                println!("Not Found Command: {}", command.data.name);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        for command in self.commands.values() {
            let command = Command::create_global_command(
                &ctx.http,
                command.create_command(CreateCommand::new(command.name())),
            )
            .await
            .unwrap();
            println!("created command {:?}", command);
        }
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GuildEntry {
    pub name: String,
    pub guild_id: u64,
    pub invite_url: Option<Url>,
    pub icon_url: Option<Url>,
}

pub struct Color;

impl Color {
    pub const INFO: Colour = Colour::BLITZ_BLUE;
    pub const WARN: Colour = Colour::ORANGE;
    pub const ERROR: Colour = Colour::RED;
}

pub fn create_embed_from_entry(entry: &GuildEntry) -> CreateEmbed {
    let mut auther = CreateEmbedAuthor::new(&entry.name);
    if let Some(invite_url) = &entry.invite_url {
        auther = auther.url(invite_url.as_str());
    }
    if let Some(icon_url) = &entry.icon_url {
        auther = auther.icon_url(icon_url.as_str());
    }

    CreateEmbed::new().color(Color::INFO).author(auther)
}

pub fn create_embed_from_entries(entries: &[GuildEntry]) -> CreateEmbed {
    if entries.is_empty() {
        CreateEmbed::new()
            .color(Color::WARN)
            .title("No Servers Found")
    } else if entries.len() == 1 {
        create_embed_from_entry(&entries[0])
    } else {
        CreateEmbed::new()
            .color(Color::INFO)
            .title("Servers")
            .fields(entries.iter().map(|entry| {
                (
                    if let Some(invite_url) = &entry.invite_url {
                        format!("[{}]({})", entry.name, invite_url.as_str())
                    } else {
                        entry.name.clone()
                    },
                    "",
                    false,
                )
            }))
    }
}
