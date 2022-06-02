use tokio::sync::mpsc::Sender;
use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use tokio::io::{BufReader, AsyncBufReadExt};
use log::*;
use serde_json::json;
use unbounded_gpsd::types::*;

use crate::message::{
    Message,
    GpsMessage,
};
use crate::config::ConfigGps;

pub struct GpsTask {
    host: String,
    port: u16,
    tx: Sender<Message>,
}

impl GpsTask {
    pub fn new(config: &ConfigGps, tx: Sender<Message>) -> Self {
        GpsTask {
            host: config.host.to_string(),
            port: config.port,
            tx,
        }
    }

    pub async fn run(&self) {
        let addr = format!("{}:{}", self.host, self.port);
        let mut stream = TcpStream::connect(&addr).await.unwrap();
        let watch_data = json!({
            "class": "WATCH",
            "enable": true,
            "json": true,
            "raw": 0u8,
        });
        let msg = format!("?WATCH={}\n", watch_data.to_string());
        stream.write_all(msg.as_bytes()).await.unwrap();
    
        let mut inner = BufReader::new(stream);
    
        loop {
            let mut buf = String::new();
            let read_result = inner.read_line(&mut buf).await;
    
            if let Ok(size) = read_result {
                if size == 0 {
                    error!("Gpsd Connection Closed");
                }
            }
    
            if buf == "" {
                debug!("empty line received from GPSD");
                continue;
            }
    
            let data = serde_json::from_str::<Response>(&buf);
            debug!("serde output: {:?}", data);
            match data {
                Err(e) => {
                    warn!("deserializing response failed: {:?}, buf: {}", e, buf);
                },
                Ok(response) => {
                    match response {
                        Response::Tpv(resp) => {
                            let gps_msg = match GpsMessage::try_from(&resp) {
                                Ok(msg) => msg,
                                _ => continue
                            };
                            self.tx.send(Message::GPS(gps_msg)).await.unwrap();
                        },
                        _ => {}
                    }
                }
            }
        }    
    }
}
