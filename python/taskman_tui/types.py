from __future__ import annotations

from typing import Literal, TypedDict

ActionName = Literal["kill", "suspend", "resume", "set_priority", "set_affinity"]


class ProcessRow(TypedDict):
    pid: int
    parent_pid: int | None
    name: str
    status: str
    cpu_percent: float
    memory_bytes: int
    virtual_memory_bytes: int


class SystemMetrics(TypedDict):
    cpu_usage_percent: float
    logical_cpu_count: int
    total_memory_bytes: int
    used_memory_bytes: int
    total_swap_bytes: int
    used_swap_bytes: int
    uptime_seconds: int


class Snapshot(TypedDict):
    timestamp_ms: int
    system: SystemMetrics
    processes: list[ProcessRow]


class Capabilities(TypedDict):
    can_kill: bool
    can_suspend: bool
    can_resume: bool
    can_set_priority: bool
    can_set_affinity: bool


class ActionResponse(TypedDict):
    ok: bool
    message: str
