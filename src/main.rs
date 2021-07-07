use clap::{AppSettings, Clap};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use reqwest::header;

mod samson;
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

fn main() ->  Result<(), Box<dyn std::error::Error>> {
    let opts: Opts = Opts::parse();

    let token = opts.token.unwrap();
    println!("Token {}", token);

    let samson_client = samson::Client::new(&token);
    if let Ok(client) = samson_client {
        if let Ok(projects) = client.projects() {
            println!("{:?}", projects)
        }
    }


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
