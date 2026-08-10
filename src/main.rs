use poise::serenity_prelude as serenity;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[tokio::main]
async fn main() {
    let token = std::env::var("TOKEN").expect("Failed to start: TOKEN unspecified!");
    let intents = serenity::GatewayIntents::non_privileged()
        .union(serenity::GatewayIntents::GUILD_MEMBERS)
        .difference(serenity::GatewayIntents::GUILD_SCHEDULED_EVENTS);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions::<(), Error> {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
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
    _framework: poise::FrameworkContext<'_, (), Error>,
    _data: &(),
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            todo!()
        }
        _ => {}
    }

    Ok(())
}
