use super::InternalEvent;
use crate::Response;

pub struct ServerStop;

impl InternalEvent for ServerStop {
    fn execute(&self) -> Response {
        println!("[❌] Server Stoped");
        Response::new().text(":x: Server stoped").stop_data().exit()
    }
}
