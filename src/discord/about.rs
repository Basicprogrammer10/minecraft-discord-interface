use crate::{Command, Response};
use async_trait::async_trait;
use serenity::{client::Context, model::channel::Message};

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

    async fn execute(&self, _cmd: Vec<&str>, ctx: Context, msg: Message) -> Response {
        msg.channel_id
            .send_message(ctx, |x| {
                x.embed(|e| e.title("About").description("// TODO: Write About"))
            })
            .await
            .unwrap();

        Response::new()
    }
}
