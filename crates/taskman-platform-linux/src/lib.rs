use std::cmp::Ordering;
use std::sync::Mutex;

use nix::sched::{CpuSet, sched_setaffinity};
use nix::sys::signal::{Signal, kill};
use nix::unistd::Pid;
use sysinfo::{ProcessesToUpdate, System};
use taskman_domain::{
    ActionExecutor, ActionKind, ActionOutcome, ActionRequest, CapabilityMatrix, ProcessInfo,
    Snapshot, SnapshotProvider, SystemMetrics, TaskmanError, TaskmanResult,
};

pub struct LinuxBackend {
    system: Mutex<System>,
}

impl LinuxBackend {
    pub fn new() -> TaskmanResult<Self> {
        let mut system = System::new_all();
        system.refresh_all();
        Ok(Self {
            system: Mutex::new(system),
        })
    }

    fn with_system<T>(
        &self,
        callback: impl FnOnce(&mut System) -> TaskmanResult<T>,
    ) -> TaskmanResult<T> {
        let mut system = self
            .system
            .lock()
            .map_err(|_| TaskmanError::Internal("failed to lock system state".to_owned()))?;
        callback(&mut system)
    }

    fn run_unix_signal(pid: i32, signal: Signal) -> TaskmanResult<ActionOutcome> {
        kill(Pid::from_raw(pid), signal)
            .map_err(|error| TaskmanError::Platform(error.to_string()))?;
        Ok(ActionOutcome {
            ok: true,
            message: format!("sent {signal:?} to pid={pid}"),
        })
    }

    fn set_priority(pid: i32, priority: i32) -> TaskmanResult<ActionOutcome> {
        // SAFETY: setpriority is called with validated scalar values and has no Rust aliasing impact.
        let code = unsafe { libc::setpriority(libc::PRIO_PROCESS, pid as u32, priority) };
        if code == 0 {
            Ok(ActionOutcome {
                ok: true,
                message: format!("set priority={priority} on pid={pid}"),
            })
        } else {
            Err(TaskmanError::Platform(
                std::io::Error::last_os_error().to_string(),
            ))
        }
    }

    fn set_affinity(pid: i32, cpus: &[usize]) -> TaskmanResult<ActionOutcome> {
        if cpus.is_empty() {
            return Err(TaskmanError::InvalidInput(
                "affinity must include at least one CPU index".to_owned(),
            ));
        }

        let mut cpu_set = CpuSet::new();
        for cpu in cpus {
            cpu_set
                .set(*cpu)
                .map_err(|error| TaskmanError::InvalidInput(error.to_string()))?;
        }

        sched_setaffinity(Pid::from_raw(pid), &cpu_set)
            .map_err(|error| TaskmanError::Platform(error.to_string()))?;
        Ok(ActionOutcome {
            ok: true,
            message: format!("set affinity={cpus:?} on pid={pid}"),
        })
    }
}

impl SnapshotProvider for LinuxBackend {
    fn fetch_snapshot(&self) -> TaskmanResult<Snapshot> {
        self.with_system(|system| {
            system.refresh_cpu_usage();
            system.refresh_memory();
            system.refresh_processes(ProcessesToUpdate::All, true);

            let mut processes = system
                .processes()
                .iter()
                .map(|(pid, process)| ProcessInfo {
                    pid: pid.as_u32() as i32,
                    parent_pid: process.parent().map(|parent| parent.as_u32() as i32),
                    name: process.name().to_string_lossy().to_string(),
                    status: format!("{:?}", process.status()),
                    cpu_percent: process.cpu_usage(),
                    memory_bytes: process.memory(),
                    virtual_memory_bytes: process.virtual_memory(),
                })
                .collect::<Vec<_>>();

            processes.sort_by(|left, right| {
                right
                    .cpu_percent
                    .partial_cmp(&left.cpu_percent)
                    .unwrap_or(Ordering::Equal)
            });

            let system_metrics = SystemMetrics {
                cpu_usage_percent: system.global_cpu_usage(),
                logical_cpu_count: system.cpus().len(),
                total_memory_bytes: system.total_memory(),
                used_memory_bytes: system.used_memory(),
                total_swap_bytes: system.total_swap(),
                used_swap_bytes: system.used_swap(),
                uptime_seconds: System::uptime(),
            };

            Ok(Snapshot::now(system_metrics, processes))
        })
    }
}

impl ActionExecutor for LinuxBackend {
    fn run_action(&self, request: ActionRequest) -> TaskmanResult<ActionOutcome> {
        match request.action {
            ActionKind::Kill => Self::run_unix_signal(request.pid, Signal::SIGKILL),
            ActionKind::Suspend => Self::run_unix_signal(request.pid, Signal::SIGSTOP),
            ActionKind::Resume => Self::run_unix_signal(request.pid, Signal::SIGCONT),
            ActionKind::SetPriority => {
                let priority = request.priority.ok_or_else(|| {
                    TaskmanError::InvalidInput("set_priority requires `priority`".to_owned())
                })?;
                Self::set_priority(request.pid, priority)
            }
            ActionKind::SetAffinity => {
                let affinity = request.affinity.ok_or_else(|| {
                    TaskmanError::InvalidInput("set_affinity requires `affinity`".to_owned())
                })?;
                Self::set_affinity(request.pid, &affinity)
            }
        }
    }

    fn capabilities(&self) -> CapabilityMatrix {
        CapabilityMatrix {
            can_kill: true,
            can_suspend: true,
            can_resume: true,
            can_set_priority: true,
            can_set_affinity: true,
        }
    }
}
