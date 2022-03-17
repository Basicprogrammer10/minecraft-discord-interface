use std::env;
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio};
use std::sync::mpsc;

use crossbeam_channel::{unbounded, Receiver};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready, id::ChannelId, prelude::*},
    prelude::*,
};
use simple_config_parser::Config;
use tokio;

mod events;

enum Event {
    TextEvent(String),
}

struct Handler {
    rx: Receiver<Event>,

    data_channel: ChannelId,
    event_channel: ChannelId,
}

#[async_trait]
impl EventHandler for Handler {
    // On User Send Message
    async fn message(&self, ctx: Context, msg: Message) {}

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        // tokio::spawn(async move { for e in self.rx.iter() {} });
        for e in self.rx.iter() {
            match e {
                Event::TextEvent(i) => self.event_channel.say(&ctx, i).await.unwrap(),
            };
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
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
                if let Some(m) = e.1.execute(&i, j) {
                    tx.send(Event::TextEvent(m));
                }
            }
        })
    }
}

// 274877926400
