# Win-EDR

Experimental Windows EDR agent proof of concept (PoC). Goal: run as a Windows Service, emit basic telemetry, and pave the way toward a lightweight, Rust-based endpoint sensor.

## What it does today
- Windows Service entry point wired to the SCM via `StartServiceCtrlDispatcherW` and `RegisterServiceCtrlHandlerW`.
- Worker loop with graceful shutdown handling and configurable heartbeat intervals.
- Structured logging with `tracing` crate and thread metadata.
- ETW (Event Tracing for Windows) event collection with baseline and process event capture.
- Process context enrichment including parent-child relationships, command-line arguments, and full path normalization.
- Process snapshot and caching for efficient state tracking and memory usage optimization.
- Event model for process and image load telemetry with path normalization utilities and unit test coverage.

## Project layout

### Core service
- [edr-agent/src/main.rs](edr-agent/src/main.rs): service entry point and service table initialization.
- [edr-agent/src/service.rs](edr-agent/src/service.rs): service control handler registration, lifecycle management, and worker coordination.
- [edr-agent/src/worker.rs](edr-agent/src/worker.rs): main event loop that orchestrates telemetry collection and handles shutdown signals.
- [edr-agent/src/logging.rs](edr-agent/src/logging.rs): `tracing` subscriber configuration with thread ID and name tracking.
- [edr-agent/src/model.rs](edr-agent/src/model.rs): core event structures, enums, and path normalization utilities.

### ETW (Event Tracing for Windows)
- [edr-agent/src/etw/mod.rs](edr-agent/src/etw/mod.rs): ETW module initialization and coordination.
- [edr-agent/src/etw/baseline.rs](edr-agent/src/etw/baseline.rs): baseline ETW event capture and filtering.
- [edr-agent/src/etw/process.rs](edr-agent/src/etw/process.rs): ETW process creation and termination event handlers.

### Process telemetry
- [edr-agent/src/process/mod.rs](edr-agent/src/process/mod.rs): process module coordination.
- [edr-agent/src/process/enrich.rs](edr-agent/src/process/enrich.rs): process context enrichment (parent relationships, command-line, environment).
- [edr-agent/src/process/cache.rs](edr-agent/src/process/cache.rs): in-memory process context cache for efficient state management.
- [edr-agent/src/process/snapshot.rs](edr-agent/src/process/snapshot.rs): process snapshot collection and serialization.

## Build and run
- Prereqs: Rust toolchain (stable) on Windows.
- Build debug: `cd edr-agent && cargo build`.
- Run as console (debugging): `cargo run`. It will log heartbeats until Ctrl+C.
- Service install: not automated yet. Use `sc create` or a service manager to point to the built binary, then `sc start EDRAgent`.

## Current limitations
- ETW event collection is process-focused; other event types (network, file, image load) are not yet captured.
- No syscall/kernel event hooking via minifilter or system call tracing.
- Service install/uninstall automation is missing (manual `sc create` required).
- No persistence layer for telemetry data (in-memory only).
- No networking or backend integration; all logs remain local.
- Configuration is hardcoded; no config file or registry-based settings yet.
- Minimal error recovery; some failures may cause service termination.

## Roadmap

### Completed
- Service Windows Rust (core service framework)
- Process context & enrichment (parent tracking, command-line capture)
- Minimal ETW collection (process creation/termination events)
- Local state management (in-memory cache and snapshots)

### In progress
- Extended ETW coverage (network, file, image load events)
- Syscall/kernel event capture
- Service install/uninstall automation

### Upcoming 
- Persistent storage (local database or file-based)
- Backend networking and cloud integration
- Detection pipeline and rule engine
- Basic behavioral detections (suspicious patterns)
- EDR response actions (process termination, quarantine)
- Self-health monitoring and observability
- Progressive hardening and privilege management

## Contributing
This is early-stage research code. Please open issues or PRs with small, reviewable changes.
