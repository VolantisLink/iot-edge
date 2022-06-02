use serde::ser::{Serialize, Serializer, SerializeStruct};
use chrono::prelude::*;
use socketcan::CANFrame;

#[derive(Debug, Clone)]
pub struct CanMessage {
    pub time: DateTime<Utc>,
    pub channel: String,
    pub frame: CANFrame,
}

impl From<&CanMessage> for String {
    fn from(msg: &CanMessage) -> Self {
        let mut can_hex_str = String::new();
        for b in msg.frame.data().iter() {
            can_hex_str.push_str(format!("{:02X}", b).as_ref());
        }

        match msg.frame.is_extended() {
            true => format!(
                "({}.{:06}) {} {:08X}#{}",
                msg.time.timestamp(),
                msg.time.timestamp_subsec_micros(), 
                msg.channel, 
                msg.frame.id(), 
                can_hex_str
            ),
            false => format!(
                "({}.{:06}) {} {:03X}#{}",
                msg.time.timestamp(), 
                msg.time.timestamp_subsec_micros(), 
                msg.channel, 
                msg.frame.id(), 
                can_hex_str
            ),
        }
    }
}

impl Serialize for CanMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("CanMessage", 1)?;
        state.serialize_field("dump", &String::from(self))?;
        state.end()
    }
}

#[test]
fn test_json() {
    use socketcan::dump::Reader;

    let input: &[u8] = b"(1469439874.299591) can1 080#\n\
                        (1469439874.299654) can1 701#7F";

    let mut reader = Reader::from_reader(input);
    for record in reader.records() {
        println!("{:?}", record);
        let r = record.unwrap();
        let (secs, nsecs) = {
            let secs = ((r.0 as f64) / 1000_000f64) as f64;
            let t1 = secs as i64;
            let t2 = ((secs - t1 as f64) * 1000_000_000f64) as u32;

            (t1, t2)
        };
        let ts = NaiveDateTime::from_timestamp(secs, nsecs);
        println!("{:#?}, {}.{}", ts, secs, nsecs);
        let msg = CanMessage {
            time: DateTime::<Utc>::from_utc(ts, Utc),
            channel: "can1".to_string(),
            frame: r.1,
        };
        println!("{:?}", msg);
        let s = serde_json::to_string(&msg).unwrap();
        println!("{}", s);
    }
}
