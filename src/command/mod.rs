use crate::{Data, Error};
use poise::Command;

mod age;
mod join;

pub fn commands() -> Vec<Command<Data, Error>> {
    return vec![
        age::age(),
        join::join(),
    ];
}
