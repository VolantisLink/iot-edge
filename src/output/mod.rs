use log::error;
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
        }
    }

    async fn send(&mut self, buf: Vec<Message>) {
        let chunk: Chunk = Chunk::new(&self.id, buf);
        let line = chunk.to_json();

        self.logger.write(&line);

        let data = match self.mqtt_config.encoder {
            Encoder::BINARY => chunk.to_vec(),
            Encoder::JSON => line.into_bytes(),
        };

        if let Err(e) = self.mqtt.write(data).await {
            error!("{}", e);
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
                    if let Err(e) = ack {
                        error!("{}", e);
                    }
                }
                _ = interval.tick() => { // tick
                    let now = Local::now();
                    if (buf.len() > 0) & (now.timestamp() - checkpoint.timestamp() > self.mqtt_config.chunk_period) {
                        self.send(buf).await;
                        buf = Vec::new();
                        checkpoint = now;
                    }                    
                }
            }
        }
    }
}