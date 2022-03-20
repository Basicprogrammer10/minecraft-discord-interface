use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam_channel::{Receiver, Sender};
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId, MessageId},
    },
    prelude::*,
};

use crate::{Command, Config, DiscordEvents, SERVER_ON};

mod colors;
pub mod commands;
mod misc;

#[derive(Clone)]
pub struct Handler {
    pub loop_init: Arc<AtomicBool>,
    pub config: Config,
    pub commands: Arc<Vec<Box<dyn Command + Send + Sync>>>,

    pub discord_rx: Receiver<Vec<DiscordEvents>>,
    pub discord_tx: Sender<Vec<DiscordEvents>>,
    pub server_tx: Sender<Vec<String>>,

    pub data_message: Option<MessageId>,
    pub data_channel: ChannelId,
    pub event_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, ctx: Context, msg: Message) {
        let prefix = &self.config.bot.command_prefix;
        if !msg.content.starts_with(prefix) {
            return;
        }

        // Split the command into parts by space
        let parts = misc::command_parts(&msg.content, prefix);

        // If command is installed and emabled, run it
        if let Some(i) = self.commands.iter().find(|x| x.name() == parts[0]) {
            if i.needs_server() && !*SERVER_ON.read() {
                misc::error(
                    msg.channel_id,
                    ctx,
                    "Server not ready",
                    "Please wait for the server to start before using this command",
                )
                .await;
                return;
            }

            let exe = i.execute(&self.config, ctx.clone(), msg.clone()).await;

            self.server_tx
                .send(exe.server)
                .expect("Error sending event to server");
            self.discord_tx
                .send(exe.discord)
                .expect("Error sending event to discord thread");
            return;
        }

        // If command is not found
        // Try to find one by a simaler name
        let best = misc::best_command(&*self.commands, &parts[0]);

        // Create a discription depending on the max similarity
        let disc = if best.1 > 0.0 {
            format!(
                "Did you mean `{pre}{}`? ({}%)\n",
                self.commands[best.0].name(),
                (best.1 * 100.0).round(),
                pre = prefix
            )
        } else {
            "".to_owned()
        };

        // Send message
        msg.channel_id
            .send_message(ctx, |x| {
                x.embed(|e| {
                    e.title("Error: Unknown Command")
                        .description(format!("{}Use `{}help` for all commands.", disc, prefix,))
                        .color(colors::RED)
                })
            })
            .await
            .unwrap();
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        if self.loop_init.load(Ordering::Relaxed) {
            return;
        }

        self.loop_init.store(true, Ordering::Relaxed);

        // Init Discord thing
        let this = self.clone();

        // Get / Create the message to modify
        let data_message = misc::get_data_message(
            &ctx,
            this.config.bot.message_id_path,
            this.data_message,
            this.data_channel,
        )
        .await;

        tokio::spawn(async move {
            // Wait for incomming events
            for e in this.discord_rx.iter() {
                // For each event in the event array
                for f in e {
                    match f {
                        DiscordEvents::TextEvent(i) => {
                            this.event_channel.say(&ctx, i).await.unwrap();
                        }
                        DiscordEvents::RefreshData => {
                            this.data_channel
                                .edit_message(&ctx, data_message, misc::data_refresh)
                                .await
                                .unwrap();
                        }
                        DiscordEvents::StopData => {
                            let now = chrono::Utc::now();

                            this.data_channel
                                .edit_message(&ctx, data_message, |x| {
                                    x.content("").embed(|e| {
                                        e.color(colors::RED).timestamp(now).title("Server Stoped")
                                    })
                                })
                                .await
                                .unwrap();
                        }
                        DiscordEvents::Exit => {
                            process::exit(0);
                        }
                    };
                }

                // Go poll other tasks
                tokio::task::yield_now().await;
            }
        });
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!(
            "[*] Bot `{}:{}` is ready!\n",
            ready.user.name, ready.user.discriminator
        );
    }
}
