use poise::serenity_prelude::Guild;
use songbird::Songbird;
use std::sync::Arc;

use crate::{util, Context, Error};

/// Bot will join your call
#[command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    // TODO add check for channel
    let (guild, manager, _channel) = util::manager::get_manager(ctx).await.unwrap();
    leave_channel(manager, guild).await;
    info!("leave command completed");
    ctx.defer_ephemeral().await?;
    ctx.say("I left you forever").await?;
    Ok(())
}

/// Leave channel
/// manager(songbird) and pass guild
pub async fn leave_channel(manager: Arc<Songbird>, guild: Guild) {
    let handler = manager.get(guild.id).unwrap();
    let _owo = manager.leave(guild.id).await;
    let music = handler.lock().await;
    music.queue().stop();
}
