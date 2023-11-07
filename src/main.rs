use std::{fmt::Display, time::Duration};

use clap::{Parser, Subcommand};
use config::SETTINGS;
use error::CliError;
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};
use requests::{start::start, device};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;

mod config;
mod error;
mod requests;

static OVERVIEW_STYLE: &str = "{spinner:.green} {wide_msg}({elapsed})";
static OVERVIEW_ERROR: &str = "✗ {wide_msg}({elapsed})";
static OVERVIEW_DONE: &str = "✓ {wide_msg}({elapsed})";
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
        ping: Option<bool>
    },
    Device {
        #[command(subcommand)]
        devicecmd: DeviceCmd,
    }
}

#[derive(Subcommand)]
enum DeviceCmd {
    Add {
        id: String,
        mac: String,
        broadcast_addr: String,
        ip: String
    },
    Get {
        id: String,
    },
    Edit {
        id: String,
        mac: String,
        broadcast_addr: String,
        ip: String
    },
}

#[tokio::main]
async fn main() -> Result<(), CliError> {
    let cli = Args::parse();

    match cli.commands {
        Commands::Start { id, ping } => {
            start(id, ping.unwrap_or(true)).await?;
        },
        Commands::Device { devicecmd } => {
            match devicecmd {
                DeviceCmd::Add { id, mac, broadcast_addr, ip } => {
                    device::put(id, mac, broadcast_addr, ip).await?;
                },
                DeviceCmd::Get { id } => {
                    device::get(id).await?;
                },
                DeviceCmd::Edit { id, mac, broadcast_addr, ip } => {
                    device::post(id, mac, broadcast_addr, ip).await?;
                },
            }
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

fn format_url(path: &str, protocol: Protocols) -> Result<String, CliError> {
    Ok(format!(
        "{}://{}/{}",
        protocol,
        SETTINGS.get_string("server").map_err(CliError::Config)?,
        path
    ))
}

fn add_pb(mp: &MultiProgress, template: &str, message: String) -> ProgressBar {
    let pb = mp.add(ProgressBar::new(1));
    pb.set_style(ProgressStyle::with_template(template).unwrap());
    pb.enable_steady_tick(Duration::from_millis(TICK_SPEED));
    pb.set_message(message);

    pb
}

fn finish_pb(pb: ProgressBar, message: String, template: &str) {
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
            Self::Websocket => f.write_str("ws")
        }
    }
}

#[derive(Debug, Deserialize)]
struct ErrorResponse {
    error: String
}
