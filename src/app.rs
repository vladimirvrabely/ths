use std::time::Duration;
use chrono::Utc;
use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::{IntervalStream, ReceiverStream};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::modbus::SensorReader;
use crate::model::Measurement;

pub async fn run() {
    init_env_logging();

    tracing::info!("Starting temperature/humidity sensor service");

    let tty_path = String::from("/dev/ttyUSB0");
    let period = Duration::from_secs(5);

    let (tx, rx) = mpsc::channel(32);

    tracing::info!("Spawning modbus read task");
    let _modbus_read_task = spawn_modbus_read_task(tty_path, period, tx);

    tracing::info!("Spawning measurement write task");
    let _measurement_write_task = spawn_measurement_write_task(rx);

    tracing::info!("Waiting for termination signal");
    let _ = catch_terminate_signal().recv().await;
}

fn spawn_modbus_read_task(tty_path: String, period: Duration, tx: mpsc::Sender<Measurement>) -> JoinHandle<()> {

    tokio::spawn(async move {
        let mut sensor_reader = SensorReader::new(&tty_path).expect("Modbus reader should be created");


        // Wait until roundish time
        let now = Utc::now();
        let wait = (now.timestamp_millis() as u64) % (period.as_millis() as u64);
        let wait = period - Duration::from_millis(wait);
        tokio::time::sleep(wait).await;

        let interval = tokio::time::interval(period);
        let mut stream = IntervalStream::new(interval);

        while let Some(_instant) = stream.next().await {
            match sensor_reader.read().await {
                Ok(measurement) => {
                    let _ = tx.send(measurement).await;
                }
                Err(error) => {
                    tracing::error!("Sensor reading error - {}", error)
                }
            }
        }
    })
}

fn spawn_measurement_write_task(rx: mpsc::Receiver<Measurement>) -> JoinHandle<()> {
    tokio::spawn(async move {
        let stream = ReceiverStream::new(rx);
        tokio::pin!(stream);
        while let Some(measurement) = stream.next().await {
            println!("{measurement:?}");
        }
    })
}

fn init_env_logging() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();
}

fn catch_terminate_signal() -> mpsc::Receiver<Option<()>> {
    let (stop_tx, stop_rx) = mpsc::channel(1);

    tokio::spawn(async move {
        let stop = signal(SignalKind::terminate())
            .expect("should create SIGTERM listener")
            .recv()
            .await;

        tracing::info!("Received stop signal");

        stop_tx.send(stop).await.unwrap();
    });

    stop_rx
}
