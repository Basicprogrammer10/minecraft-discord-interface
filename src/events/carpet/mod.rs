use super::Event;

mod bot_join;

pub fn events() -> Vec<Box<dyn Event + Send + Sync>> {
    vec![Box::new(bot_join::BotJoin)]
}
