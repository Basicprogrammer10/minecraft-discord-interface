use super::InternalEvent;
use crate::DiscordEvent;

pub struct ServerStop;

impl InternalEvent for ServerStop {
    fn execute(&self) -> DiscordEvent {
        println!("[❌] Server Stoped");
        DiscordEvent::new().text(":x: Server stoped").exit()
    }
}
