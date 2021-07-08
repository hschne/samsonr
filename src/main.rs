use clap::{AppSettings, Clap};
extern crate simplelog;
use simplelog::*;
use log::*;

#[macro_use] extern crate prettytable;

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
    setup_logger(&opts);
    let token = load_token()?;
    // println!("Token {:?}", token);

    let samson_client = samson::Client::new(&token)?;  

    match opts.subcmd {
        SubCommand::Projects(command) => { command.run(&samson_client)? },
        SubCommand::Stages(command) => { command.run(&samson_client)? },
        SubCommand::Deploy(command) => { command.run(&samson_client)? },
    }
    Ok(())
}

fn setup_logger(opts: &Opts) {
    let log_level = match opts.verbose {
        1 => Some(LevelFilter::Error),
        2 => Some(LevelFilter::Info),
        3 => Some(LevelFilter::Debug),
        4 => Some(LevelFilter::Trace),
        _ => None,
    };

    if let Some(log_level) = log_level {
        TermLogger::init(
            log_level,
            Config::default(),
            TerminalMode::Stdout,
            ColorChoice::Auto,
        ).unwrap();
    }

}

fn load_token() -> Result<String, SamsonrError> {
    debug!("Loading token...");
    let configuration = configuration::Configuration::new();
    if let Ok(config) = configuration {
        debug!("Loading token from config");
        if let Some(token) = config.token {
            debug!("Return token from config, token={}", token);
            return Ok(token.clone());
        }
    }

    let opts: Opts = Opts::parse();
    debug!("Loading token from argument");
    if let Some(token) = opts.token {
        debug!("Return token from argument, token={}", token);
        return Ok(token.clone())
    }

    error!("No token found");
    Err(SamsonrError { message: format!("Missing authorization token") })
}
