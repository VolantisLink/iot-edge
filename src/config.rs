use std::path::Path;
use toml;
use serde_derive::{
    Serialize,
    Deserialize
};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigCan {
    pub frequency: u16,
}

impl Default for ConfigCan {
    fn default() -> Self {
        ConfigCan {
            frequency: 100,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigLog {
    pub path: String,
    pub rotate_size: String,
    pub rotate_keep: usize,
    pub rotate_compress: bool,
    pub include_success: bool,
}

impl Default for ConfigLog {
    fn default() -> Self {
        ConfigLog { 
            path: "iot-edge.log".to_string(), 
            rotate_size: "100M".to_string(), 
            rotate_keep: 7, 
            rotate_compress: false,
            include_success: true,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigGps {
    pub host: String,
    pub port: u16,
}

impl Default for ConfigGps {
    fn default() -> Self {
        ConfigGps { host: "127.0.0.1".to_string(), port: 2947}
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Encoder {
    JSON,
    BINARY,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigMqtt {
    pub host: String,
    pub port: u16,
    pub topic: String,
    pub encoder: Encoder,
    pub chunk_size: usize,
    pub chunk_period: i64,
}

impl Default for ConfigMqtt {
    fn default() -> Self {
        ConfigMqtt { 
            host: "127.0.0.1".to_string(), 
            port: 1883,
            topic: "test".to_string(),
            encoder: Encoder::BINARY, 
            chunk_size: 2048,
            chunk_period: 5,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub device_id: String,
    pub log: Option<ConfigLog>,
    pub can: Option<ConfigCan>,
    pub gps: Option<ConfigGps>,
    pub mqtt: Option<ConfigMqtt>,
}

impl Config {
    pub fn id(&self) -> String {
        self.device_id.clone()
    }
    pub fn gps_config(&self) -> ConfigGps {
        match &self.gps {
            Some(config) => config.clone(),
            None => ConfigGps::default(),
        }
    }
    pub fn can_config(&self) -> ConfigCan {
        match &self.can {
            Some(config) => config.clone(),
            None => ConfigCan::default(),
        }
    }
    pub fn log_config(&self) -> ConfigLog {
        match &self.log {
            Some(config) => config.clone(),
            None => ConfigLog::default(),
        }
    }
    pub fn mqtt_config(&self) -> ConfigMqtt {
        match &self.mqtt {
            Some(config) => config.clone(),
            None => ConfigMqtt::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            device_id: "test".to_string(),
            log: Some(ConfigLog::default()),
            can: Some(ConfigCan::default()),
            mqtt: Some(ConfigMqtt::default()),
            gps: Some(ConfigGps::default()),
        }
    }
}

impl From<&Path> for Config {
    fn from(path: &Path) -> Self {
        toml::from_str(&std::fs::read_to_string(path).unwrap()).unwrap()
    }
}

#[test]
fn test_config() {
    let config: Config = toml::from_str(r#"
    device_id = "test1"

    [mqtt]
    host = "127.0.0.1"
    port = 1883
    topic = "hello/test"
    encoder = "JSON"
    chunk_size = 2048
    
    [gps]
    host = "127.0.0.1"
    port = 2947
    "#).unwrap();
    println!("{:#?}", config);

    let config = Config::from(Path::new("config.toml.sample"));
    println!("{:#?}", config);
}
