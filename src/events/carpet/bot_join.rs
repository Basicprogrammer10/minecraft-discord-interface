use super::Event;
use crate::{DiscordEvent, Player, PLAYERS};
use regex::Captures;

pub struct BotJoin;

impl Event for BotJoin {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*)\[local\] logged in"
    }

    fn execute(&self, _line: &str, regex: Captures) -> DiscordEvent {
        let name = regex.get(1).unwrap().as_str();

        // Add player as an offline bot
        PLAYERS.lock().push(Player {
            name: name.to_owned(),
            online: false,
            bot: true,
        });

        DiscordEvent::new()
    }
}
