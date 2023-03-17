use poise::Modal;

use crate::{ApplicationContext, Error};

#[derive(Debug, Modal)]
#[name = "Modal title"] // Struct name by default
struct QuestInit {
    #[name = "Name"]
    #[placeholder = "What is your name?"]
    #[min_length = 1]
    #[max_length = 20]
    name: String,
    #[name = "Race"]
    #[placeholder = "Elf, Sith, catboy, etc."]
    #[min_length = 1]
    #[max_length = 50]
    race: Option<String>,
}

/// Start your quest
///
/// Name, race and starting village
#[command(slash_command)]
pub async fn start(ctx: ApplicationContext<'_>) -> Result<(), Error> {
    let data = QuestInit::execute(ctx).await?;

    println!("{:#?}", data);

    Ok(())
}
