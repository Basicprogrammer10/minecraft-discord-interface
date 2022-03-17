use super::Event;
use crate::DiscordEvent;
use regex::Captures;

pub struct ServerStart;

impl Event for ServerStart {
    fn regex(&self) -> &'static str {
        r"\[.*\]: Done \((.*)\)!"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Option<DiscordEvent> {
        let time = regex.get(1).unwrap().as_str();

        println!("[🌠] Server Started ({})", time);
        Some(DiscordEvent::Text(format!(
            ":sparkles: Server started ({})",
            time
        )))
    }
}
