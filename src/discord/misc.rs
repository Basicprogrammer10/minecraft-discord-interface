use std::fs;

use serenity::{
    builder::EditMessage,
    model::id::{ChannelId, MessageId},
    prelude::*,
};

use super::{colors, commands};
use crate::{common, PLAYERS};

/// Refresh data message
pub fn data_refresh(m: &mut EditMessage) -> &mut EditMessage {
    let now = chrono::Utc::now();
    let mut players = String::from("\u{200b}");

    for i in PLAYERS.lock().iter().filter(|x| x.online) {
        players.push_str(i.to_string().as_str());
        players.push('\n');
    }

    m.content("").embed(|e| {
        e.color(colors::GREEN)
            .timestamp(now)
            .title("Server Online")
            .field("Players", players, false)
    });

    m
}

/// Get / Create data message_id
pub async fn get_data_message(
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
                    x.content("").add_embed(|e| {
                        e.color(colors::YELLOW)
                            .timestamp(now)
                            .title("Server Starting...")
                    })
                })
                .await
                .unwrap();
            i
        }
        None => {
            let i = data_channel
                .send_message(&ctx, |x| {
                    x.content("").add_embed(|e| {
                        e.color(colors::YELLOW)
                            .timestamp(now)
                            .title("Server Starting...")
                    })
                })
                .await
                .unwrap()
                .id;
            fs::write(msg_id_file, i.as_u64().to_string()).unwrap();
            i
        }
    }
}

pub fn command_parts(cmd: &str, prefix: &str) -> Vec<String> {
    cmd.strip_prefix(prefix)
        .unwrap()
        .split(' ')
        .map(|x| x.to_owned())
        .collect()
}

pub fn best_command(command: &str) -> (usize, f64) {
    let mut best = (0usize, 0f64);
    for (i, e) in commands::COMMANDS.iter().enumerate() {
        let sim = common::similarity(command, e.name());

        if sim > best.1 {
            best.1 = sim;
            best.0 = i;
        }
    }

    best
}
