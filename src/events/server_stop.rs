use super::InternalEvent;
use crate::DiscordEvent;

pub struct ServerStop;

impl InternalEvent for ServerStop {
    fn execute(&self) -> Option<DiscordEvent> {
        println!("[❌] Server Stoped");
        Some(DiscordEvent::ExitText(format!(":x: Server stoped")))
    }
}
