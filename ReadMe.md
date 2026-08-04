# Aetheris

Aetheris is a Rust-based spacecraft telemetry anomaly detection platform with a working replay CLI, structured JSON output, and an HTTP API that returns shaped detection results.

## What works today

- Core anomaly detection for battery temperature telemetry.
- Unit tests for the detector.
- Integration tests for the public API.
- Replay CLI that prints human-readable anomaly output.
- JSON replay artifact written to `output/replay.json`.
- API endpoint that returns structured detection responses.

## Repository layout

- `aetheris-core/` — core telemetry models, detector, replay logic, and replay binary.
- `aetheris-api/` — HTTP API built with `axum`.
- `aetheris-core/tests/` — integration tests for the public detector API.
- `aetheris-core/src/bin/replay.rs` — replay command.
- `aetheris-core/src/replay.rs` — structured replay analysis result.
- `aetheris-core/src/anomaly.rs` — detector logic and unit tests.
- `aetheris-core/src/telemetry.rs` — telemetry and anomaly data models.

## Run the replay

```bash
cargo run -p aetheris-core --bin replay
```

This prints a readable replay summary and writes the structured JSON to `output/replay.json`.

## Run the API

```bash
cargo run -p aetheris-api
```

The API exposes a detection endpoint that accepts telemetry input and returns a structured JSON result.

## Test the project

```bash
cargo test
cargo check
```

The core crate currently includes five unit tests and two integration tests, all passing.

## Current scope

Implemented:

- Detector logic.
- Replay CLI.
- JSON replay output.
- Structured API detection response.
- Test coverage for core and public API behavior.

Planned for later:

- AI-generated explanations.
- Predictive maintenance workflows.
- ML/ONNX inference integration.
- Dashboard and visualization layer.
- Multi-satellite fleet views.

## License

MIT License.
