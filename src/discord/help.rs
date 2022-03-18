use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::COMMANDS;
use crate::{Command, Response};

pub struct Help;

#[async_trait]
impl Command for Help {
    fn name(&self) -> &'static str {
        "help"
    }

    fn help(&self) -> &'static str {
        "help [command]"
    }

    fn description(&self) -> &'static str {
        "Get a list of ll commands or get more deatils on one"
    }

    async fn execute(&self, _cmd: Vec<&str>, _ctx: Context, _msg: Message) -> Response {
        let mut help = String::from("\u{200b}");

        for i in COMMANDS.iter() {
            help.push_str(i.help());
            help.push('\n');
        }

        // TODO: this

        Response::new()
    }
}
