use crate::{util, Context as CustomContext, Error};

/// Set volume for the whole queue
#[command(slash_command)]
pub async fn volume(
    ctx: CustomContext<'_>,
    #[description = "Volume amount between 0.0 - 2.0(2x default)"]
    #[min = 0.0]
    #[max = 2.0]
    volume_amount: f32,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let (manager, _voice_channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    let handler_lock = manager.get(guild.id).unwrap();
    let handler = handler_lock.lock().await;
    let call = handler.queue();
    call.modify_queue(|q| {
        q.iter().for_each(|song| {
            song.set_volume(volume_amount).unwrap();
        });
    });
    ctx.say(format!(
        "uwu imagine setting volume to {}",
        volume_amount * 100.0
    ))
    .await?;
    Ok(())
}
