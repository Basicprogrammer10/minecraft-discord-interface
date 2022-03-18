use super::InternalEvent;
use crate::Response;

pub struct ServerCrash(pub i32);

impl InternalEvent for ServerCrash {
    fn execute(&self) -> Response {
        println!("[🔥] Server Crashed ({})", self.0);
        Response::new()
            .text(format!(":fire: Server crahsed ({})", self.0))
            .stop_data()
            .exit()
    }
}
