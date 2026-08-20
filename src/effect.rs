use crate::Error;
use poise::serenity_prelude::{self as serenity, ChannelId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum CounterEffect {
    #[serde(rename = "label")]
    ChannelLabel {
        channel_id: ChannelId,
        prompt: String,
    },
}

impl CounterEffect {
    pub async fn handle_update(
        &self,
        ctx: std::sync::Arc<serenity::Http>,
        count: u32,
    ) -> Result<(), Error> {
        match self {
            CounterEffect::ChannelLabel { channel_id, prompt } => {
                channel_id
                    .edit(
                        ctx,
                        serenity::EditChannel::new().name(prompt.replace("{}", &count.to_string())),
                    )
                    .await?
            }
        };

        Ok(())
    }
}
