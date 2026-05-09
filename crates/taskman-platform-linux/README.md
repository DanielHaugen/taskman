# taskman-platform-linux

Linux platform backend for Taskman.

## What This Crate Owns

- `LinuxBackend` implementation of:
  - `SnapshotProvider`
  - `ActionExecutor`
- Process and system snapshot collection.
- Linux action handlers:
  - kill (`SIGKILL`)
  - suspend (`SIGSTOP`)
  - resume (`SIGCONT`)
  - set priority (`setpriority`)
  - set CPU affinity (`sched_setaffinity`)

## Why It Exists

It encapsulates all Linux-specific behavior behind shared domain traits so engine and bridge layers remain platform-agnostic.

## Key Dependencies

- `sysinfo` for process and resource metrics
- `nix` for signal and affinity APIs
- `libc` for priority syscall integration
- [taskman-domain](../taskman-domain/README.md)

## Used By

- [taskman-engine](../taskman-engine/README.md)

## Related Docs

- [Crates index](../README.md)
