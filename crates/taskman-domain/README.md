# taskman-domain

Shared domain contracts for Taskman.

## What This Crate Owns

- Common DTOs:
  - `Snapshot`
  - `SystemMetrics`
  - `ProcessInfo`
  - `ActionRequest`
  - `ActionOutcome`
  - `CapabilityMatrix`
- Error model:
  - `TaskmanError`
  - `TaskmanResult<T>`
- Core traits:
  - `SnapshotProvider`
  - `ActionExecutor`
  - `EngineBackend`

## Why It Exists

This crate decouples business contracts from implementation details so that platform adapters, engine orchestration, and Python bindings can share a single stable model.

## Key Dependencies

- `serde` for serialization support
- `thiserror` for ergonomic typed errors

## Used By

- [taskman-engine](../taskman-engine/README.md)
- [taskman-platform-linux](../taskman-platform-linux/README.md)
- [taskman-platform-windows](../taskman-platform-windows/README.md)
- [taskman-bridge](../taskman-bridge/README.md)

## Related Docs

- [Crates index](../README.md)
