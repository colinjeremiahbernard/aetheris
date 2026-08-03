use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AnomalyRecord {
    pub id: Uuid,
    pub time: DateTime<Utc>,
    pub satellite_id: String,
    pub sensor_id: String,
    pub anomaly_score: f64,
    pub anomaly_type: String,
    pub severity: String,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct AnomalyQueryParams {
    pub satellite_id: Option<String>,
    pub severity: Option<String>,
    pub limit: Option<i32>,
}
