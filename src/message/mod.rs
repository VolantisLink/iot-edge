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
    time: DateTime<Utc>,
    id: String,   // identifier for this chunk
    messages: Vec<Message>,
}

impl Chunk {
    pub fn new(id: &str, messages: Vec<Message>) -> Self {
        let time = Utc::now();

        Chunk { 
            time, 
            id: id.to_string(), 
            messages,
        }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_vec(&self) -> Vec<u8> {
        let mut builder = Builder::new_default();
        let mut root = builder.init_root::<chunk_capnp::chunk::Builder>();

        root.set_id(&self.id);
        root.set_time((self.time.timestamp_nanos() as f64) / 1000_000_000f64);

        let mut entries = root.init_entries(self.messages.len() as u32);        
        for (pos, msg) in self.messages.iter().enumerate() {
            let mut entry = entries.reborrow().get(pos as u32);
            
            match msg {
                Message::CAN(msg) => {
                    let ts = (msg.time.timestamp_nanos() as f64) / 1000_000_000f64;
                    entry.set_time(ts);

                    let mut can = entry.init_can();
                    can.set_channel(&msg.channel);
                    can.set_id(msg.frame.id());
                    can.set_error(msg.frame.is_error());
                    can.set_remote(msg.frame.is_rtr());
                    can.set_extended(msg.frame.is_extended());
                    can.set_data(msg.frame.data());
                    can.set_length(msg.frame.data().len() as u8);
                },
                Message::GPS(msg) => {
                    let ts = (msg.time.timestamp_nanos() as f64) / 1000_000_000f64;
                    entry.set_time(ts);

                    let mut gps = entry.init_gps();
                    gps.set_longitude(msg.longitude);
                    gps.set_latitude(msg.latitude);
                    gps.set_speed(msg.speed);
                },
            }
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

    let mut messages = Vec::new();
    let gps = GpsMessage {
        time: Utc::now(),
        latitude: 0.1,
        longitude: -0.1,
        speed: 100.0,
    };
    let gps_msg = Message::GPS(gps);
    messages.push(gps_msg);

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
        let can_msg = Message::CAN(msg);
        messages.push(can_msg);
    }

    let line = Chunk {
        time: Utc::now(),
        id: "test".to_string(),
        messages,
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

    let mut entries = root.init_entries(1);        
    let mut entry = entries.reborrow().get(0);
    entry.set_time(ts);
    let mut gps = entry.init_gps();
    gps.set_latitude(-10.01);
    gps.set_longitude(10.102);
    gps.set_speed(100.0);

    let mut buf = Vec::new();
    serialize_packed::write_message(&mut buf, &message).unwrap();

    println!("{:?}", buf);
}