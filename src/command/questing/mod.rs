use crate::{Context, Error};

mod start;

/// Questing in Discord!
#[poise::command(slash_command, subcommands("start::start"))]
pub async fn commands(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!("Can not be called by a slash command, use the subcommands")
}
