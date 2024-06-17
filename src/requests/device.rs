use crate::{check_success, config::Config, default_headers, error::Error, format_url, Protocols};

pub async fn put(
    config: &Config,
    id: String,
    mac: String,
    broadcast_addr: String,
    ip: String,
) -> Result<(), Error> {
    let url = format_url(config, "device", &Protocols::Http, None);
    println!("{url}");
    let res = reqwest::Client::new()
        .put(url)
        .headers(default_headers(config)?)
        .body(format!(
            r#"{{"id": "{id}", "mac": "{mac}", "broadcast_addr": "{broadcast_addr}", "ip": "{ip}"}}"#,
        ))
        .send()
        .await?;

    let body = check_success(res).await?;
    println!("{body}");
    Ok(())
}

pub async fn get(config: &Config, id: String) -> Result<(), Error> {
    let res = reqwest::Client::new()
        .get(format_url(config, "device", &Protocols::Http, Some(&id)))
        .headers(default_headers(config)?)
        .send()
        .await?;

    let body = check_success(res).await?;
    println!("{body}");
    Ok(())
}

pub async fn post(
    config: &Config,
    id: String,
    mac: String,
    broadcast_addr: String,
    ip: String,
) -> Result<(), Error> {
    let res = reqwest::Client::new()
        .post(format_url(config, "device", &Protocols::Http, None))
        .headers(default_headers(config)?)
        .body(format!(
            r#"{{"id": "{id}", "mac": "{mac}", "broadcast_addr": "{broadcast_addr}", "ip": "{ip}"}}"#,
        ))
        .send()
        .await?;

    let body = check_success(res).await?;
    println!("{body}");
    Ok(())
}
