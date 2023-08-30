use poise::serenity_prelude::{ChannelId, Context, Guild, User};
use songbird::{error::JoinError, Call, Event, Songbird};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{event::music_event::TrackEndHandler, util, Context as CustomContext, Error};

/// Join's your voice call
#[command(slash_command, ephemeral)]
pub async fn join(ctx: CustomContext<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let (manager, channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    let _ = join_channel(
        manager,
        &guild,
        channel,
        Some((ctx.serenity_context(), ctx.author(), &ctx.channel_id())),
    )
    .await;

    info!("Join command completed");
    ctx.say(">~<").await?;
    Ok(())
}

pub async fn join_channel(
    manager: Arc<Songbird>,
    guild: &Guild,
    voice_channel: ChannelId,
    event_context: Option<(&Context, &User, &ChannelId)>,
) -> Result<Arc<Mutex<Call>>, JoinError> {
    let (handler_lock, success) = manager.join(guild.id, voice_channel).await;
    if let Some(event) = event_context {
        let mut handler = handler_lock.lock().await;
        handler.add_global_event(
            Event::Track(songbird::TrackEvent::End),
            TrackEndHandler {
                guild: guild.clone(),
                context: event.0.clone(),
                author: event.1.clone(),
                channel_id: event.2.clone(),
            },
        );
    }
    success.map(|_| handler_lock)
}
