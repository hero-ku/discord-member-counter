use std::sync::Mutex;

use poise::serenity_prelude as serenity;

use crate::counter::MemberCounter;

pub mod counter;

struct Data {
    counters: Mutex<Vec<MemberCounter>>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().unwrap();

    let token = std::env::var("TOKEN").expect("Failed to start: TOKEN unspecified!");
    let intents = serenity::GatewayIntents::non_privileged()
        .union(serenity::GatewayIntents::GUILD_MEMBERS)
        .difference(serenity::GatewayIntents::GUILD_SCHEDULED_EVENTS);

    let counters = Vec::new();

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
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            let mut counters = data.counters.lock().unwrap();

            for counter in counters.iter_mut() {
                counter.member_joined(new_member);
            }
        }
        serenity::FullEvent::GuildMemberRemoval {
            member_data_if_available,
            ..
        } => {
            if let Some(old_member) = member_data_if_available {
                let mut counters = data.counters.lock().unwrap();

                for counter in counters.iter_mut() {
                    counter.member_left(old_member);
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
                    counter.member_updated(member, old_member);
                }
            }
        }
        _ => {}
    }

    Ok(())
}
