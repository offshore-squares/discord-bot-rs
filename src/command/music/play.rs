use crate::{command, util, Context, Error};
use poise::serenity_prelude::{Color, CreateEmbed, User};
use songbird::input::Metadata;
use std::vec;

/// Plays your song => with url
#[command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search tag"] search_query: String,
) -> Result<(), Error> {
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

    ctx.defer().await?;
    let search_query = search_query.trim().to_string();
    let input = if search_query.starts_with("https://www.youtube.com/") {
        songbird::input::ytdl(search_query)
            .await
            .map_err(|e| format!("ytdl failed {:#?}", e))?
    } else {
        songbird::input::ytdl_search(search_query)
            .await
            .map_err(|e| format!("ytdl_search failed {:#?}", e))?
    };

    let metadata = input.metadata.clone();

    let (track, _) = songbird::create_player(input);

    {
        let mut handler = handler.lock().await;
        handler.enqueue(track);

        // If this will be the current song send embed otherwise end message that its added to queue
        if handler.queue().len() == 1 {
            let _ = ctx
                .send(|reply| {
                    reply.embed(|embed| {
                        send_music_embed(metadata, embed, &ctx.author());
                        embed
                    })
                })
                .await;
        } else {
            ctx.say(format!("{} added to queue", metadata.title.unwrap()))
                .await?;
        }
    }

    Ok(())
}

fn format_duration(duration: u64) -> String {
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;

    if hours != 0 {
        format!("{:0>2}:{:0>2}:{:0>2} hours", hours, minutes, seconds)
    } else if minutes != 0 {
        format!("{:0>2}:{:0>2} minutes", minutes, seconds)
    } else {
        format!("{} seconds", seconds)
    }
}

pub fn send_music_embed(metadata: Box<Metadata>, embed: &mut CreateEmbed, author: &User) {
    let duration = format_duration(metadata.duration.unwrap().as_secs());
    if let Metadata {
        title: Some(title),
        thumbnail: Some(thumbnail),
        source_url: Some(source_url),
        artist: Some(artist),
        date: Some(date),
        ..
    } = *metadata
    {
        embed
            .title(title)
            .thumbnail(thumbnail)
            .author(|f| {
                f.icon_url(author.avatar_url().unwrap())
                    .name(author.name.clone())
            })
            .color(Color::from_rgb(0, 128, 128))
            .url(source_url)
            .fields(vec![
                ("author", artist, true),
                ("duration", duration, true),
                (
                    "upload date",
                    format!("{}-{}-{}", &date[4..6], &date[6..], &date[..4]),
                    true,
                ),
            ]);
    }
}
