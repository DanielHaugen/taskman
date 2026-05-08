use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub type TaskmanResult<T> = Result<T, TaskmanError>;

#[derive(Debug, Error)]
pub enum TaskmanError {
    #[error("unsupported action: {0}")]
    UnsupportedAction(String),
    #[error("process not found: pid={0}")]
    ProcessNotFound(i32),
    #[error("permission denied: {0}")]
    PermissionDenied(String),
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("platform error: {0}")]
    Platform(String),
    #[error("internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    pub timestamp_ms: u64,
    pub system: SystemMetrics,
    pub processes: Vec<ProcessInfo>,
}

impl Snapshot {
    pub fn now(system: SystemMetrics, processes: Vec<ProcessInfo>) -> Self {
        Self {
            timestamp_ms: now_timestamp_ms(),
            system,
            processes,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub cpu_usage_percent: f32,
    pub logical_cpu_count: usize,
    pub total_memory_bytes: u64,
    pub used_memory_bytes: u64,
    pub total_swap_bytes: u64,
    pub used_swap_bytes: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: i32,
    pub parent_pid: Option<i32>,
    pub name: String,
    pub status: String,
    pub cpu_percent: f32,
    pub memory_bytes: u64,
    pub virtual_memory_bytes: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionKind {
    Kill,
    Suspend,
    Resume,
    SetPriority,
    SetAffinity,
}

impl ActionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Kill => "kill",
            Self::Suspend => "suspend",
            Self::Resume => "resume",
            Self::SetPriority => "set_priority",
            Self::SetAffinity => "set_affinity",
        }
    }
}

impl fmt::Display for ActionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionRequest {
    pub pid: i32,
    pub action: ActionKind,
    pub priority: Option<i32>,
    pub affinity: Option<Vec<usize>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOutcome {
    pub ok: bool,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityMatrix {
    pub can_kill: bool,
    pub can_suspend: bool,
    pub can_resume: bool,
    pub can_set_priority: bool,
    pub can_set_affinity: bool,
}

impl CapabilityMatrix {
    pub const fn read_only() -> Self {
        Self {
            can_kill: false,
            can_suspend: false,
            can_resume: false,
            can_set_priority: false,
            can_set_affinity: false,
        }
    }
}

pub trait SnapshotProvider: Send + Sync {
    fn fetch_snapshot(&self) -> TaskmanResult<Snapshot>;
}

pub trait ActionExecutor: Send + Sync {
    fn run_action(&self, request: ActionRequest) -> TaskmanResult<ActionOutcome>;
    fn capabilities(&self) -> CapabilityMatrix;
}

pub trait EngineBackend: SnapshotProvider + ActionExecutor {}

impl<T> EngineBackend for T where T: SnapshotProvider + ActionExecutor {}

pub fn now_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(0)
}
