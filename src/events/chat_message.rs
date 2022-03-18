use super::Event;
use crate::Response;
use regex::Captures;

pub struct ChatMessage;

impl Event for ChatMessage {
    fn regex(&self) -> &'static str {
        r"\[.*\]: <(.*)> (.*)"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let name = regex.get(1).unwrap().as_str();
        let message = regex.get(2).unwrap().as_str();

        println!("[🎹] `{}` said `{}`", name, message);
        Response::new().text(format!(":speech_left: **{}** » {}", name, message))
    }
}
