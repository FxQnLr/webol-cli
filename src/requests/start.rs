use reqwest::StatusCode;
use serde::Deserialize;

use crate::{config::SETTINGS, error::CliError, default_headers, ErrorResponse};

pub fn start(id: String) -> Result<(), CliError> {
    let res = reqwest::blocking::Client::new()
        .post(
            format!(
                "{}/start",
                SETTINGS.get_string("server").map_err(CliError::Config)?
            )
        )
        .headers(default_headers()?)
        .body(
            format!(r#"{{"id": "{}"}}"#, id)
        )
        .send()
        .map_err(CliError::Reqwest)?;

    match res.status() {
        StatusCode::OK => {
            let body = serde_json::from_str::<StartResponse>(
                &res.text().map_err(CliError::Reqwest)?
            )
            .map_err(CliError::Serde)?;

            if body.boot {
                println!("successfully started {}", body.id);
            }
        },
        _ => {
            let body = serde_json::from_str::<ErrorResponse>(
                &res.text().map_err(CliError::Reqwest)?
            )
            .map_err(CliError::Serde)?;

            println!("got error: {}", body.error);
        }
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct StartResponse {
    boot: bool,
    id: String,
}
