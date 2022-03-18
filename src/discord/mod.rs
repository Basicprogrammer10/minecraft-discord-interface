use std::fs;
use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam_channel::{Receiver, Sender};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    builder::EditMessage,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, GuildId, MessageId},
    },
    prelude::*,
};

use crate::{Command, Config, DiscordEvents, PLAYERS};

mod about;
mod help;
mod refresh;

lazy_static! {
    pub static ref COMMANDS: Vec<Box<dyn Command + Sync>> = vec![
        Box::new(about::About),
        Box::new(refresh::Refresh),
        Box::new(help::Help)
    ];
}

#[derive(Debug, Clone)]
pub struct Handler {
    pub loop_init: Arc<AtomicBool>,
    pub config: Config,

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

        let parts = command_parts(&msg.content, prefix);

        if let Some(i) = COMMANDS.iter().find(|x| x.name() == parts[0]) {
            let exe = i.execute(&self.config, ctx.clone(), msg.clone()).await;

            self.server_tx
                .send(exe.server)
                .expect("Error sending event to server");
            self.discord_tx
                .send(exe.discord)
                .expect("Error sending event to discord thread");
            return;
        }

        // TODO: Did you mean XX message
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        if self.loop_init.load(Ordering::Relaxed) {
            return;
        }

        self.loop_init.store(true, Ordering::Relaxed);

        // Init Discord thing
        let this = self.clone();

        // Get / Create the message to modify
        let data_message = get_data_message(
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
                                .edit_message(&ctx, data_message, data_refresh)
                                .await
                                .unwrap();
                        }
                        DiscordEvents::StopData => {
                            let now = chrono::Utc::now();

                            this.data_channel
                                .edit_message(&ctx, data_message, |x| {
                                    x.content("").embed(|e| {
                                        e.color(0xFF785A).timestamp(now).title("Server Stoped")
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

/// Refresh data message
fn data_refresh(m: &mut EditMessage) -> &mut EditMessage {
    let now = chrono::Utc::now();
    let mut players = String::from("\u{200b}");

    for i in PLAYERS.lock().iter().filter(|x| x.online) {
        players.push_str(i.to_string().as_str());
        players.push('\n');
    }

    m.content("").embed(|e| {
        e.color(0x09BC8A)
            .timestamp(now)
            .title("Server Online")
            .field("Players", players, false)
    });

    m
}

/// Get / Create data message_id
async fn get_data_message(
    ctx: &Context,
    msg_id_file: String,
    data_message: Option<MessageId>,
    data_channel: ChannelId,
) -> MessageId {
    let now = chrono::Utc::now();

    match data_message {
        Some(i) => {
            data_channel
                .edit_message(&ctx, i, |x| {
                    x.content("")
                        .add_embed(|e| e.color(0xFFF05A).timestamp(now).title("Server Starting..."))
                })
                .await
                .unwrap();
            i
        }
        None => {
            let i = data_channel
                .send_message(&ctx, |x| {
                    x.content("")
                        .add_embed(|e| e.color(0xFFF05A).timestamp(now).title("Server Starting..."))
                })
                .await
                .unwrap()
                .id;
            fs::write(msg_id_file, i.as_u64().to_string()).unwrap();
            i
        }
    }
}

fn command_parts(cmd: &str, prefix: &str) -> Vec<String> {
    cmd.strip_prefix(prefix)
        .unwrap()
        .split(' ')
        .map(|x| x.to_owned())
        .collect()
}
