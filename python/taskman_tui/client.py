from __future__ import annotations

from typing import Sequence

from . import taskman_native

from .types import ActionName, ActionResponse, Capabilities, Snapshot


class TaskmanClient:
    """Thin adapter around the PyO3 extension module."""

    def __init__(self) -> None:
        self._engine = taskman_native.create_engine()

    def fetch_snapshot(self) -> Snapshot:
        return self._engine.fetch_snapshot()

    def get_capabilities(self) -> Capabilities:
        return self._engine.get_capabilities()

    def run_action(
        self,
        pid: int,
        action: ActionName,
        *,
        priority: int | None = None,
        affinity: Sequence[int] | None = None,
    ) -> ActionResponse:
        return self._engine.run_action(pid, action, priority, affinity)
