use serenity::{
    all::{CommandInteraction, Context, CreateCommand, CreateEmbed},
    async_trait,
};

use crate::GuildlyHandler;

#[async_trait]
pub trait GuildlyCommand: Sync + Send {
    fn name(&self) -> &'static str;
    fn create_command(&self, ctx: CreateCommand) -> CreateCommand;
    async fn execute(
        &self,
        handler: &GuildlyHandler,
        ctx: &Context,
        interaction: &CommandInteraction,
    ) -> CreateEmbed;
}

pub mod add;
pub mod remove;
pub mod search;
pub mod show_menu;
