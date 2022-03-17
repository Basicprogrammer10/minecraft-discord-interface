use std::env;
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio};
use std::sync::Mutex;

use crossbeam_channel::{unbounded, Receiver};
use lazy_static::lazy_static;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId, prelude::*},
    prelude::*,
};
use simple_config_parser::Config;
use tokio;

mod events;
use events::InternalEvent;

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
}

pub enum DiscordEvents {
    Text(String),
    ExitText(String),
}

struct Handler {
    rx: Receiver<DiscordEvent>,

    data_channel: ChannelId,
    event_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, _ctx: Context, _msg: Message) {}

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // tokio::spawn(async move { for e in self.rx.iter() {} });
        for e in self.rx.iter() {
            for f in e.events {
                match f {
                    DiscordEvents::Text(i) => self.event_channel.say(&ctx, i).await.unwrap(),
                    DiscordEvents::ExitText(i) => {
                        self.event_channel.say(&ctx, i).await.unwrap();
                        process::exit(0);
                    }
                };
            }
        }
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

    env::set_current_dir(start_dir).expect("Error moving to dir");
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
