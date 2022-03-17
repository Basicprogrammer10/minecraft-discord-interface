use super::Event;
use crate::DiscordEvent;
use regex::Captures;

pub struct ChatMessage;

impl Event for ChatMessage {
    fn regex(&self) -> &'static str {
        r"\[.*\]: <(.*)> (.*)"
    }

    fn execute(&self, _line: &str, regex: Captures) -> DiscordEvent {
        let name = regex.get(1).unwrap().as_str();
        let message = regex.get(2).unwrap().as_str();

        println!("[🎹] `{}` said `{}`", name, message);
        DiscordEvent::new().text(format!(":speech_left: **{}** » {}", name, message))
    }
}
