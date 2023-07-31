use crate::{
    command,
    util::{self},
};
use poise::serenity_prelude::*;
use songbird::{EventContext, EventHandler as SongbirdEvent};

pub struct ClientHandler;

#[async_trait]
impl EventHandler for ClientHandler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is ready!", ready.user.name);
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
        let guild = ctx.cache.guild(_new.guild_id.unwrap()).unwrap();
        if let Some(old) = _old {
            if check_alone(&guild, old.channel_id.unwrap(), ctx.cache.current_user_id()) {
                let manager = util::manager::get_manager_serenity(&ctx).await;
                command::music::leave::leave_channel(manager, guild).await;
            }
        }
    }
}

pub struct TrackEndHandler {
    pub guild: Guild,
    pub context: Context,
    pub author: User,
    pub channel_id: ChannelId,
}

#[async_trait]
impl SongbirdEvent for TrackEndHandler {
    async fn act(&self, ctx: &songbird::EventContext<'_>) -> Option<songbird::Event> {
        if let EventContext::Track(track_list) = ctx {
            let (manager, _) = util::manager::get_manager(&self.guild, &self.author, &self.context)
                .await
                .unwrap();
            if track_list.len() == 0 {
                command::music::leave::leave_channel(manager, self.guild.clone()).await;
            } else {
                let handler_lock = manager.get(self.guild.id).unwrap();
                let handler = handler_lock.lock().await;
                let queue = handler.queue();

                self.channel_id
                    .send_message(self.context.http(), |e| {
                        e.embed(|f| {
                            command::music::play::send_music_embed(
                                Box::new(queue.current().unwrap().metadata().clone().to_owned()),
                                f,
                                &self.author,
                            );
                            f
                        })
                    })
                    .await
                    .unwrap();
            }
        }
        None
    }
}

fn check_alone(guild: &Guild, channel_id: ChannelId, bot_id: UserId) -> bool {
    let mut states = guild.voice_states.values();

    !states.any(|vs| match vs.channel_id {
        Some(c_id) => channel_id.0 == c_id.0 && vs.user_id.0 != bot_id.0,
        None => false,
    })
}
