# taskman

Taskman is a Windows 11 Task Manager-inspired Python TUI where Rust owns collection, control actions, and business logic.

## Highlights

- Rust core with trait-based architecture for extensibility.
- PyO3 bridge exposing a compact, typed Python API.
- Thin Python Textual facade for layout and interaction.
- Linux implementation started; Windows adapter scaffolded.

## Project Layout

- `crates/taskman-domain`: DTOs, errors, traits, action contracts.
- `crates/taskman-engine`: orchestration layer and default backend selection.
- `crates/taskman-platform-linux`: Linux process and system metrics provider.
- `crates/taskman-platform-windows`: Windows adapter scaffold.
- `crates/taskman-bridge`: PyO3 extension module.
- `python/taskman_tui`: thin Textual application.

## Quick Start

```bash
cd workspace/taskman
python -m venv .venv
source .venv/bin/activate
pip install -U pip maturin
maturin develop  # builds and installs taskman_tui.taskman_native into .venv
python -m taskman_tui
```

## How the Build System Works

Taskman is a mixed Rust + Python project. Rust builds the native extension, and Python hosts the TUI.

1. `pyproject.toml` defines `maturin` as the build backend.
1. `maturin` compiles the Rust bridge crate at `crates/taskman-bridge`.
1. The bridge is packaged as `taskman_tui.taskman_native`.
1. Python sources under `python/` are installed alongside the native module.
1. The command `python -m taskman_tui` starts the Textual application, which calls into Rust through PyO3.

### Why `maturin develop` is the main command

- It compiles Rust changes and installs the updated extension into your active virtual environment.
- It installs Python dependencies declared in `pyproject.toml`.
- It sets up an editable install so Python source edits are picked up immediately.

### Typical dev loop

1. Edit Rust in `crates/*` or Python in `python/taskman_tui/*`.
1. Run `maturin develop` after Rust changes.
1. Run `python -m taskman_tui` to launch the app.

## Architecture Overview

Taskman uses a layered design where dependencies flow downward from UI to platform adapters.

1. **Python TUI Facade** (`python/taskman_tui`)
   Handles layout, keybindings, rendering, and user interaction in Textual.
1. **PyO3 Bridge** (`crates/taskman-bridge`)
   Exposes a compact Python API (`create_engine`, `fetch_snapshot`, `run_action`, `get_capabilities`).
1. **Engine Orchestrator** (`crates/taskman-engine`)
   Selects the platform backend and coordinates snapshot/action requests.
1. **Domain Contracts** (`crates/taskman-domain`)
   Shared DTOs, traits, capabilities, and error model.
1. **Platform Adapters** (`crates/taskman-platform-linux`, `crates/taskman-platform-windows`)
   OS-specific process and system integration behind shared traits.

### Runtime flow

1. Textual event triggers a request in `TaskmanClient`.
1. Request crosses the PyO3 boundary into `taskman_native`.
1. `TaskmanEngine` routes work to the selected backend.
1. Backend gathers metrics or runs control actions.
1. Results are converted to Python-friendly payloads and rendered by the TUI.

### Extending the project

- Add a new OS backend by implementing domain traits in a new crate.
- Add new actions by extending `ActionKind` and backend action handlers.
- Add new views in the TUI without moving business logic out of Rust.
