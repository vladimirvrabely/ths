use chrono::{TimeZone, Utc};
use sqlx::{Error, SqlitePool};

use crate::model::Measurement;

#[derive(Debug, Clone)]
pub struct DbPool(sqlx::SqlitePool);

impl DbPool {
    pub async fn new(path: &str) -> Result<Self, Error> {
        let pool = SqlitePool::connect(path).await?;
        tracing::info!("Connected to {}", path);
        Ok(Self(pool))
    }

    pub async fn get_latest_measurement(&mut self) -> Result<Measurement, Error> {
        let mut tx = self.0.begin().await?;

        let measurement = sqlx::query!(
            "SELECT at, temperature, humidity FROM measurement ORDER BY at DESC limit 1"
        )
        .fetch_one(&mut *tx)
        .await
        .map(|record| Measurement {
            at: Utc.timestamp_millis_opt(record.at).unwrap(),
            temperature: record.temperature,
            humidity: record.humidity,
        })?;

        tx.commit().await?;
        Ok(measurement)
    }
}
