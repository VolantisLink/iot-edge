use std::time::Duration;
use log::debug;
use rumqttc::{
    MqttOptions, 
    QoS, 
    EventLoop, 
    Publish, 
    Request,
    ConnectionError,
};

use crate::config::ConfigMqtt;
use crate::errors::IotEdgeError;

pub struct MqttOutput {
    topic: String,
    eventloop: EventLoop,
}

impl MqttOutput {
    pub fn new(id: &str, mqtt: &ConfigMqtt) -> Self {
        let mut options = MqttOptions::new(
            id,
            &mqtt.host,
            mqtt.port
        );
        options.set_keep_alive(Duration::from_secs(5));
        let eventloop = EventLoop::new(options, 10);

        MqttOutput {
            topic: mqtt.topic.clone(),
            eventloop
        }
    }

    pub async fn write(&mut self, data: Vec<u8>) -> Result<u16, IotEdgeError> {
        let mut publish = Publish::new(&self.topic, QoS::AtLeastOnce, data);
        publish.retain = false;
        let pkid = publish.pkid;

        let request = Request::Publish(publish);
        let tx = self.eventloop.handle();
        match tx.send_async(request).await {
            Ok(_) => Ok(pkid),
            Err(_) => Err(IotEdgeError::MqttPubAckError),
        }
    }

    pub async fn ack(&mut self) -> Result<(), ConnectionError> {
        match self.eventloop.poll().await {
            Ok(event) => {
                debug!("Received = {:?}", event);
                Ok(())
            },
            Err(e) => {
                Err(e)
            }
        }
    }
}
