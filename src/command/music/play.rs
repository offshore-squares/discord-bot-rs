use crate::{
    command,
    model::queue::GetQueueByGuildId,
    util::{
        self,
        music::{search, send_music_embed},
    },
    Context, Error,
};
use std::vec;

/// Plays your song => with url
#[command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search tag"] search_query: String,
) -> Result<(), Error> {
    let search_query = search_query.trim().to_string();
    let guild = ctx.guild().unwrap();

    let data = ctx.serenity_context().data.read().await;
    let data = data.get::<crate::DataKey>().unwrap();

    let mut queue_map = data.queue_map.get_queue_map().await;
    let queue = queue_map.get_queue_by_id(guild.id);

    let (manager, voice_channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    let handler = if let Some(handler) = manager.get(guild.id) {
        handler
    } else {
        command::music::join::join_channel(
            manager,
            &guild,
            voice_channel,
            Some((ctx.serenity_context(), ctx.author(), &ctx.channel_id())),
        )
        .await?
    };

    ctx.defer().await?;
    // Get song from source
    let input = search(search_query).await.unwrap();
    // Get metadata
    let metadata = input.metadata.clone();
    // if own queue is empty enqueue song
    println!("{:#?}", queue.iter().map(|s| s.title.as_ref().unwrap()));

    // TODO Should not be added when command is called for the first time
    queue.push(*metadata.clone());
    if queue.len() == 1 {
        let (track, _) = songbird::create_player(input);
        {
            let mut handler = handler.lock().await;

            handler.enqueue(track);
            let _ = ctx
                .send(|reply| {
                    reply.embed(|embed| {
                        send_music_embed(metadata, embed, &ctx.author());
                        embed
                    })
                })
                .await;
        }
    } else {
        ctx.say(format!(
            "{} - {} added to queue",
            metadata.title.unwrap(),
            metadata.artist.unwrap()
        ))
        .await?;
    }

    Ok(())
}
