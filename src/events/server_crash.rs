use super::InternalEvent;
use crate::Response;

pub struct ServerCrash(pub i32);

impl InternalEvent for ServerCrash {
    fn execute(&self) -> Response {
        println!("[🔥] Server Crashed ({})", self.0);
        Response::new()
            .discord_text(format!(":fire: Server crahsed ({})", self.0))
            .discord_stop_data()
            .async_exit()
    }
}
