use crate::{Data, Error};
use poise::Command;

mod age;
mod music;

pub fn commands() -> Vec<Command<Data, Error>> {
    return vec![age::age(), music::music()];
}
