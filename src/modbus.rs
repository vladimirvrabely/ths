use std::time::Duration;

use chrono::{DurationRound, TimeDelta, Utc};
use tokio::sync::mpsc;
use tokio::task::JoinHandle;
use tokio::time::Instant;
use tokio_modbus::{
    client::{Context, Reader, rtu::attach_slave},
    slave::Slave,
};
use tokio_stream::StreamExt;
use tokio_stream::wrappers::IntervalStream;

use crate::model::{AppError, Measurement};

pub struct SensorReader {
    client: Context,
}

impl SensorReader {
    pub fn new(tty_path: &str) -> Result<Self, tokio_serial::Error> {
        let baud_rate = 9600;
        let builder = tokio_serial::new(tty_path, baud_rate);
        let serial_stream = tokio_serial::SerialStream::open(&builder)?;
        let slave = Slave(1);
        let client = attach_slave(serial_stream, slave);

        Ok(Self { client })
    }

    pub async fn read(&mut self) -> Result<Measurement, AppError> {
        let ir = self.client.read_input_registers(1, 2).await??;
        let (temperature, humidity) = (ir[0], ir[1]);
        let at = Utc::now();
        let temperature = (temperature as i16) as f64 / 10.0;
        let humidity = humidity as f64 / 10.0;
        let measurement = Measurement {
            at,
            temperature,
            humidity,
        };
        Ok(measurement)
    }
}

pub fn spawn_modbus_read_task(
    tty_path: String,
    period: Duration,
    tx: mpsc::Sender<Measurement>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let mut sensor_reader =
            SensorReader::new(&tty_path).expect("Modbus reader should be created");

        let start = Instant::now() + duration_to_next(period);
        let interval = tokio::time::interval_at(start, period);
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

fn duration_to_next(period: Duration) -> Duration {
    let now = Utc::now();
    let period = TimeDelta::from_std(period).expect("should create TimeDelta");
    let next = now.duration_round_up(period).expect("should round up");
    (next - now).to_std().expect("should create Duration")
}

pub fn _spawn_simu_modbus_read_task(
    _tty_path: String,
    period: Duration,
    tx: mpsc::Sender<Measurement>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let start = Instant::now() + duration_to_next(period);
        let interval = tokio::time::interval_at(start, period);
        let mut stream = IntervalStream::new(interval);

        while let Some(_instant) = stream.next().await {
            let measurement = Measurement {
                at: Utc::now(),
                temperature: 25.0,
                humidity: 50.0,
            };
            let _ = tx.send(measurement).await;
        }
    })
}
