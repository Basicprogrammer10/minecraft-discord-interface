use super::Event;
use crate::Response;
use regex::Captures;

pub struct ServerStart;

impl Event for ServerStart {
    fn regex(&self) -> &'static str {
        r"\[.*\]: Done \((.*)\)!"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let time = regex.get(1).unwrap().as_str();

        println!("[🌠] Server Started ({})", time);
        Response::new()
            .text(format!(":sparkles: Server started ({})", time))
            .refresh_data()
    }
}
