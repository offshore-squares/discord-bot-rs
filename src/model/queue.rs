use std::{collections::HashMap, sync::Arc};

use poise::serenity_prelude::{Context, GuildId, TypeMap};
use songbird::input::Metadata;
use tokio::sync::{Mutex, MutexGuard, RwLockReadGuard};

use crate::Data;

// Used as type for Data inside Context
pub struct QueueMap(Arc<Mutex<HashMap<GuildId, Vec<Metadata>>>>);

// Add constructor and get_queue_map which returns mutex containing HashMap
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

// Get queue if not exist create new hashmap
impl GetQueueByGuildId for HashMap<GuildId, Vec<Metadata>> {
    fn get_queue_by_id(&mut self, guild_id: GuildId) -> &mut Vec<Metadata> {
        self.entry(guild_id).or_insert(vec![])
    }
}

#[async_trait]
pub trait GetDataFromSerenity {
    async fn get_data_lock(&mut self) -> RwLockReadGuard<'async_trait, TypeMap>;
}

#[async_trait]
impl GetDataFromSerenity for Context {
    async fn get_data_lock(&mut self) -> RwLockReadGuard<'async_trait, TypeMap> {
        self.data.read().await
    }
}

pub trait GetDataFromLock {
    fn get_data(&mut self) -> &Data;
}

impl GetDataFromLock for RwLockReadGuard<'_, TypeMap> {
    fn get_data(&mut self) -> &Data {
        self.get::<crate::DataKey>().unwrap()
    }
}
