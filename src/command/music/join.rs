use crate::{Context, Error};

/// Join's your voice call
#[command(slash_command)]
pub async fn join(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let voice_channel = guild
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect = match voice_channel {
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

    let _owo = manager.join(guild.id, connect).await;

    info!("Join command completed");
    ctx.defer_ephemeral().await?;
    ctx.say(">~<").await?;
    Ok(())
}

// let id = ctx.guild_id();
// if id.is_none() {
//     return;
// }
// let id = id.unwrap();

// if let Some(id) = ctx.guild_id() {
//     id;
// } else {
//     // No id
// }

// match ctx.guild_id() {
//     Some(id) => {

//     }
//     None => {
//         ctx.say("fucxk ytou").await?;
//     }
// }
