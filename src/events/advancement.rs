use super::Event;
use crate::DiscordEvent;
use regex::Captures;

pub struct Advancement;

impl Event for Advancement {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) has (made|completed) the (advancement|challenge) \[(.*)\]"
    }

    fn execute(&self, _line: &str, regex: Captures) -> DiscordEvent {
        let name = regex.get(1).unwrap().as_str();
        let thing = regex.get(3).unwrap().as_str();
        let advancement = regex.get(4).unwrap().as_str();

        println!("[📀] `{}` completed the {} `{}`", name, thing, advancement);
        DiscordEvent::new().text(format!(
            ":dvd: **{}** has completed the {} **{}**",
            name, thing, advancement
        ))
    }
}
