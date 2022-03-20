use lazy_static::lazy_static;

use crate::Command;

mod about;
mod help;
mod player;
mod refresh;

lazy_static! {
    pub static ref COMMANDS: Vec<Box<dyn Command + Send + Sync>> = base_commands();
}

pub fn base_commands() -> Vec<Box<dyn Command + Send + Sync>> {
    vec![
        Box::new(about::About),
        Box::new(refresh::Refresh),
        Box::new(help::Help),
        Box::new(player::Player),
    ]
}
