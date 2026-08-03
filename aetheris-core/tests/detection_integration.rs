use aetheris_core::{detect, Severity, TelemetryPoint};
use chrono::{TimeZone, Utc};

fn telemetry(sensor_id: &str, value: f64) -> TelemetryPoint {
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
fn public_api_detects_critical_battery_temperature() {
    let anomaly = detect(&telemetry("battery_temp_1", 81.0)).expect("critical anomaly expected");

    assert!(matches!(anomaly.severity, Severity::Critical));
    assert_eq!(anomaly.satellite_id, "AETHERIS-01");
    assert_eq!(anomaly.sensor_id, "battery_temp_1");
}

#[test]
fn public_api_ignores_normal_temperature() {
    assert!(detect(&telemetry("battery_temp_1", 65.0)).is_none());
}
