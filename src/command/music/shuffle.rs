use crate::model::queue::GetQueueByGuildId;
use crate::{Context as CustomContext, Error};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;

/// Shuffle the current queue
#[command(slash_command)]
pub async fn shuffle(ctx: CustomContext<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let mut queue_map = ctx.data().queue_map.get_queue_map().await;
    let queue = queue_map.get_queue_by_id(guild.id);

    if queue.len() > 2 {
        {
            let mut rng: ThreadRng = thread_rng();
            queue.shuffle(&mut rng);
        }
        ctx.say("Queue has been shuffled").await?;
    } else {
        ctx.say("Cannot shuffle a queue when there is nothing to shuffle")
            .await?;
    }

    Ok(())
}
