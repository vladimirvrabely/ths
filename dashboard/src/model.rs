use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Measurement {
    pub at: DateTime<Utc>,
    pub temperature: f64,
    pub humidity: f64,
}
