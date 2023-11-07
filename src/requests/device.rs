use crate::{error::CliError, default_headers, format_url, Protocols};

pub async fn put(id: String, mac: String, broadcast_addr: String, ip: String) -> Result<(), CliError> {
    let url = format_url("device", Protocols::Http)?;
    println!("{}", url);
    let res = reqwest::Client::new()
        .put(url)
        .headers(default_headers()?)
        .body(
            format!(
                r#"{{"id": "{}", "mac": "{}", "broadcast_addr": "{}", "ip": "{}"}}"#,
                id,
                mac,
                broadcast_addr,
                ip
            )
        )
        .send()
        .await
        .map_err(CliError::Reqwest)?
        .text()
        .await;

    println!("{:?}", res);
    Ok(())
}

pub async fn get(id: String) -> Result<(), CliError> {
    let res = reqwest::Client::new()
        .get(format_url("device", Protocols::Http)?)
        .headers(default_headers()?)
        .body(
            format!(r#"{{"id": "{}"}}"#, id)
        )
        .send()
        .await
        .map_err(CliError::Reqwest)?
        .text()
        .await;

    println!("{:?}", res);
    Ok(())
}

pub async fn post(id: String, mac: String, broadcast_addr: String, ip: String) -> Result<(), CliError> {
    let res = reqwest::Client::new()
        .post(format_url("device", Protocols::Http)?)
        .headers(default_headers()?)
        .body(
            format!(
                r#"{{"id": "{}", "mac": "{}", "broadcast_addr": "{}", "ip": "{}"}}"#,
                id,
                mac,
                broadcast_addr,
                ip
            )
        )
        .send()
        .await
        .map_err(CliError::Reqwest)?
        .text()
        .await;

    println!("{:?}", res);
    Ok(())
}
