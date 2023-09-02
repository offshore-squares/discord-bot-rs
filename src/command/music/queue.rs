use crate::{poise::futures_util::StreamExt, util::music::format_duration};
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

    let is_disabled = !(queue.len() > 10) && (page + 1 >= queue.len().div_ceil(10));
    let forward: CreateButton = create_button("▶️", "forward").disabled(is_disabled).clone();
    let backwards = create_button("◀️", "backward").disabled(page == 0).clone();
    let start = create_button("⏮️", "start");
    let end = create_button("⏭️", "end");
    let buttons = vec![start, backwards, forward, end];

    if queue.len() > 0 {
        let message = ctx
            .send(|reply| {
                create_embed(page, queue, buttons.clone(), reply);
                reply
            })
            .await?;

        let mut interactions = message
            .message()
            .await
            .unwrap()
            .await_component_interactions(&ctx)
            .timeout(Duration::from_secs(60 * 3))
            .build();

        while let Some(interaction) = interactions.next().await {
            let button_pressed = interaction.data.custom_id.clone();
            if button_pressed == "start" {
                page = 0;
            } else if button_pressed == "backward" {
                page -= 1;
            } else if button_pressed == "forward" {
                page += 1;
            } else {
                page = if queue.len() > 10 {
                    queue.len().div_ceil(10)
                } else {
                    0
                };
            }

            interaction
                .create_interaction_response(&ctx.serenity_context().http, |f| {
                    f.kind(poise::serenity_prelude::InteractionResponseType::UpdateMessage)
                        .interaction_response_data(|f| {
                            f.embed(|f| {
                                f.title(format!("Music Queue - Page {}", page + 1))
                                    .color(Color::from_rgb(120, 60, 22))
                                    .description(
                                        queue
                                            .iter()
                                            .enumerate()
                                            .map(|(i, val)| {
                                                format!(
                                                    "{}. {} - {} \n",
                                                    ((page * 10) + i) + 1,
                                                    format_duration(
                                                        val.metadata.duration.unwrap().as_secs()
                                                    ),
                                                    val.metadata.title.as_ref().unwrap()
                                                )
                                            })
                                            .collect::<String>(),
                                    )
                            })
                            .components(|create_component| {
                                create_component.create_action_row(|row| {
                                    buttons.iter().for_each(|button| {
                                        row.add_button(button.clone());
                                    });
                                    row
                                })
                            })
                        })
                })
                .await?;
        }
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

//TODO make it reuseable
fn create_embed(
    page: usize,
    queue: &mut Vec<Song>,
    buttons: Vec<CreateButton>,
    builder: &mut CreateReply<'_>,
) {
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
                                    "{}. {} - {} \n",
                                    ((page * 10) + i) + 1,
                                    format_duration(val.metadata.duration.unwrap().as_secs()),
                                    val.metadata.title.as_ref().unwrap()
                                )
                            })
                            .collect::<String>(),
                    )
            })
            .components(|create_component| {
                create_component.create_action_row(|row| {
                    buttons.iter().for_each(|button| {
                        row.add_button(button.clone());
                    });
                    row
                })
            });
    }
}
