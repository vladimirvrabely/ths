use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error(transparent)]
    ModbusError(#[from] tokio_modbus::Error),
    #[error(transparent)]
    ModbusExceptionCode(#[from] tokio_modbus::ExceptionCode),
}

#[derive(Debug, Clone)]
pub struct Measurement {
    pub at: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
}
