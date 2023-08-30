use std::{collections::HashMap, sync::Arc};

use poise::serenity_prelude::GuildId;
use songbird::input::Metadata;
use tokio::sync::{Mutex, MutexGuard};

pub struct QueueMap(Arc<Mutex<HashMap<GuildId, Vec<Metadata>>>>);

impl QueueMap {
    pub async fn get_queue_map(&self) -> MutexGuard<'_, HashMap<GuildId, Vec<Metadata>>> {
        self.0.lock().await
    }

    pub fn new() -> Self {
        Self(Arc::new(Mutex::new(HashMap::new())))
    }
}

pub trait GetQueueByGuildId {
    fn get_queue_by_id(&mut self, guild_id: GuildId) -> &mut Vec<Metadata>;
}

impl GetQueueByGuildId for HashMap<GuildId, Vec<Metadata>> {
    fn get_queue_by_id(&mut self, guild_id: GuildId) -> &mut Vec<Metadata> {
        self.entry(guild_id).or_insert(vec![])
    }
}
