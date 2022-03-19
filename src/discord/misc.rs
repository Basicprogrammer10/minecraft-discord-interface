use std::cmp::Ordering;
use std::fs;

use serenity::{
    builder::EditMessage,
    model::id::{ChannelId, MessageId},
    prelude::*,
};

use super::{colors, commands};
use crate::{common, Player, PLAYERS};

/// Refresh data message
pub fn data_refresh(m: &mut EditMessage) -> &mut EditMessage {
    let now = chrono::Utc::now();

    // Get all online players as a vector
    let online_players = PLAYERS.lock();
    let mut online_players = online_players
        .iter()
        .filter(|x| x.online)
        .collect::<Vec<&Player>>();

    // Sort the players to we have real players before bots
    // Then sort alphabetacly within players and bots
    online_players.sort_by(|x, y| {
        if x.bot && !y.bot {
            return Ordering::Greater;
        }

        if !x.bot && y.bot {
            return Ordering::Less;
        }

        x.name.to_lowercase().cmp(&y.name.to_lowercase())
    });

    // Start the players string builder
    let mut players = String::from("```\u{200b}");
    for i in online_players {
        players.push_str(i.to_string().as_str());
        players.push('\n');
    }
    players.push_str("```");

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

pub async fn error<T, M>(channel: ChannelId, ctx: Context, title: T, des: M)
where
    T: AsRef<str>,
    M: AsRef<str>,
{
    channel
        .send_message(ctx, |x| {
            x.embed(|e| {
                e.title(format!("Error: {}", title.as_ref()))
                    .description(des.as_ref())
                    .color(colors::RED)
            })
        })
        .await
        .unwrap();
}
