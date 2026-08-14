# Aetheris

Aetheris is a Rust-based spacecraft telemetry anomaly detection platform. It includes a core detector, a replay CLI, a structured HTTP API, and a live telemetry dashboard that streams changing sensor readings in real time.

## What works today

- Core anomaly detection for battery temperature telemetry.
- Unit tests for the detector.
- Integration tests for the public API.
- Replay CLI that prints human-readable anomaly output.
- JSON replay artifact written to `output/replay.json`.
- HTTP API that returns structured detection responses.
- Live telemetry dashboard served directly by the API over SSE, cycling through readings that cover normal, high, and critical severity every 3 seconds.

## Repository layout

- `aetheris-core/` — core telemetry models, detector logic, replay logic, and replay binary.
- `aetheris-api/` — HTTP API built with `axum`. Also serves the dashboard as static files.
- `aetheris-web/` — browser dashboard for live telemetry visualization.
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

## Run the API and dashboard

```bash
cargo run -p aetheris-api
```

Then open your browser at:

```text
http://127.0.0.1:3000
```

The API serves the dashboard directly — no separate web server needed. The dashboard updates every 3 seconds, cycling through sensor readings that produce normal, high, and critical severity events. The sensor card, value card, severity card (with colour change), explanation card, and sparkline graph all update live.

## Test the project

```bash
cargo test
cargo check
```

The core crate includes five unit tests and two integration tests, all passing. `cargo check` produces no warnings.

## Current scope

Implemented:

- Detector logic.
- Replay CLI.
- JSON replay output.
- Structured API detection response.
- Live telemetry dashboard served by the API with cycling SSE stream.
- Test coverage for core and public API behavior.

Planned for later:

- AI-generated explanations.
- Predictive maintenance workflows.
- ML/ONNX inference integration.
- Multi-satellite fleet views.

## License

MIT License.
