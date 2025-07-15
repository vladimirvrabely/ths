use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Measurement {
    pub at: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
}

#[derive(Debug, Clone)]
pub struct MeasurementData {
    pub timestamp: Vec<i64>,
    pub temperature: Vec<f64>,
    pub humidity: Vec<f64>,
}

impl MeasurementData {
    pub fn with_capacity(n: usize) -> Self {
        Self {
            timestamp: Vec::with_capacity(n),
            temperature: Vec::with_capacity(n),
            humidity: Vec::with_capacity(n),
        }
    }
}
