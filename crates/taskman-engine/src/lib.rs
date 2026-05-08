use taskman_domain::{ActionRequest, CapabilityMatrix, EngineBackend, Snapshot, TaskmanResult};

pub struct TaskmanEngine {
    backend: Box<dyn EngineBackend>,
}

impl TaskmanEngine {
    pub fn new(backend: impl EngineBackend + 'static) -> Self {
        Self {
            backend: Box::new(backend),
        }
    }

    pub fn new_default() -> TaskmanResult<Self> {
        let backend = default_backend()?;
        Ok(Self::new(backend))
    }

    pub fn fetch_snapshot(&self) -> TaskmanResult<Snapshot> {
        self.backend.fetch_snapshot()
    }

    pub fn run_action(
        &self,
        request: ActionRequest,
    ) -> TaskmanResult<taskman_domain::ActionOutcome> {
        self.backend.run_action(request)
    }

    pub fn capabilities(&self) -> CapabilityMatrix {
        self.backend.capabilities()
    }
}

#[cfg(target_os = "linux")]
fn default_backend() -> TaskmanResult<taskman_platform_linux::LinuxBackend> {
    taskman_platform_linux::LinuxBackend::new()
}

#[cfg(target_os = "windows")]
fn default_backend() -> TaskmanResult<taskman_platform_windows::WindowsBackend> {
    taskman_platform_windows::WindowsBackend::new()
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn default_backend() -> TaskmanResult<UnsupportedBackend> {
    Err(TaskmanError::UnsupportedAction(
        "default backend only supports Linux and Windows".to_owned(),
    ))
}

#[cfg(not(any(target_os = "linux", target_os = "windows")))]
struct UnsupportedBackend;
