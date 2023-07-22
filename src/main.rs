use std::env::{self, current_dir};
use std::path::PathBuf;

use poise::serenity_prelude::{self as serenity, Activity};
use poise::{Framework, FrameworkOptions};
use songbird::SerenityInit;

mod command;
mod event;
mod util;

#[macro_use]
extern crate poise;
#[macro_use]
extern crate log;

pub struct Data {}
// Box because we don't know how big the error, send, sync are at compile time
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ApplicationContext<'a> = poise::ApplicationContext<'a, Data, Error>;

#[tokio::main]
async fn main() {
    init_path().await.unwrap();
    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: command::commands(),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_KEY").expect("No mo token bitch"))
        .intents(serenity::GatewayIntents::non_privileged())
        .client_settings(|client_builder| client_builder.register_songbird())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_in_guild(
                    ctx,
                    &framework.options().commands,
                    serenity::GuildId(1049757728647680010),
                )
                .await?;
                framework
                    .client()
                    .ctx
                    .set_presence(
                        Some(Activity::playing(
                            std::env::var("PRESENCE").unwrap_or("oop~".to_string()),
                        )),
                        serenity::OnlineStatus::DoNotDisturb,
                    )
                    .await;
                Ok(Data {})
            })
        });

    info!("Running");
    framework.run().await.unwrap();
}

/// Init Path with logger
async fn init_path() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::var("PATH").unwrap();
    let current_dir = current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();

    // set ytdlp to Path env
    let mut ytdlp_path = PathBuf::new();
    ytdlp_path.push(current_dir);
    ytdlp_path.push("lib");

    #[cfg(windows)]
    std::env::set_var("PATH", path + ";" + ytdlp_path.to_str().unwrap());
    #[cfg(unix)]
    std::env::set_var("PATH", path + ";" + ytdlp_path.to_str().unwrap());

    dotenvy::dotenv().ok();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

    info!("YTDLP path: {:?}", ytdlp_path.to_str().unwrap());
    info!("PATH is {:?}", std::env::var("PATH"));

    Ok(())
}
