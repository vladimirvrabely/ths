use std::fs::File;
use std::io::Write;
use std::str::FromStr;

use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::ReceiverStream;

use crate::model::{AppError, Measurement};

#[derive(Debug, Clone)]
pub struct DbPool(sqlx::SqlitePool);

impl DbPool {
    pub async fn create_database(path: &str) -> Result<(), AppError> {
        let opts = SqliteConnectOptions::from_str(path)?.create_if_missing(true);
        let pool = SqlitePool::connect_with(opts).await?;

        let mut tx = pool.begin().await?;

        let _res = sqlx::query!(
            r#"
            CREATE TABLE IF NOT EXISTS measurement (
                -- UNIX milliseconds
                at INTEGER PRIMARY KEY,
                -- Celsius degrees
                temperature REAL NOT NULL,
                -- percentage
                humidity REAL NOT NULL
            );
            "#
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        Ok(())
    }

    pub async fn new(path: &str) -> Result<Self, AppError> {
        let pool = SqlitePool::connect(path).await?;
        tracing::info!("Connected to {}", path);
        Ok(Self(pool))
    }

    pub async fn write_measurement(&mut self, measurement: Measurement) -> Result<(), AppError> {
        let mut tx = self.0.begin().await?;

        let at = measurement.at.timestamp_millis();
        let _res = sqlx::query!(
            "INSERT INTO measurement VALUES (?, ?, ?)",
            at,
            measurement.temperature,
            measurement.humidity
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

pub fn spawn_sqlite_write_task(
    rx: mpsc::Receiver<Measurement>,
    db_path: String,
    csv_path: String,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let stream = ReceiverStream::new(rx);

        DbPool::create_database(&db_path)
            .await
            .expect("should create SQLite database");

        let pool = DbPool::new(&db_path)
            .await
            .expect("should connect to SQLite database");

        tokio::pin!(stream);
        while let Some(measurement) = stream.next().await {
            tracing::debug!("Writing {:?}", &measurement);

            write_measurement_to_file(&csv_path, measurement.clone());

            match pool.clone().write_measurement(measurement).await {
                Ok(_) => {}
                Err(err) => {
                    tracing::error!("{}", err);
                }
            };
        }
    })
}

fn write_measurement_to_file(file: &str, measurement: Measurement) {
    let mut file = File::options()
        .create(true)
        .append(true)
        .open(file)
        .expect("should opened the measurement file");
    writeln!(
        file,
        "{},{},{}",
        measurement.at, measurement.temperature, measurement.humidity
    )
    .expect("should wrote to measurement file");
}
