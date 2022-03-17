use std::env;
use std::fs;
use std::io::{BufRead, BufReader};
use std::process::{self, Stdio};
use std::thread;

use crossbeam_channel::unbounded;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serenity::{
    model::id::{ChannelId, MessageId},
    prelude::*,
};
use simple_config_parser::Config;

mod discord;
mod events;
use discord::DiscordEvent;
use events::InternalEvent;

lazy_static! {
    // Online Players
    pub static ref PLAYERS: Mutex<Vec<String>> = Mutex::new(Vec::new());
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
    // Load config values
    let cfg = Config::new().file("config.cfg").unwrap();

    let bot_token = cfg_get!(cfg, "bot_token");
    let bot_data_channel = cfg_get!(cfg, "bot_data_channel", u64);
    let bot_event_channel = cfg_get!(cfg, "bot_event_channel", u64);
    let data_message_id_file = cfg_get!(cfg, "data_message_id_file");

    let start_dir = cfg_get!(cfg, "mc_dir");
    let mc_start_cmd = cfg_get!(cfg, "mc_start_cmd");
    let mc_java_path = cfg_get!(cfg, "mc_java_path");

    // Move into the server dir
    env::set_current_dir(start_dir).expect("Error moving to dir");

    // Try to get the data message id
    let data_message_id = fs::read_to_string(&data_message_id_file)
        .ok()
        .map(|x| MessageId::from(x.parse::<u64>().unwrap()));

    // Load internal events
    let events = events::base_events();
    let events = events::mass_init_regex(events);

    // Create mpsc channel
    let (tx, rx) = unbounded();

    // Start async runtime and discord bot in another thread
    thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut client = Client::builder(bot_token)
                    .event_handler(discord::Handler {
                        rx,
                        msg_id_file: data_message_id_file,
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

    // Start server
    let mut server = process::Command::new(mc_java_path)
        .args(mc_start_cmd.split(' ').collect::<Vec<&str>>())
        .stdout(Stdio::piped())
        .spawn()
        .expect("Error starting process");

    // Get stdout
    let raw_stdout = server
        .stdout
        .as_mut()
        .expect("Error getting process stdout");
    let stdout = BufReader::new(raw_stdout).lines();

    // Loop though stdout stream
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

    // Send server stop / crash event
    let status = server.wait().unwrap().code().unwrap();
    tx.send(match status {
        0 => events::server_stop::ServerStop.execute(),
        _ => events::server_crash::ServerCrash(status).execute(),
    })
    .expect("Error sending event to discord thread");

    // Block thread untill final discord message sends
    thread::park();
}
