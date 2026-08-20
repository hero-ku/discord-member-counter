use crate::{config::Config, counter::MemberCounter};
use poise::serenity_prelude::{self as serenity, GuildId};
use std::sync::Mutex;

pub mod config;
pub mod counter;
pub mod effect;

struct Data {
    counters: Mutex<Vec<MemberCounter>>,
    guild_id: GuildId,
}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let token = std::env::var("TOKEN").expect("Failed to start: TOKEN unspecified!");

    let config: Config = std::fs::read("config.toml")
        .map(|buf| toml::from_slice(&buf).unwrap())
        .unwrap();

    let intents = serenity::GatewayIntents::non_privileged()
        .union(serenity::GatewayIntents::GUILD_MEMBERS)
        .difference(serenity::GatewayIntents::GUILD_SCHEDULED_EVENTS);

    let counters = config
        .counters
        .iter()
        .map(|(_, config)| config.build())
        .collect();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(move |_ctx, _ready, _framework| {
            Box::pin(async move {
                Ok(Data {
                    counters: Mutex::new(counters),
                    guild_id: config.guild_id,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { .. } => {
            let members = data.guild_id.members(ctx, None, None).await.unwrap();

            let mut counters = data.counters.lock().unwrap();
            for counter in counters.iter_mut() {
                counter.refresh_count(ctx, &members);
            }

            // Request the guild members to be sent over in chunks
            // This automatically populates the cache
            ctx.shard.chunk_guild(
                data.guild_id,
                None,
                false,
                serenity::ChunkGuildFilter::None,
                None,
            );
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let mut counters = data.counters.lock().unwrap();

            for counter in counters.iter_mut() {
                counter.member_joined(ctx, new_member);
            }
        }
        serenity::FullEvent::GuildMemberRemoval {
            member_data_if_available,
            ..
        } => {
            if let Some(old_member) = member_data_if_available {
                let mut counters = data.counters.lock().unwrap();

                for counter in counters.iter_mut() {
                    counter.member_left(ctx, old_member);
                }
            }
        }
        serenity::FullEvent::GuildMemberUpdate {
            new,
            old_if_available,
            ..
        } => {
            if let Some(member) = new
                && let Some(old_member) = old_if_available
            {
                let mut counters = data.counters.lock().unwrap();

                for counter in counters.iter_mut() {
                    counter.member_updated(ctx, member, old_member);
                }
            }
        }
        _ => {}
    }

    Ok(())
}
