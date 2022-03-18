use std::fs;
use std::process;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use crossbeam_channel::{Receiver, Sender};
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

use crate::{DiscordEvents, PLAYERS};

#[derive(Debug, Clone)]
pub struct Handler {
    pub loop_init: Arc<AtomicBool>,

    pub rx: Receiver<Vec<DiscordEvents>>,
    pub tx: Sender<String>,

    pub msg_id_file: String,
    pub data_message: Option<MessageId>,
    pub data_channel: ChannelId,
    pub event_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "~test" {
            msg.reply(ctx, "Ok").await.unwrap();
            self.tx.send("/stop\n".to_owned()).unwrap();
        }
    }

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        if self.loop_init.load(Ordering::Relaxed) {
            return;
        }

        self.loop_init.store(true, Ordering::Relaxed);

        // Init Discord thing
        let this = self.clone();

        // Get / Create the message to modify
        let data_message =
            get_data_message(&ctx, this.msg_id_file, this.data_message, this.data_channel).await;

        tokio::spawn(async move {
            // Wait for incomming events
            for e in this.rx.iter() {
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
