use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::process::{self, Stdio};
use std::sync::{atomic::AtomicBool, Arc};
use std::thread;

use crossbeam_channel::unbounded;
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use serenity::{
    model::id::{ChannelId, MessageId},
    prelude::*,
};
use simple_config_parser::Config as Cfg;

mod common;
mod discord;
mod events;
mod types;
use events::InternalEvent;
use types::{
    command::Command,
    config::Config,
    player::Player,
    response::{DiscordEvents, Response},
};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    // Online Players
    pub static ref PLAYERS: Mutex<Vec<Player>> = Mutex::new(Vec::new());

    // If mc server is on
    pub static ref SERVER_ON: RwLock<bool> = RwLock::new(false);
}

fn main() {
    // Load config values
    let cfg = Cfg::new().file("config.cfg").unwrap();
    let config = Config::new(cfg);

    // Move into the server dir
    env::set_current_dir(&config.minecraft.dir).expect("Error moving to dir");

    // Try to get the data message id
    let data_message_id = fs::read_to_string(&config.bot.message_id_path)
        .ok()
        .map(|x| MessageId::from(x.parse::<u64>().unwrap()));

    // Load internal events
    let events = events::base_events();
    let events = events::mass_init_regex(events);
    let events = events
        .into_iter()
        .filter(|x| {
            !config
                .minecraft
                .disabled_events
                .contains(&(*x.1.name()).to_owned())
        })
        .collect::<Vec<_>>();

    // Load discord commands
    let discord_commands = discord::commands::base_commands();
    let discord_commands = discord_commands
        .into_iter()
        .filter(|x| {
            !config
                .bot
                .disabled_commands
                .contains(&(*x.name()).to_owned())
        })
        .collect::<Vec<_>>();

    println!(
        "Minecraft Events: {}",
        events
            .iter()
            .map(|x| x.1.name())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    println!(
        "Discord Commands: {}",
        discord_commands
            .iter()
            .map(|x| x.name())
            .collect::<Vec<&str>>()
            .join(", ")
    );

    // Create mpsc channels
    let (discord_tx, discord_rx) = unbounded();
    let (server_tx, server_rx) = unbounded();

    // Start async runtime and discord bot in another thread
    let server_tx_1 = server_tx.clone();
    let discord_tx_1 = discord_tx.clone();
    let config_1 = config.clone();
    thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let mut client = Client::builder(config.bot.token)
                    .event_handler(discord::Handler {
                        loop_init: Arc::new(AtomicBool::new(false)),
                        config: config_1,
                        commands: Arc::new(discord_commands),
                        discord_rx,
                        discord_tx: discord_tx_1,
                        server_tx: server_tx_1,
                        data_message: data_message_id,
                        data_channel: ChannelId::from(config.bot.data_channel),
                        event_channel: ChannelId::from(config.bot.event_channel),
                    })
                    .await
                    .expect("Error creating discord client");

                if let Err(e) = client.start().await {
                    println!("Discord client error: {:?}", e);
                }
            });
    });

    // Start server
    let mut server = process::Command::new(config.minecraft.java_path)
        .args(config.minecraft.start_cmd.split(' ').collect::<Vec<&str>>())
        .stdout(Stdio::piped())
        .stdin(Stdio::piped())
        .spawn()
        .expect("Error starting process");

    // Get std(in / out)
    let raw_stdout = server
        .stdout
        .as_mut()
        .expect("Error getting process stdout");
    let stdout = BufReader::new(raw_stdout).lines();
    let mut stdin = server.stdin.take();

    // Spawn a new thread to interact with server stdin
    thread::spawn(move || {
        let stdin = stdin.as_mut().expect("Error getting process stdin");

        // Wait for incomming server events
        for i in server_rx.iter() {
            // Execute commands in the event
            for j in i {
                stdin
                    .write_all(j.as_bytes())
                    .expect("Error writing to stdout");
                stdin.flush().unwrap();
            }
        }
    });

    // Loop though stdout stream
    for i in stdout.map(|x| x.unwrap()) {
        // Pass through stdout
        println!("[$] {}", i);

        // Trigger Events if regex matches
        events.iter().for_each(|e| {
            if let Some(j) = e.0.captures(&i) {
                // Run code for event
                let exe = e.1.execute(&i, j);

                // Send server response
                server_tx
                    .send(exe.server)
                    .expect("Error sending event to server");

                // Send discord respons
                discord_tx
                    .send(exe.discord)
                    .expect("Error sending event to discord thread");
            }
        })
    }

    // Send server stop / crash event
    let status = server.wait().unwrap().code().unwrap();
    discord_tx
        .send(
            match status {
                0 => events::server_stop::ServerStop.execute(),
                _ => events::server_crash::ServerCrash(status).execute(),
            }
            .discord,
        )
        .expect("Error sending event to discord thread");

    // Block thread untill final discord message sends
    thread::park();
}
