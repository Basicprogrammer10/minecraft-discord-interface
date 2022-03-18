use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::{command_parts, COMMANDS};
use crate::{Command, Config, Response};

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
        "Get a list of ll commands or get specific deatils on one"
    }

    async fn execute(&self, cfg: &Config, ctx: Context, msg: Message) -> Response {
        let cmd = command_parts(&msg.content, &cfg.bot.command_prefix);

        if cmd.len() == 1 {
            let mut help = String::from("\u{200b}");

            for i in COMMANDS.iter() {
                help.push_str(i.help());
                help.push('\n');
            }

            msg.channel_id
                .send_message(ctx, |x| {
                    x.embed(|e| {
                        e.title(format!("Commands [{}]", COMMANDS.len()))
                            .description(format!("```\n{}```", help))
                            .color(0x09BC8A)
                    })
                })
                .await
                .unwrap();

            return Response::new();
        }

        Response::new()
    }
}
