use aetheris_core::replay::analyze_point;
use aetheris_core::{Severity, TelemetryPoint};
use chrono::{TimeZone, Utc};

fn main() {
    let readings = [
        ("battery_temp_1", 65.0),
        ("battery_temp_1", 68.5),
        ("battery_temp_1", 72.0),
        ("battery_temp_1", 81.0),
    ];

    println!("Aetheris telemetry replay");
    println!("Satellite: AETHERIS-01");
    println!();

    for (index, (sensor_id, value)) in readings.iter().enumerate() {
        let point = TelemetryPoint {
            time: Utc
                .with_ymd_and_hms(2026, 8, 3, 14, index as u32, 0)
                .unwrap(),
            satellite_id: "AETHERIS-01".to_string(),
            subsystem: "power".to_string(),
            sensor_id: (*sensor_id).to_string(),
            value: *value,
            unit: "C".to_string(),
            quality_flag: 0,
        };

        let result = analyze_point(point);

        match result.severity {
            Some(Severity::High) => println!(
                "[ANOMALY] reading={:.1}{} severity=High score={:.3}",
                result.point.value,
                result.point.unit,
                result.anomaly_score.unwrap()
            ),
            Some(Severity::Critical) => println!(
                "[ANOMALY] reading={:.1}{} severity=Critical score={:.3}",
                result.point.value,
                result.point.unit,
                result.anomaly_score.unwrap()
            ),
            Some(_) => println!(
                "[ANOMALY] reading={:.1}{} score={:.3}",
                result.point.value,
                result.point.unit,
                result.anomaly_score.unwrap()
            ),
            None => println!(
                "[NORMAL]  reading={:.1}{}",
                result.point.value, result.point.unit
            ),
        }
    }
}