use super::InternalEvent;
use crate::Response;

pub struct ServerStop;

impl InternalEvent for ServerStop {
    fn execute(&self) -> Response {
        println!("[❌] Server Stoped");
        Response::new()
            .discord_text(":x: Server stoped")
            .discord_stop_data()
            .async_exit()
    }
}
