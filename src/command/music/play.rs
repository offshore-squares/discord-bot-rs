use crate::{Context, Error};
use poise::serenity_prelude::Color;
use songbird::{input::Metadata, Driver};
use std::vec;

/// Plays your song => with url
#[command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search tag"] search_query: String,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    // Gets voice channel of user
    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    // Checks if user is in said voice channel
    let connect = match voice_channel {
        Some(channel) => channel,
        None => {
            ctx.say("You are not in a voice channel, baka").await?;
            return Ok(());
        }
    };

    // Get manager for voice channel
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    // Joins voice channel if bot hasn't joined already
    let handler = if let Some(handler) = manager.get(guild.id) {
        handler
    } else {
        let (handler, success) = manager.join(guild.id, connect).await;
        // Throw if join failed
        success?;
        handler
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

    let (track, track_handle) = songbird::create_player(input);
    {
        let mut handler = handler.lock().await;
        handler.enqueue(track);
        let queue = handler.queue();
    }

    let duration = metadata.duration.unwrap().as_secs();
    let seconds = duration % 60;
    let minutes = (duration / 60) % 60;
    let hours = (duration / 60) / 60;

    let duration = if hours != 0 {
        format!("{:0>2}:{:0>2}:{:0>2} hours", hours, minutes, seconds)
    } else if minutes != 0 {
        format!("{:0>2}:{:0>2} minutes", minutes, seconds)
    } else {
        format!("{} seconds", seconds)
    };

    if let Metadata {
        title: Some(title),
        thumbnail: Some(thumbnail),
        source_url: Some(source_url),
        artist: Some(artist),
        date: Some(date),
        ..
    } = *metadata
    {
        ctx.send(|f| {
            f.embed(|f| {
                f.title(title)
                    .thumbnail(thumbnail)
                    .author(|f| {
                        f.icon_url(ctx.author().avatar_url().unwrap())
                            .name(ctx.author().name.clone())
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
                    ])
            })
        })
        .await?;
    }

    Ok(())
}
