use crate::{Context, Error};

/// Plays your song => with url
#[command(slash_command)]
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
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

    let handler = manager.get(guild.id).unwrap();
    info!("got handler");

    let track = songbird::create_player(songbird::input::ytdl("https://www.youtube.com/watch?v=e4dWh0gKjMI").await?);
    handler.lock().await.play(track.0);

    Ok(())
}
