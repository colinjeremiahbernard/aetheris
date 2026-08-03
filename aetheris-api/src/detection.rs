use aetheris_core::TelemetryPoint;
use axum::http::StatusCode;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TelemetryInput {
    pub timestamp: String,
    pub satellite_id: String,
    pub subsystem: String,
    pub sensor_id: String,
    pub value: f64,
    pub unit: String,
    pub quality_flag: i16,
}

impl TelemetryInput {
    pub fn into_point(self) -> Result<TelemetryPoint, StatusCode> {
        let time = chrono::DateTime::parse_from_rfc3339(&self.timestamp)
            .map_err(|_| StatusCode::BAD_REQUEST)?
            .with_timezone(&chrono::Utc);

        Ok(TelemetryPoint {
            time,
            satellite_id: self.satellite_id,
            subsystem: self.subsystem,
            sensor_id: self.sensor_id,
            value: self.value,
            unit: self.unit,
            quality_flag: self.quality_flag,
        })
    }
}
