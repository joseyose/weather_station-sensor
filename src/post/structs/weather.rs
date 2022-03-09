use serde::{Deserialize, Serialize};
use chrono::{DateTime, Local};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub temperature: f64,
    pub humidity: f64,
    pub pressure: f64,
    pub date: DateTime<Local>,
}