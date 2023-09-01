use poise::{
    serenity_prelude::{ButtonStyle, Color, CreateButton, ReactionType},
    CreateReply,
};
use std::time::Duration;

use crate::{
    model::queue::{GetQueueByGuildId, Song},
    Context, Error,
};

/// Present song in queue
#[command(slash_command)]
pub async fn queue(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();

    let data = ctx.serenity_context().data.read().await;
    let data = data.get::<crate::DataKey>().unwrap();
    let mut queue_map = data.queue_map.get_queue_map().await;
    let queue = queue_map.get_queue_by_id(guild.id);
    let mut page = 0;

    if queue.len() > 0 {
        let message = ctx
            .send(|reply| {
                create_embed(page, queue, reply);
                reply
            })
            .await?;

        // TODO might need different wait interaction
        let interaction = message
            .message()
            .await
            .unwrap()
            .await_component_interaction(&ctx)
            .timeout(Duration::from_secs(60 * 3));

        let button_pressed = &interaction.await.unwrap().data.custom_id;

        if button_pressed == "start" {
            page = 0;
        } else if button_pressed == "backward" {
            page -= 1;
        } else if button_pressed == "forward" {
            page += 1;
        } else {
            page = queue.len().div_ceil(10);
        }

        message
            .edit(ctx, |reply| {
                create_embed(page, queue, reply);
                reply
            })
            .await?;
    } else {
        ctx.defer_ephemeral().await?;
        ctx.say("Cannot list an empty queue").await?;
    }

    Ok(())
}

fn create_button(emoji: &str, id: &str) -> CreateButton {
    let mut b = CreateButton::default();
    b.custom_id(id);
    // To add an emoji to buttons, use .emoji(). The method accepts anything ReactionType or
    // anything that can be converted to it. For a list of that, search Trait Implementations in the
    // docs for From<...>.
    b.emoji(ReactionType::Unicode(emoji.to_string()));
    b.style(ButtonStyle::Primary);
    b
}

fn create_embed(page: usize, queue: &mut Vec<Song>, builder: &mut CreateReply<'_>) {
    // TODO not disabled
    let forward = create_button("▶️", "forward")
        .disabled(page == queue.len().div_ceil(10) && queue.len() > 10)
        .clone();
    let backwards = create_button("◀️", "backward").disabled(page == 0).clone();
    let start = create_button("⏮️", "start");
    let end = create_button("⏭️", "end");

    {
        builder
            .embed(|embed| {
                embed
                    .title(format!("Music Queue - Page {}", page + 1))
                    .color(Color::from_rgb(120, 60, 22))
                    .description(
                        queue
                            .iter()
                            .enumerate()
                            .map(|(i, val)| {
                                format!(
                                    "{} - {} \n",
                                    ((page * 10) + i) + 1,
                                    val.metadata.artist.as_ref().unwrap()
                                )
                            })
                            .collect::<String>(),
                    )
            })
            .components(|create_component| {
                create_component.create_action_row(|row| {
                    row.add_button(start)
                        .add_button(backwards)
                        .add_button(forward)
                        .add_button(end)
                })
            });
    }
}
