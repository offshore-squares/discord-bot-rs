use poise::serenity_prelude::Guild;
use songbird::Songbird;
use std::sync::Arc;

use crate::{model::queue::GetQueueByGuildId, util, Context, Error};

/// Bot will join your call
#[command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let (manager, _channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();
    leave_channel(manager, guild.clone()).await;
    let mut queue_map = ctx.data().queue_map.get_queue_map().await;
    let queue = queue_map.get_queue_by_id(guild.id);
    queue.clear();
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
