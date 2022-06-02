use serde::ser::{Serialize, Serializer, SerializeStruct};
use chrono::prelude::*;
use unbounded_gpsd::types::TpvResponse;


#[derive(Debug, Clone)]
pub struct GpsMessage {
    pub time: DateTime<Utc>,
    pub longitude: f64,
    pub latitude: f64,
    pub speed: f64,
}

impl TryFrom<&TpvResponse> for GpsMessage {
    type Error = &'static str;

    fn try_from(resp: &TpvResponse) -> Result<Self, Self::Error> {
        match resp {
            TpvResponse::Fix2D{time, lat, lon, speed, ..} => {
                Ok(GpsMessage {
                    time: *time,
                    latitude: *lat,
                    longitude: *lon,
                    speed: *speed,
                })
            },
            TpvResponse::Fix3D {time, lat, lon, speed, .. } => {
                Ok(GpsMessage {
                    time: *time,
                    latitude: *lat,
                    longitude: *lon,
                    speed: *speed,
                })
            },
            TpvResponse::LatLonOnly {time, lat, lon, speed, .. } => {
                let speed = match speed {
                    Some(speed) => *speed,
                    _ => 0.0
                };

                Ok(GpsMessage {
                    time: *time,
                    latitude: *lat,
                    longitude: *lon,
                    speed,
                })
            },
            _ => Err("No used message")
        }
    }
}

impl Serialize for GpsMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("GpsMessage", 4)?;
        state.serialize_field("ts", &self.time)?;
        state.serialize_field("lon", &self.longitude)?;
        state.serialize_field("lat", &self.latitude)?;
        state.serialize_field("speed", &self.speed)?;
        state.end()
    }
}

#[test]
fn test_json() {
    let msg = GpsMessage {
        time: Utc::now(),
        latitude: 0.1,
        longitude: -0.1,
        speed: 100.0,
    };
    let s = serde_json::to_string(&msg).unwrap();
    println!("{}", &s);
}