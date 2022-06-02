use std::collections::hash_map::HashMap;
use tokio::time::{self, Duration};
use chrono::prelude::*;

mod file;
mod mqtt;

pub use crate::output::file::FileLogger;
pub use crate::output::mqtt::MqttOutput;

use tokio::{
    select,
    sync::{
        mpsc::Receiver,
    }
};
use crate::config::{ConfigMqtt, ConfigLog, Encoder};
use crate::message::{Message, Chunk};


pub struct Output {
    id: String,
    mqtt_config: ConfigMqtt,

    rx: Receiver<Message>,

    mqtt: MqttOutput,
    logger: FileLogger,
    chunks: HashMap<u16, Chunk>,
    times: Vec::<(DateTime<Local>, u16)>,
}

impl Output {
    pub fn new(
        id: &str, mqtt_config: ConfigMqtt, log_config: ConfigLog, rx: Receiver<Message>
    ) -> Self {
        let mqtt = MqttOutput::new(id, &mqtt_config);
        let logger = FileLogger::from(&log_config);

        Output {
            id: id.to_string(),
            mqtt_config,

            rx,

            mqtt,
            logger,
            chunks: HashMap::<u16, Chunk>::new(),
            times: Vec::<(DateTime<Local>, u16)>::new(),
        }
    }

    async fn send(&mut self, buf: Vec<Message>) {
        let mut chunk: Chunk = Chunk::new(&self.id, buf);
        let data = match self.mqtt_config.encoder {
            Encoder::BINARY => chunk.to_vec(),
            Encoder::JSON => chunk.to_json().into_bytes(),
        };

        if let Ok(pkid) = self.mqtt.write(data).await {
            self.chunks.insert(pkid, chunk);
        } else {
            chunk.set_synced(false);
            self.logger.write(&chunk);
        }
    }

    pub async fn run(&mut self) {
        let mut checkpoint = Local::now();
        let mut interval = time::interval(Duration::from_secs(1));
        let mut buf = Vec::new();

        loop {
            select! {
                msg = self.rx.recv() => {
                    if let Some(msg) = msg {
                        buf.push(msg);
                        if buf.len() >= self.mqtt_config.chunk_size {
                            self.send(buf).await;
                            buf = Vec::new();
                            checkpoint = Local::now();
                        } 
                    }
                }
                ack = self.mqtt.ack() => {
                    if let Some(pkid) = ack {
                        self.times.retain(|(_, p)| {
                            if *p == pkid {
                                if let Some(chunk) = self.chunks.remove(p) {
                                    self.logger.write(&chunk);
                                }

                                false
                            } else {
                                true
                            }
                        });
                    }
                }
                _ = interval.tick() => { // tick
                    let now = Local::now();
                    if (buf.len() > 0) & (now.timestamp() - checkpoint.timestamp() > self.mqtt_config.chunk_period) {
                        self.send(buf).await;
                        buf = Vec::new();
                        checkpoint = now;
                    }
                    
                    self.times.retain(|(t, p)| {
                        let dur = now.timestamp() - t.timestamp();
                        if dur > 120 {
                            if let Some(mut chunk) = self.chunks.remove(p) {
                                chunk.set_synced(false);
                                self.logger.write(&chunk);
                            }

                            false
                        } else {
                            true
                        }
                    });
                }
            }
        }
    }
}