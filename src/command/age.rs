use crate::{Context, Error};
use poise::serenity_prelude as serenity;

/// Displays your or another user's account creation date
#[command(slash_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!(
        "{}'s account was created at {}",
        user.name,
        user.created_at()
    );
    ctx.say(response).await?;
    Ok(())
}
