# 🔗 Guildly

Guildly is a Discord bot designed to quickly retrieve a server's invite link from a message link.

## How it Works ❓

Guildly maintains a database where server IDs are paired with their respective invite links. When a user provides a message link, the bot performs the following steps:

1. Extracts the server ID from the message link.
2. Searches its database for the extracted server ID.
3. If a matching entry is found, it returns the corresponding invite link.

The bot does not generate invite links on its own. It simply retrieves the invite link that was previously stored for that server.

## Features ⭐

- Retrieves a server's invite link using a message link.
- Manages a database of server IDs and invite links.
- Add and remove server entries through Discord commands.
- Provides a command-line interface to start the bot and manage data (import/export).

## Tech Stack 🛠️

This project is built with Rust and utilizes the following key libraries:

- **[Clap](https://github.com/clap-rs/clap)**: For parsing command-line arguments.
- **[Serenity](https://github.com/serenity-rs/serenity)**: A Rust library for the Discord API.
- **[DuckDB](https://duckdb.org/)**: An in-process SQL OLAP database management system.

## Usage 🚀

### Command-Line Interface

The bot is started and its data is managed through the command line. A database file must be specified for all operations.

```bash
guildly --database <DATABASE_FILE> <COMMAND>
```

- **Run the bot:**
  Starts the Discord bot.
  ```bash
  guildly --database <DATABASE_FILE> run --token <BOT_TOKEN>

  # example:
  guildly --database ./database.db run --token AAAAAAAAAAAAAAA
  ```

- **Export data:**
  Exports the server data from the database.
  ```bash
  guildly --database <DATABASE_FILE> export --file <PATH_TO_JSON_FILE>

  # example:
  guildly --database ./database.db export --file backup.json
  ```

- **Import data:**
  Imports server data into the database from a file.
  ```bash
  guildly --database <DATABASE_FILE> import --file <PATH_TO_JSON_FILE>

  # example:
  guildly --database ./database.db import --file backup.json
  ```

### Bot Commands

Once the bot is running on Discord, you can use the following commands:

- **Add a server:**
  `add`
  - `id:<server_id>` required
  - `name:<name>` required
  - `icon:<icon_url>` optional
  - `invite:<invite_link>` optional

- **Remove a server:**
  `remove`
  - `id:<server_id>` required
    
- **Search servers:**
  `search`
  - `id:<server_id>` optional
  - `name:<name>` optional

- **Show servers**
  You can search for message links within a message's content. Right-click on a message, navigate to `Apps`, and select `show servers`. The bot will then find any message links in the message and show you the corresponding server information.
