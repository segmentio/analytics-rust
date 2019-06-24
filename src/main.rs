use analytics::client::Client;
use analytics::http::HttpClient;
use analytics::message::Message;
use clap::{App, Arg, AppSettings, SubCommand};
use failure::Error;
use std::io;

fn main() -> Result<(), Error> {
    let matches = App::new("Analytics")
        .version("0.1")
        .author("Segment <friends@segment.com>")
        .about("Sends analytics events to Segment")
        .setting(AppSettings::ColoredHelp)
        .arg(
            Arg::with_name("write-key")
                .help("Write key to send message with")
                .takes_value(true)
                .short("w")
                .long("write-key")
                .required(true),
        )
        .arg(
            Arg::with_name("host")
                .help("Scheme and host to send to")
                .default_value("https://api.segment.io")
                .long("host"),
        )
        .subcommand(SubCommand::with_name("identify"))
        .subcommand(SubCommand::with_name("track"))
        .subcommand(SubCommand::with_name("page"))
        .subcommand(SubCommand::with_name("screen"))
        .subcommand(SubCommand::with_name("group"))
        .subcommand(SubCommand::with_name("alias"))
        .get_matches();

    let client = HttpClient::new(
        reqwest::Client::new(),
        matches.value_of("host").unwrap().to_owned(),
    );

    let message = match matches.subcommand_name() {
        Some("identify") => Message::Identify(serde_json::from_reader(io::stdin())?),
        Some("track") => Message::Track(serde_json::from_reader(io::stdin())?),
        Some("page") => Message::Page(serde_json::from_reader(io::stdin())?),
        Some("screen") => Message::Screen(serde_json::from_reader(io::stdin())?),
        Some("group") => Message::Group(serde_json::from_reader(io::stdin())?),
        Some("alias") => Message::Alias(serde_json::from_reader(io::stdin())?),
        Some(_) => panic!("unknown message type"),
        None => panic!("subcommand is required"),
    };

    println!("{:?}", message);

    let result = client.send(
        matches.value_of("write-key").unwrap(),
        &message,
    )?;

    println!("{:?}", result);
    Ok(())
}
