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

/// SamsonR is a simple command line client for Samson
#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Hans Schnedlitz <hans.schnedlitz@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Your authorization token
    #[clap(short, long)]
    token: Option<String>,
    /// Verbosity, repeat this to increate the log level
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    /// Subcommands to interact with the server
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

    let url = load_base_url()?;
    let samson_client = samson::Client::new(&token, &url)?;  

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

fn load_base_url() -> Result<String, SamsonrError> {
    debug!("Loading base url...");
    let configuration = configuration::Configuration::new();
    if let Ok(config) = configuration {
        debug!("Return url from config, url={}", config.url);
        return Ok(config.url);
    }

    error!("No url found");
    Err(SamsonrError { message: format!("Missing authorization token") })
}
