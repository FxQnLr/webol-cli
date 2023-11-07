use std::fmt::Debug;

pub enum CliError {
    Reqwest(reqwest::Error),
    Config(config::ConfigError),
    Serde(serde_json::Error),
    WsResponse,
}

impl Debug for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Reqwest(err) => { err.fmt(f) },
            Self::Config(err) => { err.fmt(f) },
            Self::Serde(err) => { err.fmt(f) },
            Self::WsResponse => { f.write_str("Error in Response") },
        }
    }
}
