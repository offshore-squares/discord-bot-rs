use poise::serenity_prelude::{ChannelId, Context as SerenityContext, Guild, GuildId};
use songbird::Songbird;
use std::sync::Arc;

use crate::Context;

/**
 * does work, but needs to be made into macro for more cool
 */
pub async fn get_manager(
    ctx: Context<'_>,
) -> Result<(Guild, Arc<Songbird>, ChannelId), Box<dyn std::error::Error>> {
    let guild = ctx.guild().unwrap();

    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
        .unwrap();

    //manager for voice channel
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    Ok((guild, manager, voice_channel))
}

pub async fn get_manager_serenity(ctx: SerenityContext) -> Arc<Songbird> {
    //manager for voice channel
    songbird::get(&ctx).await.expect("loaded").clone()
}
