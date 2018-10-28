#![feature(nll)]
#![feature(custom_attribute)]

use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::framework::StandardFramework;
use serenity::command;

use std::env;
use env_logger::*;
use log::{info};

use std::collections::HashMap;

mod commands;

use self::commands::*;
use branchy_attrib::commandify;

struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn main() {
    env_logger::init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token");
    let mut client = Client::new(&token, Handler).expect("Couldn't create client");

    client.with_framework(StandardFramework::new()
        .configure(|c| c.prefix("%%"))
        .command("sentiment", |c| c.cmd(mathematica::sentiment))
        .command("spam", |c| c.cmd(mathematica::spam))
        .command("language", |c| c.cmd(mathematica::language))
        .command("profane", |c| c.cmd(mathematica::profane))
        .command("topic", |c| c.cmd(mathematica::topic))
        .command("fortune", |c| c.cmd(fortune::fortune))
        .command("emojify", |c| c.cmd(emojify::Emojify::new())));

    if let Err(why) = client.start() {
        println!("Aaaah it's broken: {:?}", why);
    }
}
