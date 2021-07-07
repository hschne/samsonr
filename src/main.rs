use clap::{AppSettings, Clap};

mod samson;
mod commands;
mod configuration;

use commands::*;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap)]
#[clap(version = "1.0", author = "Hans Schnedlitz <hans.schnedlitz@gmail.com>")]
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
    #[clap(subcommand)]
    subcmd: SubCommand,
}


#[derive(Clap)]
enum SubCommand {
    Projects(ProjectsCommand),
    Stages(StagesCommand),
    Deploy(DeployCommand),
}

#[derive(Clap)]
struct Test {
    /// Print debug info
    #[clap(short)]
    debug: bool
}


fn main() ->  Result<(), CommandError> {
    let opts: Opts = Opts::parse();

    let token = opts.token.unwrap();
    println!("Token {}", token);

    let samson_client = samson::Client::new(&token).expect("Failed to create client");

    match opts.subcmd {
        SubCommand::Projects(command) => { command.run(&samson_client) },
        SubCommand::Stages(command) => { command.run(&samson_client) },
        SubCommand::Deploy(command) => { command.run(&samson_client)? },
    }
    Ok(())
}
