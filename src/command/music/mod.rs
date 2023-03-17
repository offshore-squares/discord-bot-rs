use crate::{Context, Error};

mod join;
mod leave;
mod play;
mod skip;

/// Music commands
#[poise::command(slash_command, subcommands("join::join", "leave::leave", "play::play", "skip::skip"))]
pub async fn music(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!("Can not be called by a slash command, use the subcommands")
}
