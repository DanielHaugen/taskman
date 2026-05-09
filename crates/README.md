# Taskman Crates

This directory contains the Rust workspace crates that power Taskman.

## Crate Index

| Crate | Purpose | Platform | Crate Type | Current Status |
|---|---|---|---|---|
| [taskman-domain](taskman-domain/README.md) | Shared domain model, error types, and core traits (`SnapshotProvider`, `ActionExecutor`) | Cross-platform | Library | Active |
| [taskman-engine](taskman-engine/README.md) | Orchestrator that selects the default platform backend and exposes a stable engine API | Cross-platform | Library | Active |
| [taskman-platform-linux](taskman-platform-linux/README.md) | Linux backend for process snapshots and control actions | Linux | Library | Active |
| [taskman-platform-windows](taskman-platform-windows/README.md) | Windows backend scaffold using domain contracts | Windows | Library | Scaffold |
| [taskman-bridge](taskman-bridge/README.md) | PyO3 bridge crate exposing `taskman_native` Python extension APIs | Cross-platform | `cdylib` + `rlib` | Active |

## Dependency Flow

The intended dependency direction is:

1. `taskman-domain`
1. `taskman-platform-*`
1. `taskman-engine`
1. `taskman-bridge`

This keeps domain contracts independent and allows platform adapters to evolve without changing Python-facing API code.

## Design Notes

- `taskman-domain` has no PyO3 dependency.
- Platform-specific logic stays in dedicated crates.
- `taskman-bridge` is the only crate that translates Rust types/errors into Python objects/exceptions.
