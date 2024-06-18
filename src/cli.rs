use clap::{arg, command, Parser, Subcommand};
use clap_complete::{generate, Generator, Shell};

/// webol client
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub commands: Commands,

    #[arg(long)]
    pub server: Option<String>,

    #[arg(short, long)]
    pub secret: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
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
pub enum DeviceCmd {
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

pub fn print_completions<G: Generator>(gen: G, cmd: &mut clap::Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}
