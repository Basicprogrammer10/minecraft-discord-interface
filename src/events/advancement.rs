use super::Event;
use crate::Response;
use regex::Captures;

pub struct Advancement;

impl Event for Advancement {
    fn regex(&self) -> &'static str {
        r"\[.*\]: (.*) has (made|completed) the (advancement|challenge) \[(.*)\]"
    }

    fn execute(&self, _line: &str, regex: Captures) -> Response {
        let name = regex.get(1).unwrap().as_str();
        let thing = regex.get(3).unwrap().as_str();
        let advancement = regex.get(4).unwrap().as_str();

        println!("[📀] `{}` completed the {} `{}`", name, thing, advancement);
        Response::new().text(format!(
            ":dvd: **{}** has completed the {} **{}**",
            name, thing, advancement
        ))
    }
}
