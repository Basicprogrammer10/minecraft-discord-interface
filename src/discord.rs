use std::fs;
use std::process;
use std::sync::Once;

use crossbeam_channel::Receiver;
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

use crate::PLAYERS;

static INIT_DISCORD: Once = Once::new();

#[derive(Debug, Clone)]
pub struct Handler {
    pub rx: Receiver<DiscordEvent>,

    pub msg_id_file: String,
    pub data_message: Option<MessageId>,
    pub data_channel: ChannelId,
    pub event_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        INIT_DISCORD.call_once(|| {
            // Init Discord thing
            let this = self.clone();
            tokio::spawn(async move {
                // Get / Create the message to modify
                let data_message =
                    get_data_message(&ctx, this.msg_id_file, this.data_message, this.data_channel)
                        .await;

                // Wait for incomming events
                for e in this.rx.iter() {
                    for f in e.events {
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
                }
            });
        });
    }

    async fn ready(&self, _ctx: Context, ready: Ready) {
        println!(
            "[*] Bot `{}:{}` is ready!\n",
            ready.user.name, ready.user.discriminator
        );
    }
}

pub enum DiscordEvents {
    TextEvent(String),
    RefreshData,
    StopData,
    Exit,
}

/// A series of events to execute in the discord runtime
pub struct DiscordEvent {
    pub events: Vec<DiscordEvents>,
}

impl DiscordEvent {
    pub fn new() -> Self {
        DiscordEvent { events: Vec::new() }
    }

    pub fn text<T>(self, text: T) -> Self
    where
        T: AsRef<str>,
    {
        let mut events = self.events;
        events.push(DiscordEvents::TextEvent(text.as_ref().to_owned()));

        DiscordEvent { events }
    }

    pub fn exit(self) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::Exit);

        DiscordEvent { events }
    }

    pub fn refresh_data(self) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::RefreshData);

        DiscordEvent { events }
    }

    pub fn stop_data(self) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::StopData);

        DiscordEvent { events }
    }
}

/// Refresh data message
fn data_refresh(m: &mut EditMessage) -> &mut EditMessage {
    let now = chrono::Utc::now();
    let mut players = String::new();

    for i in PLAYERS.lock().iter().filter(|x| x.online) {
        players.push_str(i.to_string().as_str());
        players.push('\n');
    }

    m.content("").embed(|e| {
        e.color(0x09BC8A)
            .timestamp(now)
            .title("Players Online")
            .description(players)
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
