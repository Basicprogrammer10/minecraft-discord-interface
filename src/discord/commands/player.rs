use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    client::Context,
    model::{
        channel::{Message, ReactionType},
        id::ChannelId,
    },
};

use super::super::{colors, misc::command_parts};
use crate::{Command, Config, Response};

lazy_static! {
    static ref PLAYER_REGEX: Regex = Regex::new(
        r"(.*) (spawn|kill)( at [-\.\d ]+)?( ?facing ([-\.\d ]+))?( ?in minecraft:(overworld|the_end|the_nether))?"
    )
    .unwrap();
}

pub struct Player;

#[async_trait]
impl Command for Player {
    fn name(&self) -> &'static str {
        "player"
    }

    fn help(&self) -> &'static str {
        "player <name> (spawn|kill) [at X X X] [facing X X] [in X]"
    }

    fn description(&self) -> &'static str {
        "Runs carpet player actions on the server"
    }

    async fn execute(&self, cfg: &Config, ctx: Context, msg: Message) -> Response {
        let cmd = command_parts(&msg.content, &cfg.bot.command_prefix);
        let mut cmd_str = cmd
            .iter()
            .skip(1)
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ");

        // Its jank but it works :/
        while cmd_str.contains("  ") {
            cmd_str = cmd_str.replace("  ", " ");
        }

        if let Some(reg) = PLAYER_REGEX.captures(&cmd_str) {
            let name = reg.get(1).unwrap().as_str();
            let command = reg.get(2).unwrap().as_str();
            let pos = reg.get(3);
            let fas = reg.get(4);
            let dim = reg.get(6);

            let mut carpet_cmd = format!("/player {name} {command}");

            if let Some(pos) = pos {
                carpet_cmd.push_str(pos.as_str());
            }

            if command == "spawn" {
                if let Some(fas) = fas {
                    let fas = fas.as_str();

                    if !(fas.starts_with(' ') || carpet_cmd.ends_with(' ')) {
                        carpet_cmd.push(' ');
                    }

                    carpet_cmd.push_str(fas);
                } else {
                    carpet_cmd.push_str("facing 0 0");
                }

                if let Some(dim) = dim {
                    let dim = dim.as_str();

                    if !(dim.starts_with(' ') || carpet_cmd.ends_with(' ')) {
                        carpet_cmd.push(' ');
                    }

                    carpet_cmd.push_str(dim);
                } else {
                    carpet_cmd.push_str("in minecraft:overworld");
                }
            }

            carpet_cmd.push('\n');

            msg.react(ctx, ReactionType::Unicode("✅".to_owned()))
                .await
                .unwrap();

            return Response::new().server_command(carpet_cmd);
        }

        error(
            msg.channel_id,
            ctx,
            "Invalid command",
            format!(
                "This command used the following arg structure: `{}`",
                self.help()
            ),
        )
        .await;

        Response::new()
    }
}

async fn error(channel: ChannelId, ctx: Context, title: &str, des: String) {
    channel
        .send_message(ctx, |x| {
            x.embed(|e| {
                e.title(format!("Error: {}", title))
                    .description(des)
                    .color(colors::RED)
            })
        })
        .await
        .unwrap();
}
