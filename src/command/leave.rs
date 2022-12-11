use crate::{Context, Error};

/// Bot will join your call
#[command(slash_command)]
pub async fn leave(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    match voice_channel {
        Some(channel) => channel,
        None => {
            ctx.say("You are not in a voice channel, poopoo").await?;
            return Ok(());
        }
    };

    let manager = songbird::get(ctx.serenity_context())
        .await
        .expect("loaded")
        .clone();

    let _owo = manager.leave(guild.id).await;

    info!("leave command completed");
    ctx.defer_ephemeral().await?;
    ctx.say("I left you forever").await?;
    Ok(())
}
