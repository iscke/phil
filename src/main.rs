
use std::env;
use serenity::{
    model::{channel::Message, gateway::Ready, user::User, id::ChannelId},
    prelude::*,
};

mod dice;
mod blades;
pub mod common;
#[macro_use] extern crate lazy_static;

struct Handler;
impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("Connected as {}", ready.user.name);
    }

    fn message(&self, ctx: Context, msg: Message) {
        if let Some(m) = parse_message(msg.content, msg.author, msg.channel_id) {
            if let Err(e) = msg.channel_id.say(&ctx.http, m) {
                println!("Error sending message: {:?}", e);
            }
        }
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token as envvar DISCORD_TOKEN");
    let mut client = Client::new(&token, Handler)
        .expect("Error creating client.");
    if let Err(e) = client.start() {
        println!("Error connecting: {:?}", e);
    }
}

fn parse_message(content: String, author: User, _: ChannelId) -> Option<String> {
    if author.bot {
        return None;
    }
    if let Some(roll) = blades::try_roll(&content) {
        return Some(roll);
    } else if let Some(roll) = dice::try_roll(&content) {
        return Some(roll);
    }
    None
}
