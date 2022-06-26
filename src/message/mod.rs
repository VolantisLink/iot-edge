use std::vec::Vec;
use serde::Serialize;
use chrono::prelude::*;
use capnp::message::Builder;
use capnp::serialize_packed;

use crate::chunk_capnp;

mod can;
mod gps;

pub use can::CanMessage;
pub use gps::GpsMessage;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum Message {
    GPS(GpsMessage),
    CAN(CanMessage),
}

#[derive(Debug, Clone, Serialize)]
pub struct Chunk {
    pub time: DateTime<Utc>,
    id: String,   // identifier for this chunk
    can: Vec<CanMessage>,
    gps: Vec<GpsMessage>,
}

impl Chunk {
    pub fn new(id: &str) -> Self {
        let time = Utc::now();

        Chunk { 
            time, 
            id: id.to_string(), 
            can: Vec::new(),
            gps: Vec::new()
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn push(&mut self, msg: Message) {
        match msg {
            Message::CAN(msg) => self.can.push(msg),
            Message::GPS(msg) => self.gps.push(msg)
        }
    }

    pub fn len(&self) -> usize {
        self.can.len() + self.gps.len()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut builder = Builder::new_default();
        let mut root = builder.init_root::<chunk_capnp::chunk::Builder>();

        root.set_id(&self.id);
        root.set_time((self.time.timestamp_nanos() as f64) / 1000_000_000f64);

        let mut can_messages = root.reborrow().init_can(self.can.len() as u32);
        for (pos, msg) in self.can.iter().enumerate() {
            let mut can = can_messages.reborrow().get(pos as u32);
            let ts = (msg.time.timestamp_nanos() as f64) / 1000_000_000f64;
            can.set_time(ts);
            can.set_channel(&msg.channel);
            can.set_id(msg.frame.id());
            can.set_error(msg.frame.is_error());
            can.set_remote(msg.frame.is_rtr());
            can.set_extended(msg.frame.is_extended());
            can.set_data(msg.frame.data());
            can.set_length(msg.frame.data().len() as u8);

        }

        let mut gps_messages = root.reborrow().init_gps(self.gps.len() as u32);
        for (pos, msg) in self.gps.iter().enumerate() {
            let mut gps = gps_messages.reborrow().get(pos as u32);
            let ts = (msg.time.timestamp_nanos() as f64) / 1000_000_000f64;
            gps.set_time(ts);
            gps.set_longitude(msg.longitude);
            gps.set_latitude(msg.latitude);
            gps.set_speed(msg.speed);
        }

        let mut buf = Vec::new();
        serialize_packed::write_message(&mut buf, &builder).unwrap();

        buf
    }
}


#[test]
fn test_json() {
    use socketcan::dump::Reader;
    use chrono::prelude::*;

    let msg = GpsMessage {
        time: Utc::now(),
        latitude: 0.1,
        longitude: -0.1,
        speed: 100.0,
    };
    let gps_msgs = vec![msg];

    let mut can_msgs = Vec::new();
    let input: &[u8] = b"(1655098589.035226) can1 202#A1000000000000A1";
    let mut reader = Reader::from_reader(input);
    for record in reader.records() {
        let r = record.unwrap();
        let ts = Utc::now();
        let msg = CanMessage {
            time: ts,
            channel: "can1".to_string(),
            frame: r.1,
        };
        can_msgs.push(msg);
    }

    let line = Chunk {
        time: Utc::now(),
        id: "test".to_string(),
        can: can_msgs,
        gps: gps_msgs,
    };
    let line_str = serde_json::to_string(&line).unwrap();
    println!("{}", line_str);
}


#[test]
fn test_time() {
    let ts = Local::now();
    let offset = ts.offset();
    println!("{}", offset.local_minus_utc() / 3600);
}

#[test]
fn test_capnp() {
    use chrono::prelude::*;

    let now = Utc::now();
    let ts = now.timestamp_nanos() as f64 / 1000_000_000f64;
    
    let mut message = ::capnp::message::Builder::new_default();
    let mut root = message.init_root::<chunk_capnp::chunk::Builder<'_>>();
    root.set_id("value");

    let mut gps_messages = root.init_gps(1);
    let mut gps = gps_messages.reborrow().get(0);
    gps.set_time(ts);
    gps.set_latitude(-10.01);
    gps.set_longitude(10.102);
    gps.set_speed(100.0);

    let mut buf = Vec::new();
    serialize_packed::write_message(&mut buf, &message).unwrap();

    println!("{:?}", buf);
}