use crate::{Context, Error};

/// Displays help
#[command(slash_command)]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: ":flushed:",
        ..Default::default()
    };
    poise::builtins::help(ctx, command.as_deref(), config).await?;
    Ok(())
}
