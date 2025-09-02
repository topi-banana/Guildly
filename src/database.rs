use std::path::Path;

use duckdb::{Connection, Error, params};
use url::Url;

use crate::GuildEntry;

pub struct Database {
    database: Connection,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS guilds (
                guild_id   BIGINT PRIMARY KEY,
                name       TEXT NOT NULL,
                invite_url TEXT,
                icon_url   TEXT
            );",
        )?;
        Ok(Self { database: conn })
    }

    pub fn insert(&self, value: &GuildEntry) -> Result<Option<GuildEntry>, Error> {
        let old = self.get(value.guild_id)?;

        self.database.execute(
            "INSERT OR REPLACE INTO guilds (guild_id, name, invite_url, icon_url)
             VALUES (?, ?, ?, ?);",
            params![
                value.guild_id as i64,
                value.name,
                value.invite_url.as_ref().map(|u| u.to_string()),
                value.icon_url.as_ref().map(|u| u.to_string())
            ],
        )?;
        Ok(old)
    }

    pub fn remove(&self, guild_id: u64) -> Result<Option<GuildEntry>, Error> {
        let old = self.get(guild_id)?;
        if old.is_some() {
            self.database.execute(
                "DELETE FROM guilds WHERE guild_id = ?;",
                params![guild_id as i64],
            )?;
        }
        Ok(old)
    }

    pub fn get(&self, guild_id: u64) -> Result<Option<GuildEntry>, Error> {
        let mut stmt = self.database.prepare(
            "SELECT name, guild_id, invite_url, icon_url
             FROM guilds WHERE guild_id = ?;",
        )?;
        let mut rows = stmt.query(params![guild_id as i64])?;

        if let Some(row) = rows.next()? {
            Ok(Some(GuildEntry {
                name: row.get::<_, String>(0)?,
                guild_id: row.get::<_, i64>(1)? as u64,
                invite_url: row
                    .get::<_, Option<String>>(2)?
                    .and_then(|s| Url::parse(&s).ok()),
                icon_url: row
                    .get::<_, Option<String>>(3)?
                    .and_then(|s| Url::parse(&s).ok()),
            }))
        } else {
            Ok(None)
        }
    }

    pub fn search(&self, name: &str) -> Result<Vec<GuildEntry>, Error> {
        let mut stmt = self.database.prepare(
            "SELECT name, guild_id, invite_url, icon_url
             FROM guilds WHERE name LIKE ?;",
        )?;
        let mut rows = stmt.query(params![format!("%{}%", name)])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(GuildEntry {
                name: row.get::<_, String>(0)?,
                guild_id: row.get::<_, i64>(1)? as u64,
                invite_url: row
                    .get::<_, Option<String>>(2)?
                    .and_then(|s| Url::parse(&s).ok()),
                icon_url: row
                    .get::<_, Option<String>>(3)?
                    .and_then(|s| Url::parse(&s).ok()),
            });
        }
        Ok(results)
    }

    pub fn export(&self) -> Result<Vec<GuildEntry>, Error> {
        let mut stmt = self.database.prepare(
            "SELECT name, guild_id, invite_url, icon_url
             FROM guilds;",
        )?;
        let mut rows = stmt.query([])?;

        let mut results = Vec::new();
        while let Some(row) = rows.next()? {
            results.push(GuildEntry {
                name: row.get::<_, String>(0)?,
                guild_id: row.get::<_, i64>(1)? as u64,
                invite_url: row
                    .get::<_, Option<String>>(2)?
                    .and_then(|s| Url::parse(&s).ok()),
                icon_url: row
                    .get::<_, Option<String>>(3)?
                    .and_then(|s| Url::parse(&s).ok()),
            });
        }
        Ok(results)
    }

    pub fn import(&self, entries: &[GuildEntry]) -> Result<(), Error> {
        for entry in entries {
            self.insert(entry)?;
        }
        Ok(())
    }
}
