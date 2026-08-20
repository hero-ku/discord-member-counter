use poise::serenity_prelude::{self as serenity, ChannelId};

#[derive(Clone)]
pub enum CounterEffect {
    ChannelLabel(ChannelId, String),
}

impl CounterEffect {
    pub async fn handle_update(&self, ctx: std::sync::Arc<serenity::Http>, count: u32) {
        let _ = match self {
            CounterEffect::ChannelLabel(channel_id, prompt) => {
                channel_id
                    .edit(
                        ctx,
                        serenity::EditChannel::new().name(prompt.replace("{}", &count.to_string())),
                    )
                    .await
            }
        };
    }
}
