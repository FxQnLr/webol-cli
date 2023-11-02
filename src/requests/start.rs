use std::time::Duration;

use futures_util::{StreamExt, SinkExt};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{error::CliError, default_headers, ErrorResponse, format_url, Protocols};

pub async fn start(id: String, ping: bool) -> Result<(), CliError> {

    let send_start = ProgressBar::new(1);

    // TODO: calculate average start-time on server
    send_start.set_style(
        ProgressStyle::with_template("{spinner:.green} ({elapsed}) {wide_msg}")
        .unwrap()
        .tick_chars("|/-\\\\")
    );

    let url = format_url("start", Protocols::Http)?;

    send_start.set_message(format!("connect to {}", url));
    send_start.enable_steady_tick(Duration::from_millis(125));

    let res = reqwest::Client::new()
        .post(url)
        .headers(default_headers()?)
        .body(
            format!(r#"{{"id": "{}", "ping": {}}}"#, id, ping)
        )
        .send()
        .await
        .map_err(CliError::Reqwest)?;

    match res.status() {
        StatusCode::OK => {
            let body = serde_json::from_str::<StartResponse>(
                &res.text().await.map_err(CliError::Reqwest)?
            )
            .map_err(CliError::Serde)?;

            if body.boot {
                send_start.println("connected, sent start packet");
            }

            if ping {
                send_start.println(status_socket(body.uuid, &send_start).await?.to_string());
            }
        },
        _ => {
            let body = serde_json::from_str::<ErrorResponse>(
                &res.text().await.map_err(CliError::Reqwest)?
            )
            .map_err(CliError::Serde)?;

            println!("got error: {}", body.error);
        }
    }

    Ok(())
}

async fn status_socket(uuid: String, pb: &ProgressBar) -> Result<bool, CliError> {
    pb.set_message("setup websocket");

    let (mut ws_stream, _response) = connect_async(format_url("status", Protocols::Websocket)?)
        .await
        .expect("Failed to connect");
    pb.println("connected to websocket");

    pb.set_message("send uuid message");
    ws_stream.send(Message::Text(uuid)).await.unwrap();
    pb.println("sent uuid message");

    pb.set_message("wait for message");
    let msg = ws_stream.next().await.unwrap();

    pb.println(format!("msg: {:?}", msg));
    
    ws_stream.close(None).await.unwrap();
    pb.println("connection closed");
    // TODO: Check for correct UUID and timeout
    pb.set_message("verifying message");
    if msg.is_ok() { return Ok(true) }

    Ok(false)
}

#[derive(Debug, Deserialize)]
struct StartResponse {
    boot: bool,
    id: String,
    uuid: String,
}
