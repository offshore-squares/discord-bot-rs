use crate::{util, Context as CustomContext, Error};
use rand::thread_rng;
use rand::Rng;

#[command(slash_command)]
pub async fn shuffle(ctx: CustomContext<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let (manager, _voice_channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    let handler_lock = manager.get(guild.id).unwrap();
    let handler = handler_lock.lock().await;
    let queue = handler.queue();
    if queue.len() > 2 {
        queue.modify_queue(|q| {
            let mut rng: rand::rngs::ThreadRng = thread_rng();
            for _ in 0..q.len() * 3 {
                let old_song = rng.gen_range(0..q.len());
                let new_song = rng.gen_range(0..q.len());
                q.swap(old_song, new_song)
            }
        });
        ctx.say("Queue has been shuffled").await?;
    } else {
        ctx.say("Cannot shuffle a queue when there is nothing to shuffle")
            .await?;
    }

    Ok(())
}
