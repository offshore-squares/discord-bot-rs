use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions};

mod command;

#[macro_use]
extern crate poise;

pub struct Data {}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let framework = Framework::builder()
        .options(FrameworkOptions {
            commands: command::exports(),
            ..Default::default()
        })
        .token(std::env::var("DISCORD_KEY").expect("No mo token bitch"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}
