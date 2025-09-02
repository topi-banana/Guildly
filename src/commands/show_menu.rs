use serenity::{
    all::{CommandInteraction, CommandType, Context, CreateCommand, CreateEmbed},
    async_trait,
};

use crate::{GuildlyHandler, commands::GuildlyCommand, create_embed_from_entries};

pub struct ShowServersMenu;

#[async_trait]
impl GuildlyCommand for ShowServersMenu {
    fn name(&self) -> &'static str {
        "show servers"
    }

    fn create_command(&self, ctx: CreateCommand) -> CreateCommand {
        ctx.kind(CommandType::Message)
    }

    async fn execute(
        &self,
        handler: &GuildlyHandler,
        _ctx: &Context,
        interaction: &CommandInteraction,
    ) -> CreateEmbed {
        let mut entries = Vec::new();
        for message in interaction.data.resolved.messages.values() {
            for guild_id in handler.guild_link_finder(&message.content) {
                if let Some(result) = handler.database.get(guild_id).unwrap() {
                    entries.push(result);
                }
            }
        }
        create_embed_from_entries(&entries)
    }
}
