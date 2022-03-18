use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::{
    super::{colors, command_parts},
    COMMANDS,
};
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
        "Get a list of all commands or get specific deatils on one"
    }

    async fn execute(&self, cfg: &Config, ctx: Context, msg: Message) -> Response {
        let cmd = command_parts(&msg.content, &cfg.bot.command_prefix);

        if cmd.len() <= 1 {
            let mut help = String::from("\u{200b}");

            for i in COMMANDS.iter() {
                help.push_str(&cfg.bot.command_prefix);
                help.push_str(i.help());
                help.push('\n');
            }

            msg.channel_id
                .send_message(ctx, |x| {
                    x.embed(|e| {
                        e.title(format!("Commands [{}]", COMMANDS.len()))
                            .description(format!("```\n{}```", help))
                            .color(colors::GREEEN)
                    })
                })
                .await
                .unwrap();

            return Response::new();
        }

        let cmd = &cmd[1].to_lowercase();
        if let Some(cmd) = COMMANDS.iter().find(|x| x.name() == cmd) {
            msg.channel_id
                .send_message(ctx, |x| {
                    x.embed(|e| {
                        e.title(format!("Help: {}", cmd.name()))
                            .description(format!(
                                "{}\nUsage: `{}{}`",
                                cmd.description(),
                                cfg.bot.command_prefix,
                                cmd.help()
                            ))
                            .color(colors::GREEEN)
                    })
                })
                .await
                .unwrap();
        }

        Response::new()
    }
}
