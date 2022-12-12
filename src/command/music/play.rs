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

    let input = songbird::input::ytdl_search(search_query).await?;
    let metadata = input.metadata.clone();

    info!("{:?}", metadata);

    ctx.say("nggh").await?;
    match metadata.title {
        Some(title) => ctx.say(format!("Speelt nu: {:?}", title)).await.unwrap(),
        None => {
            ctx.say("Geen titel gevonden").await.unwrap();
            return Ok(());
        }
    };

    //TODO handeling
    let handler = manager.get(guild.id).unwrap();
    let track = songbird::create_player(input);
    handler.lock().await.play(track.0);

    Ok(())
}
