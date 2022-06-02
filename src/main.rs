use tokio::task;
use futures::future::join_all;

use std::path::PathBuf;

use clap::Parser;
use anyhow::Result;
use tokio::sync::mpsc::channel;

mod config;
mod errors;
mod connect;
mod message;
mod output;
mod utils;

pub mod chunk_capnp {
    include!(concat!(env!("OUT_DIR"), "/schema/chunk_capnp.rs"));
}

use output::Output;
use config::Config;
use connect::{CanTask, GpsTask};
use utils::can_devices;


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Config file
    #[clap(short, long, parse(from_os_str), value_name = "FILE")]
    config: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = match cli.config.as_deref() {
        Some(config_path) => {
            Config::from(config_path)
        },
        _ => {
            Config::default()
        }
    };

    let mut handles = Vec::new();
    let (source_tx, source_rx) = channel(65536);
    let mut output = Output::new(
        &config.id(), config.mqtt_config(), config.log_config(), source_rx);
    handles.push(task::spawn(async move {
        output.run().await;
    }));

    let devices = can_devices();
    for dev in devices {
        let out = source_tx.clone();
        let can_config = config.can_config();
        handles.push(task::spawn(async move {
            let mut can_task = CanTask::new(&dev, out, can_config.frequency);
            can_task.run().await;
        }));
    }

    let gps_config = config.gps_config();
    handles.push(task::spawn(async move {
        let task = GpsTask::new(&gps_config, source_tx);
        task.run().await;
    }));
   
    join_all(handles).await;

    Ok(())
}
