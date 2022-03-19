use super::InternalEvent;

use crate::{Response, SERVER_ON};

pub struct ServerStop;

impl InternalEvent for ServerStop {
    fn execute(&self) -> Response {
        println!("[❌] Server Stoped");

        // Tell the rest of the system that the server has stoped
        *SERVER_ON.write() = false;

        Response::new()
            .discord_text(":x: Server stoped")
            .discord_stop_data()
            .async_exit()
    }
}
