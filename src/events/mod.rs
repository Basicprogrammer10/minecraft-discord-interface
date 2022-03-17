use regex::{Captures, Regex};

use crate::DiscordEvent;

mod join_game;
mod leave_game;
pub mod server_crash;
mod server_start;
pub mod server_stop;

pub trait Event {
    fn regex(&self) -> &'static str;
    fn execute(&self, line: &str, regex: Captures) -> DiscordEvent;
}

pub trait InternalEvent {
    fn execute(&self) -> DiscordEvent;
}

pub fn mass_init_regex(events: Vec<Box<dyn Event>>) -> Vec<(Regex, Box<dyn Event>)> {
    events
        .into_iter()
        .map(|x| (Regex::new(x.regex()).unwrap(), x))
        .collect()
}

pub fn base_events() -> Vec<Box<dyn Event>> {
    vec![
        Box::new(server_start::ServerStart),
        Box::new(join_game::JoinGame),
        Box::new(leave_game::LeaveGame),
    ]
}
