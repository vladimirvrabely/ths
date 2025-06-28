use crate::modbus::SensorReader;
use std::error::Error;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub async fn run() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    tracing::info!("Starting temperature/humidity sensor service");

    let tty_path = "/dev/ttyUSB0";

    let mut sensor_reader = SensorReader::new(tty_path).expect("Modbus reader should be created");

    loop {
        match sensor_reader.read().await {
            Ok(measurement) => {
                println!("{:?}", measurement);
            }
            Err(error) => {
                tracing::error!("Sensor reading error - {}", error)
            }
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    }
}
