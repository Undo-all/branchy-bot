use std::process::Command;
use serenity::command;
use std::str;

const FORTUNE: &'static str = "fortune";

command!(fortune(ctx, msg, args) {
    let output = Command::new(FORTUNE).output().unwrap();
    let response = str::from_utf8(&output.stdout).unwrap();
    msg.channel_id.say(response)?;
});
