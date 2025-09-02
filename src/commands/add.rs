use serenity::{
    all::{
        CommandInteraction, CommandOptionType, CommandType, Context, CreateCommand,
        CreateCommandOption, CreateEmbed, ResolvedValue,
    },
    async_trait,
};
use url::Url;

use crate::{Color, GuildEntry, GuildlyHandler, commands::GuildlyCommand, create_embed_from_entry};

pub struct AddServer;

#[async_trait]
impl GuildlyCommand for AddServer {
    fn name(&self) -> &'static str {
        "add"
    }
    fn create_command(&self, ctx: CreateCommand) -> CreateCommand {
        ctx.kind(CommandType::ChatInput)
            .description("Add a server")
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "id", "Server Id")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "name", "Guild Name")
                    .required(true),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "icon", "Icon Url")
                    .required(false),
            )
            .add_option(
                CreateCommandOption::new(CommandOptionType::String, "invite", "Invite Url")
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
        let mut guild_icon = None;
        let mut guild_invite = None;
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
                "icon" => {
                    let ResolvedValue::String(option) = option.value else {
                        unreachable!()
                    };
                    let Ok(icon_url) = Url::parse(option) else {
                        return CreateEmbed::new()
                            .color(Color::ERROR)
                            .title("Invalid Icon URL");
                    };
                    guild_icon = Some(icon_url)
                }
                "invite" => {
                    let ResolvedValue::String(option) = option.value else {
                        unreachable!()
                    };
                    let Ok(invite_url) = Url::parse(option) else {
                        return CreateEmbed::new()
                            .color(Color::ERROR)
                            .title("Invalid Icon URL");
                    };
                    guild_invite = Some(invite_url)
                }
                _ => {}
            }
        }

        let Some(name) = guild_name else {
            return CreateEmbed::new()
                .color(Color::ERROR)
                .title("No Guild Name");
        };

        let Some(guild_id) = guild_id else {
            return CreateEmbed::new().color(Color::ERROR).title("No Guild ID");
        };

        let entry = GuildEntry {
            name: name.to_string(),
            guild_id,
            invite_url: guild_invite,
            icon_url: guild_icon,
        };

        let _ = handler.database.insert(&entry).unwrap();
        create_embed_from_entry(&entry).title("Server Added")
    }
}
