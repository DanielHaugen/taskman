from __future__ import annotations

from typing import Sequence

from .types import ActionResponse, Capabilities, Snapshot

class Engine:
    def fetch_snapshot(self) -> Snapshot: ...
    def get_capabilities(self) -> Capabilities: ...
    def run_action(
        self,
        pid: int,
        action: str,
        priority: int | None = None,
        affinity: Sequence[int] | None = None,
    ) -> ActionResponse: ...

def create_engine() -> Engine: ...
def __version__() -> str: ...
