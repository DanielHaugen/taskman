# taskman-bridge

PyO3 bridge crate for Taskman.

## What This Crate Owns

- Python extension module `taskman_native`.
- PyO3 class/API surface:
  - `Engine`
  - `create_engine()`
  - `__version__()`
- Rust-to-Python data conversion:
  - snapshot payloads
  - capabilities payload
  - action outcome payload
- Rust error to Python exception mapping.

## Why It Exists

This crate is the only translation boundary between Rust core logic and the Python Textual facade. It keeps Python code thin and business logic in Rust.

## Key Dependencies

- `pyo3`
- [taskman-domain](../taskman-domain/README.md)
- [taskman-engine](../taskman-engine/README.md)

## Artifact

`Cargo.toml` configures both:

- `cdylib` for Python extension loading
- `rlib` for standard Rust library usage

## Related Docs

- [Crates index](../README.md)
