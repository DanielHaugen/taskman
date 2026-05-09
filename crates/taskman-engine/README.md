# taskman-engine

Engine orchestration for Taskman.

## What This Crate Owns

- `TaskmanEngine`, the main Rust orchestration type used by the PyO3 bridge.
- Backend dispatch for:
  - Linux via `taskman-platform-linux`
  - Windows via `taskman-platform-windows`
- Stable methods for:
  - `fetch_snapshot`
  - `run_action`
  - `capabilities`

## Why It Exists

This crate centralizes backend selection and isolates higher layers from platform-specific construction details.

## Backend Selection

- On Linux, default backend is `LinuxBackend`.
- On Windows, default backend is `WindowsBackend`.
- Other targets currently return an unsupported-backend error.

## Key Dependencies

- [taskman-domain](../taskman-domain/README.md)
- Target-specific platform crates

## Used By

- [taskman-bridge](../taskman-bridge/README.md)

## Related Docs

- [Crates index](../README.md)
