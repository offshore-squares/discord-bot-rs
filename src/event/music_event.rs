use crate::{
    command,
    model::queue::GetQueueByGuildId,
    util::{
        self,
        music::{search, send_music_embed},
    },
};
use poise::serenity_prelude::*;
use songbird::{EventContext, EventHandler as SongbirdEvent};
use tokio::sync::RwLockReadGuard;

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
                command::music::leave::leave_channel(manager, guild.clone()).await;
                let data = ctx.data.read().await;
                clear_queue(data, guild.id).await;
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
        if let EventContext::Track(_track_list) = ctx {
            let volume = _track_list[0].0.volume;
            let (manager, _) = util::manager::get_manager(&self.guild, &self.author, &self.context)
                .await
                .unwrap();
            let data = self.context.data.read().await;
            let data = data.get::<crate::DataKey>().unwrap();
            let mut queue_map = data.queue_map.get_queue_map().await;
            let queue = queue_map.get_queue_by_id(self.guild.id);

            if queue.len() == 0 {
                command::music::leave::leave_channel(manager, self.guild.clone()).await;
                queue.clear();
            } else {
                let handler_lock = manager.get(self.guild.id).unwrap();
                let mut handler = handler_lock.lock().await;
                let next_song = queue.remove(0);

                // If Queue is bigger than 1
                if queue.len() >= 1 {
                    // TODO maybe make embed now looks derpy
                    // Get song from source
                    let input = search(format!(
                        "{} - {}",
                        next_song.title.as_ref().unwrap(),
                        next_song.artist.as_ref().unwrap()
                    ))
                    .await
                    .unwrap();

                    // Get metadata
                    let metadata = input.metadata.clone();
                    let (mut track, _) = songbird::create_player(input);
                    {
                        // Set song volume to ended song volume
                        track.set_volume(volume);
                        handler.enqueue(track);
                        self.channel_id
                            .send_message(self.context.http(), |e| {
                                e.embed(|f| {
                                    send_music_embed(metadata, f, &self.author);
                                    f
                                })
                            })
                            .await
                            .unwrap();
                    }
                }
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

async fn clear_queue(data: RwLockReadGuard<'_, TypeMap>, guild_id: GuildId) {
    let data = data.get::<crate::DataKey>().unwrap();
    let mut queue_map = data.queue_map.get_queue_map().await;
    let queue = queue_map.get_queue_by_id(guild_id);
    queue.clear();
}
