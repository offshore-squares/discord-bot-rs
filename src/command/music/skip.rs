use crate::{util, Context as CustomContext, Error};

#[command(slash_command)]
pub async fn skip(ctx: CustomContext<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let (manager, _voice_channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    let handler_lock = manager.get(guild.id).unwrap();
    let handler = handler_lock.lock().await;

    let queue = handler.queue();
    if queue.len() > 0 {
        let current_song = queue.current().unwrap();
        let _ = queue.skip().map_err(|e| ctx.say(e.to_string()));
        ctx.say(format!(
            "skipped {}",
            current_song.metadata().title.clone().unwrap()
        ))
        .await?;
    } else {
        ctx.say("cannot skip a song without a queue").await?;
    }

    Ok(())
}
