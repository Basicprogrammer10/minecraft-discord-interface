use super::InternalEvent;

use crate::{Response, SERVER_ON};

pub struct ServerCrash(pub i32);

impl InternalEvent for ServerCrash {
    fn name(&self) -> &'static str {
        "server_crash"
    }

    fn execute(&self) -> Response {
        println!("[🔥] Server Crashed ({})", self.0);

        // Tell the rest of the system that the server has stoped
        *SERVER_ON.write() = false;

        Response::new()
            .discord_text(format!(":fire: Server crahsed ({})", self.0))
            .discord_stop_data()
            .async_exit()
    }
}
