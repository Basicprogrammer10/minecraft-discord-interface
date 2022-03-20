use regex::{Captures, Regex};

use crate::Response;

mod advancement;
mod carpet;
mod chat_message;
mod join_game;
mod leave_game;
pub mod server_crash;
mod server_start;
pub mod server_stop;

type SafeEvent = Box<dyn Event + Send + Sync>;

pub trait Event {
    fn name(&self) -> &'static str;
    fn regex(&self) -> &'static str;
    fn execute(&self, line: &str, regex: Captures) -> Response;
}

pub trait InternalEvent {
    fn name(&self) -> &'static str;
    fn execute(&self) -> Response;
}

pub fn mass_init_regex(events: Vec<SafeEvent>) -> Vec<(Regex, SafeEvent)> {
    events
        .into_iter()
        .map(|x| (Regex::new(x.regex()).unwrap(), x))
        .collect()
}

pub fn base_events() -> Vec<SafeEvent> {
    let mut events: Vec<SafeEvent> = vec![
        Box::new(server_start::ServerStart),
        Box::new(join_game::JoinGame),
        Box::new(leave_game::LeaveGame),
        Box::new(chat_message::ChatMessage),
        Box::new(advancement::Advancement),
    ];
    events.extend(carpet::events());

    events
}
