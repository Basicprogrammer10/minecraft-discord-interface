use super::Event;
use regex::Captures;

use crate::{Response, SERVER_ON};

pub struct ServerStart;

impl Event for ServerStart {
    fn name(&self) -> &'static str {
        "server_start"
    }

    fn regex(&self) -> &'static str {
        r"\[.*\]: Done \((.*)\)!"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let time = regex.get(1).unwrap().as_str();

        // Tell the rest of the system that the server is running
        *SERVER_ON.write() = true;

        println!("[🌠] Server Started ({})", time);
        Response::new()
            .discord_text(format!(":sparkles: Server started ({})", time))
            .discord_refresh_data()
    }
}
