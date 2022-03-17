use super::Event;
use crate::{DiscordEvent, PLAYERS};
use regex::Captures;

pub struct JoinGame;

impl Event for JoinGame {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) joined the game"
    }

    fn execute(&self, _line: &str, regex: Captures) -> DiscordEvent {
        let name = regex.get(1).unwrap().as_str();

        // Add player to global playerlist
        PLAYERS.lock().unwrap().push(name.to_owned());

        println!("[🧑] `{}` joined the game", name);
        DiscordEvent::new().text(format!(":green_circle: `{}` joined the game", name))
    }
}
