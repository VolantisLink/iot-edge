use log::warn;
use tokio::sync::mpsc::Sender;
use chrono::prelude::*;

use futures_util::stream::StreamExt;
use tokio_socketcan::CANSocket;

use crate::message::{
    Message,
    CanMessage,
};

pub struct CanTask {
    tx: Sender<Message>,
    bus: CANSocket,
    freq: u16,
    dev: String,
}

impl CanTask {
    pub fn new(ifname: &str, tx: Sender<Message>, freq: u16) -> Self {
        CanTask {
            dev: ifname.to_string(),
            bus: CANSocket::open(ifname).unwrap(),
            freq,
            tx,
        }
    }

    pub async fn run(&mut self) {
        let interval = 1000 / self.freq as i64;
        let mut current: DateTime<Utc> = Utc::now();

        while let Some(Ok(frame)) = self.bus.next().await {
            let time: DateTime<Utc> = Utc::now();
            if time.timestamp_millis() / interval == current.timestamp_millis() / interval {
                continue;
            }
    
            let msg = CanMessage {
                time,
                channel: self.dev.clone(),
                frame
            };
            
            match self.tx.send(Message::CAN(msg)).await {
                Ok(()) => {},
                Err(e) => { warn!("{:?}", e) }
            }
            current = time;
        }
    }
}