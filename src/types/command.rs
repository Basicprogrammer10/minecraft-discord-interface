use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::response::Response;

#[async_trait]
pub trait Command {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn description(&self) -> &'static str;
    async fn execute(&self, cmd: Vec<&str>, ctx: Context, msg: Message) -> Response;
}
