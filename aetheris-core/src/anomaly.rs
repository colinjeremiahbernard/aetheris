use crate::telemetry::{Anomaly, AnomalyType, Severity, TelemetryPoint};
use chrono::Utc;
use uuid::Uuid;

pub fn detect(point: &TelemetryPoint) -> Option<Anomaly> {
    let upper_threshold = match point.sensor_id.as_str() {
        "battery_temp_1" => 70.0,
        _ => return None,
    };

    if point.value < upper_threshold {
        return None;
    }

    let severity = if point.value >= 80.0 {
        Severity::Critical
    } else {
        Severity::High
    };

    let anomaly_score = ((point.value - upper_threshold) / upper_threshold).clamp(0.0, 1.0);

    Some(Anomaly {
        id: Uuid::new_v4(),
        time: point.time,
        satellite_id: point.satellite_id.clone(),
        sensor_id: point.sensor_id.clone(),
        anomaly_score,
        anomaly_type: AnomalyType::PointAnomaly,
        severity,
        created_at: Utc::now(),
    })
}
