use async_trait::async_trait;
use regex::Captures;

use super::Event;
use crate::{Player, Response, PLAYERS};

pub struct BotJoin;

#[async_trait]
impl Event for BotJoin {
    fn name(&self) -> &'static str {
        "bot_join"
    }

    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*)\[local\] logged in"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let name = regex.get(1).unwrap().as_str();

        // Add player as an offline bot
        PLAYERS.lock().push(Player {
            name: name.to_owned(),
            online: false,
            bot: true,
        });

        Response::new()
    }
}
