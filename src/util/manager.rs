use poise::serenity_prelude::{ChannelId, Context as SerenityContext, Guild, User};
use songbird::Songbird;
use std::sync::Arc;

/**
 * does work, but needs to be made into macro for more cool
 */
pub async fn get_manager(
    guild: &Guild,
    author: &User,
    context: &SerenityContext,
) -> Result<(Arc<Songbird>, ChannelId), Box<dyn std::error::Error>> {
    let voice_channel = guild
        .voice_states
        .get(&author.id)
        .and_then(|voice_state| voice_state.channel_id)
        .unwrap();

    //manager for voice channel
    let manager = get_manager_serenity(context).await;

    Ok((manager, voice_channel))
}

pub async fn get_manager_serenity(ctx: &SerenityContext) -> Arc<Songbird> {
    //manager for voice channel
    songbird::get(ctx).await.expect("loaded").clone()
}
