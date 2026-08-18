use poise::serenity_prelude::GuildId;

#[derive(serde::Deserialize)]
pub struct Config {
    pub guild_id: GuildId,
}
