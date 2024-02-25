use std::{fmt::Display, time::Duration};

use crate::config::Config;
use clap::{Command, CommandFactory, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};
use error::Error;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use requests::{device, start::start};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Response,
};
use serde::Deserialize;

mod config;
mod error;
mod requests;

static OVERVIEW_STYLE: &str = "{spinner:.green} ({elapsed}{wide_msg}";
static OVERVIEW_ERROR: &str = "✗ ({elapsed}) {wide_msg}";
static OVERVIEW_DONE: &str = "✓ ({elapsed}) {wide_msg}";
static DEFAULT_STYLE: &str = "  {spinner:.green} {wide_msg}";
static DONE_STYLE: &str = "  ✓ {wide_msg}";
static ERROR_STYLE: &str = "  ✗ {wide_msg}";
static TICK_SPEED: u64 = 1000 / 16;

/// webol client
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
        id: String,
        #[arg(short, long)]
        ping: Option<bool>,
    },
    Device {
        #[command(subcommand)]
        devicecmd: DeviceCmd,
    },
    CliGen {
        id: Shell,
    },
}

#[derive(Subcommand)]
enum DeviceCmd {
    Add {
        id: String,
        mac: String,
        broadcast_addr: String,
        ip: String,
    },
    Get {
        id: String,
    },
    Edit {
        id: String,
        mac: String,
        broadcast_addr: String,
        ip: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let config = Config::load()?;

    let cli = Args::parse();

    match cli.commands {
        Commands::Start { id, ping } => {
            start(&config, id, ping.unwrap_or(true)).await?;
        }
        Commands::Device { devicecmd } => match devicecmd {
            DeviceCmd::Add {
                id,
                mac,
                broadcast_addr,
                ip,
            } => {
                device::put(&config, id, mac, broadcast_addr, ip).await?;
            }
            DeviceCmd::Get { id } => {
                device::get(&config, id).await?;
            }
            DeviceCmd::Edit {
                id,
                mac,
                broadcast_addr,
                ip,
            } => {
                device::post(&config, id, mac, broadcast_addr, ip).await?;
            }
        },
        Commands::CliGen { id } => {
            eprintln!("Generating completion file for {id:?}...");
            let mut cmd = Args::command();
            print_completions(id, &mut cmd);
        }
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn default_headers(config: &Config) -> Result<HeaderMap, Error> {
    let mut map = HeaderMap::new();
    map.append("Accept-Content", HeaderValue::from_str("application/json")?);
    map.append("Content-Type", HeaderValue::from_str("application/json")?);
    map.append("Authorization", HeaderValue::from_str(&config.apikey)?);

    Ok(map)
}

fn format_url(config: &Config, path: &str, protocol: &Protocols) -> String {
    format!("{}://{}/{}", protocol, config.server, path)
}

async fn check_success(res: Response) -> Result<String, Error> {
    let status = res.status();
    if status.is_success() {
        Ok(res.text().await?)
    } else if status.as_u16() == 401 {
        Err(Error::Authorization)
    } else {
        Err(Error::HttpStatus(status.as_u16()))
    }
}

fn add_pb(mp: &MultiProgress, template: &str, message: String) -> ProgressBar {
    let pb = mp.add(ProgressBar::new(1));
    pb.set_style(ProgressStyle::with_template(template).unwrap());
    pb.enable_steady_tick(Duration::from_millis(TICK_SPEED));
    pb.set_message(message);

    pb
}

fn finish_pb(pb: &ProgressBar, message: String, template: &str) {
    pb.set_style(ProgressStyle::with_template(template).unwrap());
    pb.finish_with_message(message);
}

enum Protocols {
    Http,
    Websocket,
}

impl Display for Protocols {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Http => f.write_str("http"),
            Self::Websocket => f.write_str("ws"),
        }
    }
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String,
}
