use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

use super::super::colors;
use crate::{Command, Config, Response, VERSION};

const SIGMA_LOGO: &str =
    "https://raw.githubusercontent.com/Basicprogrammer10/NoseBot/master/src/assets/Sigma.png";

pub struct About;

#[async_trait]
impl Command for About {
    fn name(&self) -> &'static str {
        "about"
    }

    fn help(&self) -> &'static str {
        "about"
    }

    fn description(&self) -> &'static str {
        "Gives infomation about this *amazing* bot"
    }

    async fn execute(&self, _cfg: &Config, ctx: Context, msg: Message) -> Response {
        msg.channel_id
            .send_message(ctx, |x| {
                x.embed(|e| {
                    e.title("About")
                        .color(colors::GREEN)
                        .thumbnail(SIGMA_LOGO)
                        .description(format!("Version: {}\nMade by: Sigma#8214", VERSION))
                })
            })
            .await
            .unwrap();

        Response::new()
    }
}
