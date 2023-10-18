use serde::Deserialize;

use crate::{config::SETTINGS, error::CliError, default_headers};

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
        .map_err(CliError::Reqwest)?
        .text();

    let res = serde_json::from_str::<StartResponse>(&res.map_err(CliError::Reqwest)?).map_err(CliError::Serde)?;

    if res.boot {
        println!("successfully started {}", res.id);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct StartResponse {
    boot: bool,
    id: String,
}
