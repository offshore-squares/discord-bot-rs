use crate::{Context, Error};

/// Skip the current song
#[command(slash_command)]
pub async fn skip(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    // Gets voice channel of user
    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    // Checks if user is in said voice channel
    match voice_channel {
        None => {
            ctx.say("You are not in a voice channel, baka").await?;
        },
        _ => {}
    };

    // Get manager for voice channel
    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    if let Some(handler) = manager.get(guild.id) {
        let handler = handler.lock().await;
        handler.queue().skip()?;
        ctx.say("skipped").await?;
    } else {
        ctx.say("not playing").await?;
    }

    Ok(())
}
