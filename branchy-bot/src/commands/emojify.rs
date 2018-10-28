use std::io::Read;
use std::io::Write;
use std::process;
use std::sync::{Arc, Mutex};

use serenity::framework::standard::Args;
use serenity::framework::standard::Command;
use serenity::framework::standard::CommandError;
use serenity::model::channel::Message;
use serenity::prelude::*;

const MAPPING: [char; 64] = [
    '\u{1f612}',
    '\u{1f612}',
    '\u{1f629}',
    '\u{1f62d}',
    '\u{1f60d}',
    '\u{1f614}',
    '\u{1f44c}',
    '\u{1f60a}',
    '\u{2764}',
    '\u{1f60f}',
    '\u{1f601}',
    '\u{1f3b6}',
    '\u{1f633}',
    '\u{1f4af}',
    '\u{1f634}',
    '\u{1f60c}',
    '\u{263a}',
    '\u{1f64c}',
    '\u{1f495}',
    '\u{1f611}',
    '\u{1f605}',
    '\u{1f64f}',
    '\u{1f615}',
    '\u{1f618}',
    '\u{2665}',
    '\u{1f610}',
    '\u{1f481}',
    '\u{1f61e}',
    '\u{1f648}',
    '\u{1f62b}',
    '\u{270c}',
    '\u{1f60e}',
    '\u{1f621}',
    '\u{1f44d}',
    '\u{1f622}',
    '\u{1f62a}',
    '\u{1f60b}',
    '\u{1f624}',
    '\u{270b}',
    '\u{1f637}',
    '\u{1f44f}',
    '\u{1f440}',
    '\u{1f52b}',
    '\u{1f623}',
    '\u{1f608}',
    '\u{1f613}',
    '\u{1f494}',
    '\u{2661}',
    '\u{1f3a7}',
    '\u{1f64a}',
    '\u{1f609}',
    '\u{1f480}',
    '\u{1f616}',
    '\u{1f604}',
    '\u{1f61c}',
    '\u{1f620}',
    '\u{1f645}',
    '\u{1f4aa}',
    '\u{1f44a}',
    '\u{1f49c}',
    '\u{1f496}',
    '\u{1f499}',
    '\u{1f62c}',
    '\u{2728}',
];

pub struct Emojify {
    deepmoji: Mutex<process::Child>,
}

impl Emojify {
    pub fn new() -> Emojify {
        let child = process::Command::new("python3")
            .arg("servers/deepmoji/server.py")
            .stdin(process::Stdio::piped())
            .stdout(process::Stdio::piped())
            .spawn()
            .unwrap();
        Emojify {
            deepmoji: Mutex::new(child),
        }
    }
}

impl Command for Emojify {
    fn execute(&self, ctx: &mut Context, msg: &Message, args: Args) -> Result<(), CommandError> {
        let arg = args.full_quoted();
        let mut server = self.deepmoji.lock().unwrap();
        let mut stdin = server.stdin.as_mut().unwrap();
        writeln!(stdin, "{}", arg);
        let mut stdout = server.stdout.as_mut().unwrap();
        let mut out = [0, 0];
        stdout.read(&mut out);
        let index = std::str::from_utf8(&out).unwrap().parse::<usize>().unwrap();

        msg.channel_id.say(MAPPING[index]);

        Ok(())
    }
}
