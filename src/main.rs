use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio};
use std::sync::Once;

use crossbeam_channel::{unbounded, Receiver};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        id::{ChannelId, MessageId},
        prelude::*,
    },
    prelude::*,
};
use simple_config_parser::Config;
use tokio;

mod events;
use events::InternalEvent;

const MESSAGE_ID_PATH: &str = ".message_id.txt";
static INIT_DISCORD: Once = Once::new();

lazy_static! {
    pub static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
}

pub struct DiscordEvent {
    pub events: Vec<DiscordEvents>,
}

impl DiscordEvent {
    fn new() -> Self {
        DiscordEvent { events: Vec::new() }
    }

    fn text(self, text: String) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::Text(text));

        DiscordEvent { events, ..self }
    }

    fn exit_text(self, text: String) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::ExitText(text));

        DiscordEvent { events, ..self }
    }

    fn refresh_data(self) -> Self {
        let mut events = self.events;
        events.push(DiscordEvents::RefreshData);

        DiscordEvent { events, ..self }
    }
}

pub enum DiscordEvents {
    Text(String),
    ExitText(String),
    RefreshData,
}

#[derive(Debug, Clone)]
struct Handler {
    rx: Receiver<DiscordEvent>,

    data_message: Option<MessageId>,
    data_channel: ChannelId,
    event_channel: ChannelId,
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
                    get_data_message(&ctx, this.data_message, this.data_channel).await;

                // Wait for incomming events
                for e in this.rx.iter() {
                    for f in e.events {
                        match f {
                            DiscordEvents::Text(i) => {
                                this.event_channel.say(&ctx, i).await.unwrap();
                            }
                            DiscordEvents::RefreshData => {
                                this.data_channel
                                    .edit_message(&ctx, data_message, |x| x.content(data_refresh()))
                                    .await
                                    .unwrap();
                            }
                            DiscordEvents::ExitText(i) => {
                                this.event_channel.say(&ctx, i).await.unwrap();
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
            "{}:{} is ready!\n",
            ready.user.name, ready.user.discriminator
        );
    }
}

macro_rules! cfg_get {
    ($cfg:expr, $name:expr) => {
        $cfg.get_str($name)
            .expect(concat!("Error getting `", $name, "` from Config"))
    };

    ($cfg:expr, $name:expr, $parse_type:ty) => {
        $cfg.get::<$parse_type>($name)
            .expect(concat!("Error getting `", $name, "` from Config"))
    };
}

fn main() {
    let cfg = Config::new().file("config.cfg").unwrap();

    let bot_token = cfg_get!(cfg, "bot_token");
    let bot_data_channel = cfg_get!(cfg, "bot_data_channel", u64);
    let bot_event_channel = cfg_get!(cfg, "bot_event_channel", u64);

    let start_dir = cfg_get!(cfg, "mc_dir");
    let mc_start_cmd = cfg_get!(cfg, "mc_start_cmd");
    let mc_java_path = cfg_get!(cfg, "mc_java_path");

    // Move into the server dir
    env::set_current_dir(start_dir).expect("Error moving to dir");

    // Try to get the data message id
    let data_message_id = fs::read_to_string(MESSAGE_ID_PATH)
        .ok()
        .map_or(None, |x| Some(MessageId::from(x.parse::<u64>().unwrap())));

    // Load internal events
    let events = events::base_events();
    let events = events::mass_init_regex(events);

    // Create mpsc channel
    let (tx, rx) = unbounded();

    // Start async runtime and discord bot in another thread
    std::thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut client = Client::builder(bot_token)
                    .event_handler(Handler {
                        rx,
                        data_message: data_message_id,
                        data_channel: ChannelId::from(bot_data_channel),
                        event_channel: ChannelId::from(bot_event_channel),
                    })
                    .await
                    .expect("Error creating discord client");

                if let Err(e) = client.start().await {
                    println!("Discord client error: {:?}", e);
                }
            });
    });

    let mut server = process::Command::new(mc_java_path)
        .args(mc_start_cmd.split(' ').collect::<Vec<&str>>())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error starting process");

    let raw_stdout = server
        .stdout
        .as_mut()
        .expect("Error getting process stdout");
    let stdout = BufReader::new(raw_stdout).lines();

    for i in stdout.map(|x| x.unwrap()) {
        // Pass through stdout
        println!("[$] {}", i);

        // Trigger Events if regex matches
        events.iter().for_each(|e| {
            if let Some(j) = e.0.captures(&i) {
                tx.send(e.1.execute(&i, j))
                    .expect("Error sending event to discord thread");
            }
        })
    }

    let status = server.wait().unwrap().code().unwrap();
    tx.send(match status {
        0 => events::server_stop::ServerStop.execute(),
        _ => events::server_crash::ServerCrash(status).execute(),
    })
    .expect("Error sending event to discord thread");
    loop {}
}

fn data_refresh() -> String {
    let mut base = String::from("Players Online:\n");

    {
        for i in PLAYERS.lock().iter() {
            base.push_str(&format!("  - {}\n", i));
        }
    }

    base
}

async fn get_data_message(
    ctx: &Context,
    data_message: Option<MessageId>,
    data_channel: ChannelId,
) -> MessageId {
    match data_message {
        Some(i) => {
            data_channel
                .edit_message(&ctx, i, |x| x.content("Server Starting"))
                .await
                .unwrap();
            i
        }
        None => {
            let i = data_channel.say(&ctx, "Server Starting").await.unwrap().id;
            fs::write(MESSAGE_ID_PATH, i.as_u64().to_string()).unwrap();
            i
        }
    }
}
