use super::InternalEvent;
use crate::DiscordEvent;

pub struct ServerCrash(pub i32);

impl InternalEvent for ServerCrash {
    fn execute(&self) -> DiscordEvent {
        println!("[🔥] Server Crashed ({})", self.0);
        DiscordEvent::new()
            .text(format!(":fire: Server crahsed ({})", self.0))
            .stop_data()
            .exit()
    }
}
