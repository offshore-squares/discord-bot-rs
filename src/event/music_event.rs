use crate::{command, util};
use poise::serenity_prelude::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn voice_state_update(&self, ctx: Context, _old: Option<VoiceState>, _new: VoiceState) {
        //let guild = ctx.cache.
        let guild = ctx.cache.guild(_new.guild_id.unwrap()).unwrap();
        if let Some(old) = _old {
            if check_alone(&guild, old.channel_id.unwrap(), ctx.cache.current_user_id()) {
                let manager = util::manager::get_manager_serenity(ctx).await;
                command::music::leave::leave_channel(manager, guild).await;
            }
        }
    }
}

fn check_alone(guild: &Guild, channel_id: ChannelId, bot_id: UserId) -> bool {
    let mut states = guild.voice_states.values();

    !states.any(|vs| match vs.channel_id {
        Some(c_id) => channel_id.0 == c_id.0 && vs.user_id.0 != bot_id.0,
        None => false,
    })
}
