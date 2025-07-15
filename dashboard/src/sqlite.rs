use chrono::{TimeZone, Utc};
use sqlx::{Error, SqlitePool};

use crate::model::{Measurement, MeasurementData};

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

    pub async fn get_latest_week_measurement_data(&mut self) -> Result<MeasurementData, Error> {
        let mut tx = self.0.begin().await?;

        // "SELECT at, temperature, humidity FROM measurement ORDER BY at DESC limit 10"
        let records = sqlx::query!(
            r#"
            SELECT min(at) AS at, avg(temperature) AS temperature, avg(humidity) AS humidity
            FROM measurement 
            WHERE at > unixepoch('now', '-7 days') * 1000
            GROUP BY (at / 60000)
            ORDER BY at ASC;
        "#
        )
        .fetch_all(&mut *tx)
        .await?;
        tx.commit().await?;

        let mut measurement_data = MeasurementData::with_capacity(records.len());
        for record in records {
            measurement_data.timestamp.push(record.at / 1_000);
            measurement_data.temperature.push(record.temperature);
            measurement_data.humidity.push(record.humidity);
        }

        Ok(measurement_data)
    }
}
