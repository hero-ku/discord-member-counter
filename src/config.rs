use std::collections::HashMap;

use poise::serenity_prelude::{GuildId, RoleId};
use serde::{Deserialize, Serialize};

use crate::{counter::MemberCounter, effect::CounterEffect};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub guild_id: GuildId,
    pub counters: HashMap<String, CounterConfig>,
}

#[derive(Serialize, Deserialize)]
pub struct CounterConfig {
    #[serde(flatten)]
    counter_type: CounterType,
    effects: Vec<CounterEffect>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CounterType {
    Role { id: RoleId },
}

impl CounterConfig {
    pub fn build(&self) -> MemberCounter {
        match &self.counter_type {
            CounterType::Role { id } => MemberCounter::from_role(*id, vec![]),
        }
    }
}
