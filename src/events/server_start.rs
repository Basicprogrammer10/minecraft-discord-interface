use super::Event;
use regex::Captures;

pub struct ServerStart;

impl Event for ServerStart {
    fn regex(&self) -> &'static str {
        r"\[.*\]: Done \((.*)\)!"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Option<String> {
        let time = regex.get(1).unwrap().as_str();

        println!("[🌠] Server Started ({})", time);
        Some(format!(":sparkles: Server started ({})", time))
    }
}
