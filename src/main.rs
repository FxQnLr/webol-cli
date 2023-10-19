use clap::{Parser, Subcommand};
use config::SETTINGS;
use error::CliError;
use requests::{start::start, get::get};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

mod config;
mod error;
mod requests;

/// webol http client
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    commands: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Start {
        /// id of the device
        id: String
    },
    Get {
        id: String
    }
}

fn main() -> Result<(), CliError> {
    let cli = Args::parse();

    match cli.commands {
        Commands::Start { id } => {
            start(id)?;
        },
        Commands::Get { id } => {
            get(id)?;
        }
    }

    Ok(())
}

fn default_headers() -> Result<HeaderMap, CliError> {
    let mut map = HeaderMap::new();
    map.append("Accept-Content", HeaderValue::from_str("application/json").unwrap());
    map.append("Content-Type", HeaderValue::from_str("application/json").unwrap());
    map.append(
        "Authorization",
        HeaderValue::from_str(
            SETTINGS.get_string("key")
            .map_err(CliError::Config)?
            .as_str()
        ).unwrap()
    );

    Ok(map)

}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String
}
