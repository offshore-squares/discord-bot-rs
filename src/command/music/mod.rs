use crate::{Context, Error};

pub mod join;
pub mod leave;
pub mod play;

/// Music commands
#[poise::command(slash_command, subcommands("join::join", "leave::leave", "play::play"))]
pub async fn music(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!("Can not be called by a slash command, use the subcommands")
}
