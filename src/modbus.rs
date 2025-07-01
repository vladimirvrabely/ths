use chrono::Utc;
use tokio_modbus::{
    client::{Context, Reader, rtu::attach_slave},
    slave::Slave,
};

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
