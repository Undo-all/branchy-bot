use log::info;
use std::io;
use std::process::Command;
use std::str;

use serenity::command;

const WOLFRAMSCRIPT_PATH: &'static str = "wolframscript.exe";

fn execute(code: String) -> io::Result<String> {
    let output = Command::new(WOLFRAMSCRIPT_PATH)
        .arg("-code")
        .arg(code)
        .output()?;
    //let output = cmd.output()?;
    let stdout = str::from_utf8(&output.stdout).expect("Invalid UTF8 in Mathematica output!");
    let result = stdout.lines().last().unwrap_or("");
    Ok(result.to_string())
}

fn classify(classifier: &str, what: &str) -> io::Result<String> {
    execute(format!("Classify [\"{}\", {:?}]", classifier, what))
}

command!(sentiment(ctx, msg, args) {
    let what = args.full_quoted();

    let class = classify("Sentiment", what).unwrap();
    msg.channel_id.say(class)?;
});

command!(profane(ctx, msg, args) {
    let what = args.full_quoted();
    let class = classify("Profanity", what).unwrap();
    msg.channel_id.say(class)?;
});

command!(language(ctx, msg, args) {
    let what = args.full_quoted();
    let class = classify("Language", what).unwrap();
    msg.channel_id.say(class)?;
});

command!(topic(ctx, msg, args) {
    let what = args.full_quoted();
    let class = classify("FacebookTopic", what).unwrap();
    msg.channel_id.say(class)?;
});

command!(spam(ctx, msg, args) {
    let what = args.full_quoted();
    let class = classify("Spam", what).unwrap();
    msg.channel_id.say(class)?;
});

/*
pub enum Sentiment {
    Positive,
    Neutral,
    Negative,
}

impl Sentiment {
    fn from_string(string: String) {
        match string {
            "Positive" => Sentiment::Positive,
            "Neutral" => Sentiment::Neutral,
            "Negative" => Sentiment::Negative,
        }
    }
}*/
