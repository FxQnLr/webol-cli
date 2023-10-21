use crate::{error::CliError, default_headers, format_url};

pub fn put(id: String, mac: String, broadcast_addr: String) -> Result<(), CliError> {
    let res = reqwest::blocking::Client::new()
        .put(format_url("device")?)
        .headers(default_headers()?)
        .body(
            format!(
                r#"{{"id": "{}", "mac": "{}", "broadcast_addr": "{}"}}"#,
                id,
                mac,
                broadcast_addr
            )
        )
        .send()
        .map_err(CliError::Reqwest)?
        .text();

    println!("{:?}", res);
    Ok(())
}

pub fn get(id: String) -> Result<(), CliError> {
    let res = reqwest::blocking::Client::new()
        .get(format_url("device")?)
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

pub fn post(id: String, mac: String, broadcast_addr: String) -> Result<(), CliError> {
    let res = reqwest::blocking::Client::new()
        .post(format_url("device")?)
        .headers(default_headers()?)
        .body(
            format!(
                r#"{{"id": "{}", "mac": "{}", "broadcast_addr": "{}"}}"#,
                id,
                mac,
                broadcast_addr
            )
        )
        .send()
        .map_err(CliError::Reqwest)?
        .text();

    println!("{:?}", res);
    Ok(())
}
