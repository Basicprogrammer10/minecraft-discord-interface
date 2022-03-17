use super::InternalEvent;
use crate::DiscordEvent;

pub struct ServerCrash(pub i32);

impl InternalEvent for ServerCrash {
    fn execute(&self) -> Option<DiscordEvent> {
        println!("[🔥] Server Crashed ({})", self.0);
        Some(DiscordEvent::ExitText(format!(
            ":fire: Server crahsed ({})",
            self.0
        )))
    }
}
