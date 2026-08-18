use std::collections::HashMap;

use poise::serenity_prelude::{GuildId, RoleId};
use serde::{Deserialize, Serialize};

use crate::counter::MemberCounter;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub guild_id: GuildId,
    pub counters: HashMap<String, CounterConfig>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
#[serde()]
pub enum CounterConfig {
    Role { id: RoleId },
}

impl CounterConfig {
    pub fn build(&self) -> MemberCounter {
        match self {
            CounterConfig::Role { id } => MemberCounter::from_role(*id),
        }
    }
}
