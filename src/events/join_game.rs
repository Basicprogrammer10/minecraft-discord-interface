use super::Event;
use crate::{Player, Response, PLAYERS};
use regex::Captures;

pub struct JoinGame;

impl Event for JoinGame {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) joined the game"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let name = regex.get(1).unwrap().as_str();

        // Add player to global playerlist
        add_player(name.to_owned());

        println!("[🧑] `{}` joined the game", name);
        Response::new()
            .text(format!(":green_circle: **{}** joined the game", name))
            .refresh_data()
    }
}

fn add_player(name: String) {
    let mut players = PLAYERS.lock();

    // Check if player is in playerlist already
    if let Some(i) = players.iter_mut().find(|x| x.name == name) {
        // It player is a bot and currently offline
        // Set to online
        if !i.online && i.bot {
            i.online = true;
        }
        return;
    }

    // Add player normally
    players.push(Player::new(name.to_owned()));
}
