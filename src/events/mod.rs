use regex::{Captures, Regex};

mod server_start;

pub trait Event {
    fn regex(&self) -> &'static str;
    fn execute(&self, line: &str, regex: Captures) -> Option<String>;
}

pub fn mass_init_regex(events: Vec<Box<dyn Event>>) -> Vec<(Regex, Box<dyn Event>)> {
    events
        .into_iter()
        .map(|x| (Regex::new(x.regex()).unwrap(), x))
        .collect()
}

pub fn base_events() -> Vec<Box<dyn Event>> {
    vec![Box::new(server_start::ServerStart)]
}
