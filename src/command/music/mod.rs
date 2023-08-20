use crate::{Context, Error};

pub mod join;
pub mod leave;
pub mod play;
pub mod shuffle;
pub mod skip;

/// Music commands
#[poise::command(
    slash_command,
    guild_only,
    subcommands(
        "join::join",
        "leave::leave",
        "play::play",
        "skip::skip",
        "shuffle::shuffle"
    )
)]
pub async fn music(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!("Can not be called by a slash command, use the subcommands")
}
