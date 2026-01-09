# Win-EDR

Experimental Windows EDR agent proof of concept (PoC). Goal: run as a Windows Service, emit basic telemetry, and pave the way toward a lightweight, Rust-based endpoint sensor.

## What it does today
- Windows Service entry point wired to the SCM via `StartServiceCtrlDispatcherW` and `RegisterServiceCtrlHandlerW`.
- Simple worker loop with a stop signal, emitting heartbeat logs every 5 seconds.
- Structured logging with `tracing` and thread metadata.
- Event model sketches for process and image load telemetry, including path normalization helpers and unit test coverage.

## Project layout
- [edr-agent/src/main.rs](edr-agent/src/main.rs): boots the service table and hands off to the service runner.
- [edr-agent/src/service.rs](edr-agent/src/service.rs): registers control handler, tracks lifecycle state, and hosts the worker thread.
- [edr-agent/src/worker.rs](edr-agent/src/worker.rs): heartbeat loop that listens for stop/shutdown requests.
- [edr-agent/src/logging.rs](edr-agent/src/logging.rs): sets up `tracing` subscriber with thread ids and names.
- [edr-agent/src/model.rs](edr-agent/src/model.rs): event structs, enums, and path normalization utilities.

## Build and run
- Prereqs: Rust toolchain (stable) on Windows.
- Build debug: `cd edr-agent && cargo build`.
- Run as console (debugging): `cargo run`. It will log heartbeats until Ctrl+C.
- Service install: not automated yet. Use `sc create` or a service manager to point to the built binary, then `sc start EDRAgent`.

## Current limitations
- Telemetry capture is stubbed (only heartbeats, no ETW or syscall data yet).
- Service install/uninstall scripts are missing.
- No persistence, networking, or configuration layer.
- Limited error handling and diagnostics; logs are local only.

## Roadmap (prochaines étapes)
- Service Windows Rust
- Process context & enrichment
- Collecte ETW minimale
- State management local
- Module réseau / backend
- Pipeline détection
- Détections comportementales basiques
- Actions EDR
- Observabilité & self-health
- Hardening progressif

## Contributing
This is early-stage research code. Please open issues or PRs with small, reviewable changes.
