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
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{TimeZone, Utc};

    fn point(sensor_id: &str, value: f64) -> TelemetryPoint {
        TelemetryPoint {
            time: Utc.with_ymd_and_hms(2026, 8, 3, 14, 0, 0).unwrap(),
            satellite_id: "AETHERIS-01".to_string(),
            subsystem: "power".to_string(),
            sensor_id: sensor_id.to_string(),
            value,
            unit: "C".to_string(),
            quality_flag: 0,
        }
    }

    #[test]
    fn normal_battery_temperature_is_not_an_anomaly() {
        assert!(detect(&point("battery_temp_1", 69.9)).is_none());
    }

    #[test]
    fn unsupported_sensor_is_ignored() {
        assert!(detect(&point("unknown_sensor", 100.0)).is_none());
    }

    #[test]
    fn high_battery_temperature_creates_high_anomaly() {
        let anomaly = detect(&point("battery_temp_1", 70.0)).expect("anomaly expected");

        assert_eq!(anomaly.satellite_id, "AETHERIS-01");
        assert_eq!(anomaly.sensor_id, "battery_temp_1");
        assert!(matches!(anomaly.anomaly_type, AnomalyType::PointAnomaly));
        assert!(matches!(anomaly.severity, Severity::High));
        assert_eq!(anomaly.anomaly_score, 0.0);
    }

    #[test]
    fn extreme_battery_temperature_creates_critical_anomaly() {
        let anomaly = detect(&point("battery_temp_1", 80.0)).expect("anomaly expected");

        assert!(matches!(anomaly.severity, Severity::Critical));
        assert!((anomaly.anomaly_score - (10.0 / 70.0)).abs() < f64::EPSILON);
    }
}
