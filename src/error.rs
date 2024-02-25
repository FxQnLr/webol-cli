use std::{fmt::Debug, num::ParseIntError};

use reqwest::header::InvalidHeaderValue;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("request: {source}")]
    Reqwest {
        #[from]
        source: reqwest::Error,
    },
    #[error("config: {source}")]
    Config {
        #[from]
        source: config::ConfigError,
    },
    #[error("serde: {source}")]
    Serde {
        #[from]
        source: serde_json::Error,
    },
    #[error("parse int: {source}")]
    Parse {
        #[from]
        source: ParseIntError,
    },
    #[error("parse header: {source}")]
    InvalidHeaderValue {
        #[from]
        source: InvalidHeaderValue,
    },
    #[error("tungstenite: {source}")]
    Tungstenite {
        #[from]
        source: tokio_tungstenite::tungstenite::Error,
    },
    #[error("faulty websocket response")]
    WsResponse,
    #[error("authorization failed")]
    Authorization,
    #[error("Http error status: {0}")]
    HttpStatus(u16),
}
