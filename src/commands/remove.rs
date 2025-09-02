use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
        CreateCommandOption, CreateEmbed, ResolvedValue,
    },
    async_trait,
};

use crate::{Color, GuildlyHandler, commands::GuildlyCommand, create_embed_from_entry};

pub struct RemoveServer;

#[async_trait]
impl GuildlyCommand for RemoveServer {
    fn name(&self) -> &'static str {
        "remove"
    }

    fn create_command(&self, ctx: CreateCommand) -> CreateCommand {
        ctx.kind(CommandType::ChatInput)
            .description("Remove a server")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "id", "Server Id")
                    .required(true),
            )
    }

    async fn execute(
        &self,
        handler: &GuildlyHandler,
        _ctx: &Context,
        interaction: &CommandInteraction,
    ) -> CreateEmbed {
        let mut guild_id = None;
        for option in interaction.data.options() {
            if option.name == "id" {
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
        }

        let Some(guild_id) = guild_id else {
            return CreateEmbed::new().color(Color::ERROR).title("No Guild ID");
        };

        if let Some(old_entry) = handler.database.remove(guild_id).unwrap() {
            create_embed_from_entry(&old_entry).title("Removed Server")
        } else {
            CreateEmbed::new()
                .color(Color::ERROR)
                .title("Not Found Guild")
        }
    }
}
