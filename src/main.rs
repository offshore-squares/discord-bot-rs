use std::env::{self, current_dir};

use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};
use songbird::SerenityInit;

mod command;

#[macro_use]
extern crate poise;
#[macro_use]
extern crate log;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let path = env::var("PATH").unwrap();
    let current_dir = current_dir().unwrap();
    let current_dir = current_dir.to_str().unwrap();
    std::env::set_var("PATH", path + ";" + current_dir + "/lib");
    let _ = dotenvy::dotenv();
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();

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
                Ok(Data {})
            })
        });

    info!("Running");

    framework.run().await.unwrap();
}
