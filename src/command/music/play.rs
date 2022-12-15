use std::vec;

use poise::serenity_prelude::Color;
use songbird::input::Metadata;

use crate::{Context, Error};

/// Plays your song => with url
#[command(slash_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Search tag"] search_query: String,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let _connect = match voice_channel {
        Some(channel) => channel,
        None => {
            ctx.say("You are not in a voice channel, baka").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    ctx.defer().await?;
    let input = songbird::input::ytdl_search(search_query).await?;
    let metadata = input.metadata.clone();

    //info!("{:?}", metadata);

    //TODO handeling
    let handler = manager.get(guild.id).unwrap();
    let track = songbird::create_player(input);
    handler.lock().await.play(track.0);
    //OWo

    let seconds = metadata.clone().duration.unwrap().as_secs() % 60;
    let minutes = (metadata.clone().duration.unwrap().as_secs() / 60) % 60;
    let hours = (metadata.clone().duration.unwrap().as_secs() / 60) / 60;

    let duration = if hours != 0 {
        format!("{}:{}:{} hours", hours, minutes, seconds)
    } else if minutes != 0 {
        format!("{}:{} minutes", minutes, seconds)
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
    } = *metadata.clone()
    {
        ctx.send(|f| {
            f.embed(|f| {
                f.title(format!("{}", title))
                    .thumbnail(format!("{}", thumbnail))
                    .author(|f| {
                        f.icon_url(format!("{}", ctx.author().avatar_url().unwrap()))
                            .name(ctx.author().name.clone())
                    })
                    .color(Color::from_rgb(0, 128, 128))
                    .url(format!("{}", source_url))
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
