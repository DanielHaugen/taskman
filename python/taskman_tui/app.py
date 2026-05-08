from __future__ import annotations

from textual.app import App, ComposeResult
from textual.containers import Container
from textual.widgets import DataTable, Footer, Header, Static, TabbedContent, TabPane

from .client import TaskmanClient
from .types import ActionName, Snapshot


class TaskManagerApp(App[None]):
    TITLE = "Taskman"
    SUB_TITLE = "Rust core via PyO3"

    CSS = """
    Screen {
        background: #0f111a;
        color: #dfe7f3;
    }

    TabbedContent {
        height: 1fr;
    }

    DataTable {
        height: 1fr;
        border: round #2a3d5f;
    }

    #performance-view,
    #services-view,
    #startup-view,
    #details-view {
        padding: 1 2;
        border: round #2a3d5f;
        color: #cfd8e8;
    }

    .hint {
        color: #7f90ad;
    }
    """

    BINDINGS = [
        ("q", "quit", "Quit"),
        ("k", "kill_selected", "Kill"),
        ("s", "suspend_selected", "Suspend"),
        ("r", "resume_selected", "Resume"),
        ("p", "priority_selected", "Priority +5"),
        ("a", "affinity_selected", "Affinity CPU0"),
    ]

    def __init__(self) -> None:
        super().__init__()
        self._client = TaskmanClient()
        self._capabilities = self._client.get_capabilities()
        self._table_initialized = False
        self._visible_pids: list[int] = []

    def compose(self) -> ComposeResult:
        yield Header(show_clock=True)
        with TabbedContent(initial="processes"):
            with TabPane("Processes", id="processes"):
                yield DataTable(id="processes-table")
            with TabPane("Performance", id="performance"):
                with Container(id="performance-view"):
                    yield Static("Waiting for first snapshot...", id="performance-text")
            with TabPane("Services", id="services"):
                with Container(id="services-view"):
                    yield Static(
                        "Service support is adapter-backed and platform-aware."
                    )
            with TabPane("Startup", id="startup"):
                with Container(id="startup-view"):
                    yield Static(
                        "Startup management panel scaffolded for OS-specific adapters."
                    )
            with TabPane("Details", id="details"):
                with Container(id="details-view"):
                    yield Static("Detailed process metadata view scaffolded.")
        yield Static(
            "Keys: [k]ill [s]uspend [r]esume [p]riority [a]ffinity [q]uit",
            classes="hint",
        )
        yield Footer()

    def on_mount(self) -> None:
        self.set_interval(1.0, self._refresh_snapshot)
        self._refresh_snapshot()

    def _refresh_snapshot(self) -> None:
        snapshot = self._client.fetch_snapshot()
        self._render_processes(snapshot)
        self._render_performance(snapshot)
        self._render_tab_summaries(snapshot)

    def _render_processes(self, snapshot: Snapshot) -> None:
        table = self.query_one("#processes-table", DataTable)
        if not self._table_initialized:
            table.add_columns(
                "PID",
                "Name",
                "CPU %",
                "Memory MB",
                "Virtual MB",
                "Status",
            )
            self._table_initialized = True

        table.clear(columns=False)
        self._visible_pids.clear()

        for process in snapshot["processes"][:400]:
            self._visible_pids.append(process["pid"])
            table.add_row(
                str(process["pid"]),
                process["name"],
                f"{process['cpu_percent']:.1f}",
                f"{process['memory_bytes'] / (1024 * 1024):.1f}",
                f"{process['virtual_memory_bytes'] / (1024 * 1024):.1f}",
                process["status"],
            )

    def _render_performance(self, snapshot: Snapshot) -> None:
        system = snapshot["system"]
        memory_pct = 0.0
        if system["total_memory_bytes"] > 0:
            memory_pct = (
                100.0 * system["used_memory_bytes"] / system["total_memory_bytes"]
            )

        performance = self.query_one("#performance-text", Static)
        performance.update(
            "\n".join(
                [
                    f"CPU Usage: {system['cpu_usage_percent']:.1f}%",
                    f"Logical CPUs: {system['logical_cpu_count']}",
                    f"Memory: {system['used_memory_bytes'] / (1024**3):.2f} / "
                    f"{system['total_memory_bytes'] / (1024**3):.2f} GiB ({memory_pct:.1f}%)",
                    f"Swap: {system['used_swap_bytes'] / (1024**3):.2f} / "
                    f"{system['total_swap_bytes'] / (1024**3):.2f} GiB",
                    f"Uptime: {system['uptime_seconds']} seconds",
                ]
            )
        )

    def _render_tab_summaries(self, snapshot: Snapshot) -> None:
        process_count = len(snapshot["processes"])
        self.query_one("#services-view > Static", Static).update(
            f"Services tab scaffolded. Live process count: {process_count}."
        )
        self.query_one("#startup-view > Static", Static).update(
            f"Startup tab scaffolded. Active capability set: {self._capabilities}."
        )
        self.query_one("#details-view > Static", Static).update(
            f"Details tab scaffolded. Last refresh: {snapshot['timestamp_ms']}."
        )

    def _selected_pid(self) -> int | None:
        table = self.query_one("#processes-table", DataTable)
        row = table.cursor_row
        if row is None or row < 0 or row >= len(self._visible_pids):
            return None
        return self._visible_pids[row]

    def _run_selected_action(
        self, action: ActionName, *, priority: int | None = None
    ) -> None:
        pid = self._selected_pid()
        if pid is None:
            self.notify("No process selected", severity="warning")
            return

        kwargs: dict[str, object] = {"priority": priority}
        if action == "set_affinity":
            kwargs["affinity"] = [0]

        response = self._client.run_action(pid, action, **kwargs)
        severity = "information" if response["ok"] else "error"
        self.notify(response["message"], severity=severity)
        self._refresh_snapshot()

    def action_kill_selected(self) -> None:
        if not self._capabilities["can_kill"]:
            self.notify(
                "Kill action is unavailable on this platform", severity="warning"
            )
            return
        self._run_selected_action("kill")

    def action_suspend_selected(self) -> None:
        if not self._capabilities["can_suspend"]:
            self.notify(
                "Suspend action is unavailable on this platform", severity="warning"
            )
            return
        self._run_selected_action("suspend")

    def action_resume_selected(self) -> None:
        if not self._capabilities["can_resume"]:
            self.notify(
                "Resume action is unavailable on this platform", severity="warning"
            )
            return
        self._run_selected_action("resume")

    def action_priority_selected(self) -> None:
        if not self._capabilities["can_set_priority"]:
            self.notify(
                "Set priority action is unavailable on this platform",
                severity="warning",
            )
            return
        self._run_selected_action("set_priority", priority=5)

    def action_affinity_selected(self) -> None:
        if not self._capabilities["can_set_affinity"]:
            self.notify(
                "Set affinity action is unavailable on this platform",
                severity="warning",
            )
            return
        self._run_selected_action("set_affinity")
