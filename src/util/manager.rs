use poise::serenity_prelude::ChannelId;
use songbird::Songbird;
use std::sync::Arc;

use crate::{Context, Error};

/**
 * does not work yet, multiple return it no worky
 */
pub async fn get_manager(
    ctx: Context<'_>,
) -> (
    Option<(Arc<Songbird>, ChannelId, Option<ChannelId>)>,
    Option<Result<(), Error>>,
) {
    let guild = ctx.guild().unwrap();

    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect = match voice_channel {
        Some(channel) => channel,
        None => {
            ctx.say("You are not in a voice channel, baka").await?;
            return Ok(());
        }
    };

    //manager for voice channel
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    return Some((manager, connect, voice_channel));
}
