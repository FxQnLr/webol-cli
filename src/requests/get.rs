use crate::{error::CliError, config::SETTINGS, default_headers};

pub fn get(id: String) -> Result<(), CliError> {
    let res = reqwest::blocking::Client::new()
        .get(
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

    println!("{:?}", res);
    Ok(())
}
