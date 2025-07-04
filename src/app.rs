use std::time::Duration;

use tokio::signal::unix::{SignalKind, signal};
use tokio::sync::mpsc;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::modbus::spawn_modbus_read_task;
use crate::model::Measurement;
use crate::sqlite::spawn_sqlite_write_task;

pub async fn run(tty_path: String, db_path: String) {
    init_env_logging();

    tracing::info!("Starting temperature/humidity sensor service");

    let period = Duration::from_secs(5);

    let (tx, rx) = mpsc::channel::<Measurement>(32);

    tracing::info!("Spawning modbus read task");
    let _modbus_read_task = spawn_modbus_read_task(tty_path, period, tx);

    tracing::info!("Spawning measurement write task");
    let _sqlite_write_task = spawn_sqlite_write_task(rx, db_path);

    tracing::info!("Waiting for termination signal");
    let _ = catch_terminate_signal().recv().await;
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
