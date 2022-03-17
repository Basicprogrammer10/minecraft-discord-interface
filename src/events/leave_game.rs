use super::Event;
use crate::{DiscordEvent, PLAYERS};
use regex::Captures;

pub struct LeaveGame;

impl Event for LeaveGame {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) left the game"
    }

    fn execute(&self, _line: &str, regex: Captures) -> DiscordEvent {
        let name = regex.get(1).unwrap().as_str();

        // Remove player from global playerlist
        PLAYERS.lock().unwrap().retain(|x| *x != name);

        println!("[🧑] `{}` left the game", name);
        DiscordEvent::new()
            .text(format!(":red_circle: `{}` left the game", name))
            .refresh_data()
    }
}
