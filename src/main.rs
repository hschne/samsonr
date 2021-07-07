use clap::{AppSettings, Clap};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header;

mod configuration;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Kevin K. <kbknapp@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Sets a custom config file. Could have been an Option<T> with no default too
    #[clap(short, long)]
    config: Option<String>,
    #[clap(short, long)]
    token: Option<String>,
    /// Some input. Because this isn't an Option<T> it's required to be used
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
}


#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.3", author = "Someone E. <someone_else@other.com>")]
    Test(Test),
}

#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool
}

#[derive(Debug, Deserialize, Serialize)]
struct Project {
    id: i32,
    name: String,
    last_deployed_at: String,
    last_deployed_by: String,
}

static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let token = opts.token.unwrap();
    println!("Token {}", token);

    let mut headers = header::HeaderMap::new();
    let auth_header = &format!("Bearer {}", token)[..];
    let header_value = header::HeaderValue::from_str(auth_header)?;
    headers.insert(header::AUTHORIZATION, header_value);

    println!("{:#?}", headers);

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .default_headers(headers)
        .build()?;
    let base_url = "https://deploy.meisterlabs.com";
    let projects_url = format!("{}/projects.json", base_url);
    let builds_url = format!("{}/projects/{}/builds.json", base_url, 1);
    let body = client.get(projects_url)
        .send()?
        .text()?;

    println!("{:#?}", body);

    let result: HashMap<String, Vec<Project>> = serde_json::from_str(&body)?;
    println!("{:#?}", result);
    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match opts.verbose {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    // match opts.subcmd {
    //     SubCommand::Test(t) => {
    //         if t.debug {
    //             println!("Printing debug info...");
    //         } else {
    //             println!("Printing normally...");
    //         }
    //     }
    // }
    Ok(())
}
