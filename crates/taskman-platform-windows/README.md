# taskman-platform-windows

Windows platform backend scaffold for Taskman.

## What This Crate Owns

- `WindowsBackend` type implementing domain traits.
- Placeholder behavior for:
  - snapshot retrieval
  - control action execution
  - capability reporting

## Current Status

This crate is intentionally scaffolded and currently returns unsupported-action responses. It exists to lock architecture boundaries and define the extension point for a native Windows implementation.

## Planned Scope

- Process/system snapshot collection from Windows APIs.
- Action execution (process control, priority, affinity) with permission-aware errors.
- Capability matrix based on runtime support and privileges.

## Key Dependencies

- [taskman-domain](../taskman-domain/README.md)

## Used By

- [taskman-engine](../taskman-engine/README.md)

## Related Docs

- [Crates index](../README.md)
