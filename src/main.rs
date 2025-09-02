use std::path::PathBuf;

use clap::{Parser, Subcommand};
use guildly::{commands, database::Database, GuildEntry, GuildlyHandler};
use serenity::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "FILE")]
    database: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Export {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Import {
        #[arg(short, long, value_name = "FILE")]
        file: PathBuf,
    },
    Run {
        #[arg(short, long)]
        token: String,
    },
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let database = Database::open(args.database).unwrap();

    match args.command {
        Commands::Export { file } => {
            let entries = database.export().unwrap();
            serde_json::to_writer_pretty(std::fs::File::create(file).unwrap(), &entries).unwrap();
        }
        Commands::Import { file } => {
            let entries: Vec<GuildEntry> = serde_json::from_reader(std::fs::File::open(file).unwrap()).unwrap();
            database.import(&entries).unwrap();
        }
        Commands::Run { token } => {
            let intents = GatewayIntents::GUILD_MESSAGES
                | GatewayIntents::DIRECT_MESSAGES
                | GatewayIntents::MESSAGE_CONTENT;

            let mut handler = GuildlyHandler::new(database);

            handler.register(Box::new(commands::add::AddServer));
            handler.register(Box::new(commands::search::SearchServer));
            handler.register(Box::new(commands::show_menu::ShowServersMenu));

            let mut client = Client::builder(&token, intents)
                .event_handler(handler)
                .await
                .expect("Err creating client");

            if let Err(why) = client.start().await {
                println!("Client error: {why:?}");
            }
        }
    }
}
