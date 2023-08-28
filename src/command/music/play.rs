use crate::{
    command,
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
    let mut queue = ctx.data().queue.clone();
    let guild = ctx.guild().unwrap();
    let (manager, voice_channel) =
        util::manager::get_manager(&guild, ctx.author(), ctx.serenity_context())
            .await
            .unwrap();

    // TODO Joins voice channel if bot hasn't joined already
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

    // if own queue is empty search song and play it
    if queue.len() == 0 {
        ctx.defer().await?;
        let input = search(search_query).await.unwrap();
        warn!("{:#?}", input);
        let metadata = input.metadata.clone();

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
        queue.push(search_query.clone());
        ctx.say(format!(
            "{} added to queue",
            search_query.trim().to_string()
        ))
        .await?;
    }

    Ok(())
}
