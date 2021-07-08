use clap::{AppSettings, Clap};
use anyhow::{Context, Result };

mod samson;
mod commands;
mod errors;
mod configuration;

use commands::*;
use errors::SamsonrError;

/// This doc string acts as a help message when the user runs '--help'
/// as do all doc strings on fields
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Hans Schnedlitz <hans.schnedlitz@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long)]
    token: Option<String>,
    /// Some input. Because this isn't an Option<T> it's required to be used
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: SubCommand,
}


#[derive(Clap, Debug)]
enum SubCommand {
    Projects(ProjectsCommand),
    Stages(StagesCommand),
    Deploy(DeployCommand),
}

fn main() ->  Result<(), SamsonrError> {
    let opts: Opts = Opts::parse();

    let token = load_token()?;
    // println!("Token {:?}", token);

    let samson_client = samson::Client::new(&token)?;  

    match opts.subcmd {
        SubCommand::Projects(command) => { command.run(&samson_client) },
        SubCommand::Stages(command) => { command.run(&samson_client)? },
        SubCommand::Deploy(command) => { command.run(&samson_client)? },
    }
    Ok(())
}

fn load_token() -> Result<String, SamsonrError> {
    let configuration = configuration::Configuration::new();
    if let Ok(config) = configuration {
        if let Some(token) = config.token {
            return Ok(token.clone());
        }
    }
    
    let opts: Opts = Opts::parse();
    if let Some(token) = opts.token {
        return Ok(token.clone())
    }

    Err(SamsonrError { message: format!("Missing authorization token") })
}
