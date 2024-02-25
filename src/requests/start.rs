use futures_util::{SinkExt, StreamExt};
use indicatif::{MultiProgress, ProgressBar};
use reqwest::StatusCode;
use serde::Deserialize;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{http::Request, Message},
};

use crate::{
    add_pb, config::Config, default_headers, error::Error, finish_pb, format_url, ErrorResponse,
    Protocols, DEFAULT_STYLE, DONE_STYLE, ERROR_STYLE, OVERVIEW_DONE, OVERVIEW_ERROR,
    OVERVIEW_STYLE,
};

pub async fn start(config: &Config, id: String, ping: bool) -> Result<(), Error> {
    let send_start = MultiProgress::new();
    let overview = add_pb(&send_start, OVERVIEW_STYLE, format!(") start {id}"));

    let url = format_url(config, "start", &Protocols::Http);
    let connect = add_pb(&send_start, DEFAULT_STYLE, format!("connect to {url}"));
    let res = reqwest::Client::new()
        .post(url)
        .headers(default_headers(config)?)
        .body(format!(r#"{{"id": "{id}", "ping": {ping}}}"#))
        .send()
        .await?;
    finish_pb(&connect, "connected, got response".to_string(), DONE_STYLE);

    let res_pb = add_pb(&send_start, DEFAULT_STYLE, "analyzing response".to_string());

    if res.status() == StatusCode::OK {
        let body = serde_json::from_str::<StartResponse>(&res.text().await?)?;

        if body.boot {
            finish_pb(&res_pb, "sent start packet".to_string(), DONE_STYLE);
        }

        if ping {
            let status = status_socket(config, body.uuid, &send_start, &overview, id).await?;
            if status {
                finish_pb(
                    &overview,
                    format!("successfully started {}", body.id),
                    OVERVIEW_DONE,
                );
            } else {
                finish_pb(
                    &overview,
                    format!("error while starting {}", body.id),
                    OVERVIEW_ERROR,
                );
            }
        }
    } else {
        let body = serde_json::from_str::<ErrorResponse>(&res.text().await?)?;

        res_pb.finish_with_message(format!("✗ got error: {}", body.error));
    }

    Ok(())
}

async fn status_socket(
    config: &Config,
    uuid: String,
    pb: &MultiProgress,
    overview: &ProgressBar,
    id: String,
) -> Result<bool, Error> {
    let ws_pb = add_pb(pb, DEFAULT_STYLE, "connect to websocket".to_string());

    let request = Request::builder()
        .uri(format_url(config, "status", &Protocols::Websocket))
        .header("Authorization", &config.apikey)
        .header("sec-websocket-key", "")
        .header("host", &config.server)
        .header("upgrade", "websocket")
        .header("connection", "upgrade")
        .header("sec-websocket-version", 13)
        .body(())
        .unwrap();

    let (mut ws_stream, _response) = connect_async(request).await?;
    finish_pb(&ws_pb, "connected to websocket".to_string(), DONE_STYLE);

    ws_stream.send(Message::Text(uuid.clone())).await.unwrap();

    // Get ETA
    let eta_msg = ws_stream.next().await.unwrap().unwrap();
    let eta = get_eta(&eta_msg.into_text().unwrap(), &uuid)?;
    overview.set_message(format!("/{eta}) start {id}"));

    let msg_pb = add_pb(pb, DEFAULT_STYLE, "await message".to_string());
    let msg = ws_stream.next().await.unwrap();
    finish_pb(&msg_pb, "received message".to_string(), DONE_STYLE);

    ws_stream.close(None).await.unwrap();

    let v_pb = add_pb(pb, DEFAULT_STYLE, "verify response".to_string());
    let res = verify_response(&msg.unwrap().to_string(), &uuid)?;
    match res {
        Verified::WrongUuid => {
            finish_pb(&v_pb, "returned wrong uuid".to_string(), ERROR_STYLE);
            Ok(false)
        }
        Verified::ResponseType(res_type) => match res_type {
            ResponseType::Start => {
                finish_pb(&v_pb, "device started".to_string(), DONE_STYLE);
                Ok(true)
            }
            ResponseType::Timeout => {
                finish_pb(&v_pb, "ping timed out".to_string(), ERROR_STYLE);
                Ok(false)
            }
            ResponseType::NotFound => {
                finish_pb(&v_pb, "unknown uuid".to_string(), ERROR_STYLE);
                Ok(false)
            }
        },
    }
}

fn get_eta(msg: &str, uuid: &str) -> Result<String, Error> {
    let spl: Vec<&str> = msg.split('_').collect();
    if (spl[0] != "eta") || (spl[2] != uuid) {
        return Err(Error::WsResponse);
    };
    let input: u64 = spl[1].parse()?;

    let sec = input % 60;
    let min = (input / 60) % 60;
    let hou = (input / (60 * 60)) % 60;

    Ok(format!("{hou:0>2}:{min:0>2}:{sec:0>2}"))
}

fn verify_response(res: &str, org_uuid: &str) -> Result<Verified, Error> {
    let spl: Vec<&str> = res.split('_').collect();
    let res_type = spl[0];
    let uuid = spl[1];

    if uuid != org_uuid {
        return Ok(Verified::WrongUuid);
    };

    Ok(Verified::ResponseType(ResponseType::from(res_type)?))
}

#[derive(Debug, Deserialize)]
struct StartResponse {
    boot: bool,
    id: String,
    uuid: String,
}

enum Verified {
    ResponseType(ResponseType),
    WrongUuid,
}

enum ResponseType {
    Start,
    Timeout,
    NotFound,
}

impl ResponseType {
    fn from(value: &str) -> Result<Self, Error> {
        match value {
            "start" => Ok(ResponseType::Start),
            "timeout" => Ok(ResponseType::Timeout),
            "notfound" => Ok(ResponseType::NotFound),
            _ => Err(Error::WsResponse),
        }
    }
}
