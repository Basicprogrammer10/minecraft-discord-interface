use crate::{Command, Response};
use async_trait::async_trait;
use serenity::{
    client::Context,
    model::channel::{Message, ReactionType},
};

pub struct Refresh;

#[async_trait]
impl Command for Refresh {
    fn name(&self) -> &'static str {
        "refresh"
    }

    fn help(&self) -> &'static str {
        "refresh"
    }

    fn description(&self) -> &'static str {
        "Refreshes data embed"
    }

    async fn execute(&self, _cmd: Vec<&str>, ctx: Context, msg: Message) -> Response {
        msg.react(ctx, ReactionType::try_from(":white_check_mark:").unwrap())
            .await
            .unwrap();

        Response::new().discord_refresh_data()
    }
}
