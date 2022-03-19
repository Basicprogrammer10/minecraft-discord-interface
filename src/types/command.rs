use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::config::Config;
use super::response::Response;

#[async_trait]
pub trait Command {
    fn name(&self) -> &'static str;
    fn help(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn needs_server(&self) -> bool {
        false
    }
    async fn execute(&self, cfg: &Config, ctx: Context, msg: Message) -> Response;
}
