use super::Event;
use crate::{Response, PLAYERS};
use regex::Captures;

pub struct LeaveGame;

impl Event for LeaveGame {
    fn name(&self) -> &'static str {
        "leave_game"
    }

    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) left the game"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let name = regex.get(1).unwrap().as_str();

        // Remove player from global playerlist
        PLAYERS.lock().retain(|x| x.name != name);

        println!("[🧑] `{}` left the game", name);
        Response::new()
            .discord_text(format!(":red_circle: **{}** left the game", name))
            .discord_refresh_data()
    }
}
