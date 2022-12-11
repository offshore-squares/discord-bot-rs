use crate::{Data, Error};
use poise::Command;

mod age;
mod join;
mod leave;
mod play;

pub fn commands() -> Vec<Command<Data, Error>> {
    return vec![age::age(), join::join(), leave::leave(), play::play()];
}
