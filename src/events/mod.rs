use regex::{Captures, Regex};

use crate::DiscordEvent;

pub mod server_crash;
mod server_start;
pub mod server_stop;

pub trait Event {
    fn regex(&self) -> &'static str;
    fn execute(&self, line: &str, regex: Captures) -> Option<DiscordEvent>;
}

pub trait InternalEvent {
    fn execute(&self) -> Option<DiscordEvent>;
}

pub fn mass_init_regex(events: Vec<Box<dyn Event>>) -> Vec<(Regex, Box<dyn Event>)> {
    events
        .into_iter()
        .map(|x| (Regex::new(x.regex()).unwrap(), x))
        .collect()
}

pub fn base_events() -> Vec<Box<dyn Event>> {
    vec![Box::new(server_start::ServerStart)]
}
