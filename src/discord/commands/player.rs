use async_trait::async_trait;
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
    client::Context,
    model::channel::{Message, ReactionType},
};

use super::super::misc;
use crate::{Command, Config, Response};

lazy_static! {
    // if only i was good a regex then i would be a regexpert lol
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

    fn needs_server(&self) -> bool {
        true
    }

    async fn execute(&self, cfg: &Config, ctx: Context, msg: Message) -> Response {
        let cmd = misc::command_parts(&msg.content, &cfg.bot.command_prefix);
        let cmd_str = cmd
            .iter()
            .skip(1)
            .map(|x| x.to_owned())
            .collect::<Vec<String>>()
            .join(" ");
        let cmd_str = rem_dub_space(cmd_str);

        if let Some(reg) = PLAYER_REGEX.captures(&cmd_str) {
            let name = reg.get(1).unwrap().as_str();
            let command = reg.get(2).unwrap().as_str();
            let pos = reg.get(3);
            let fas = reg.get(4);
            let dim = reg.get(6);

            let mut carpet_cmd = format!("/player {name} {command}");

            if command == "spawn" {
                carpet_cmd.push(' ');
                if let Some(pos) = pos {
                    carpet_cmd.push_str(pos.as_str());
                } else {
                    carpet_cmd.push_str("at ~ ~ ~");
                }

                carpet_cmd.push(' ');
                if let Some(fas) = fas {
                    carpet_cmd.push_str(fas.as_str());
                } else {
                    carpet_cmd.push_str("facing 0 0");
                }

                carpet_cmd.push(' ');
                if let Some(dim) = dim {
                    carpet_cmd.push_str(dim.as_str());
                } else {
                    carpet_cmd.push_str("in minecraft:overworld");
                }
            }

            carpet_cmd.push('\n');

            msg.react(ctx, ReactionType::Unicode("✅".to_owned()))
                .await
                .unwrap();

            return Response::new().server_command(rem_dub_space(carpet_cmd));
        }

        misc::error(
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

fn rem_dub_space(inp: String) -> String {
    let inp = inp.chars().collect::<Vec<char>>();

    inp.iter()
        .enumerate()
        .filter(|(i, x)| !(**x == ' ' && inp[i.saturating_sub(1)] == ' '))
        .map(|x| x.1)
        .collect::<String>()
}
