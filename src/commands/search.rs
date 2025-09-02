use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
        CreateCommandOption, CreateEmbed, ResolvedValue,
    },
    async_trait,
};

use crate::{
    Color, GuildlyHandler, commands::GuildlyCommand, create_embed_from_entries,
    create_embed_from_entry,
};

pub struct SearchServer;

#[async_trait]
impl GuildlyCommand for SearchServer {
    fn name(&self) -> &'static str {
        "search"
    }

    fn create_command(&self, ctx: CreateCommand) -> CreateCommand {
        ctx.kind(CommandType::ChatInput)
            .description("Search servers")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "id", "Server Id")
                    .required(false),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "name", "Server Name")
                    .required(false),
            )
    }

    async fn execute(
        &self,
        handler: &GuildlyHandler,
        _ctx: &Context,
        interaction: &CommandInteraction,
    ) -> CreateEmbed {
        let mut guild_id = None;
        let mut guild_name = None;
        for option in interaction.data.options() {
            match option.name {
                "id" => {
                    let ResolvedValue::String(option) = option.value else {
                        unreachable!()
                    };
                    let Ok(parsed) = option.parse::<u64>() else {
                        return CreateEmbed::new()
                            .color(Color::ERROR)
                            .title("Invalid Server ID");
                    };
                    guild_id = Some(parsed)
                }
                "name" => {
                    let ResolvedValue::String(option) = option.value else {
                        unreachable!()
                    };
                    guild_name = Some(option)
                }
                _ => {}
            }
        }
        match (guild_id, guild_name) {
            (Some(guild_id), None) => {
                if let Some(entry) = handler.database.get(guild_id).unwrap() {
                    create_embed_from_entry(&entry)
                } else {
                    CreateEmbed::new()
                        .color(Color::WARN)
                        .title("No Servers Found")
                }
            }
            (None, Some(guild_name)) => {
                let entries = handler.database.search(guild_name).unwrap();
                create_embed_from_entries(&entries)
            }
            _ => todo!(),
        }
    }
}
