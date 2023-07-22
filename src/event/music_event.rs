use poise::{
    serenity_prelude::{ChannelId, Guild, UserId},
    Context,
};
use songbird::{Event, EventContext, EventHandler};

use crate::{command, util, Data, Error};

struct ChannelUserNotifier<'a> {
    context: Context<'a, Data, Error>,
}

#[async_trait]
impl EventHandler for ChannelUserNotifier<'_> {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::ClientDisconnect(_) = ctx {
            let (guild, _, voice_channel) = util::manager::get_manager(self.context).await.unwrap();

            let uwu = self.context.serenity_context();
            if check_alone(&guild, voice_channel, uwu.cache.current_user_id()) {
                command::music::leave::leave();
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
