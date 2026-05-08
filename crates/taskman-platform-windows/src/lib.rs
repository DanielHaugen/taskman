use taskman_domain::{
    ActionExecutor, ActionOutcome, ActionRequest, CapabilityMatrix, Snapshot, SnapshotProvider,
    TaskmanError, TaskmanResult,
};

pub struct WindowsBackend;

impl WindowsBackend {
    pub fn new() -> TaskmanResult<Self> {
        Ok(Self)
    }
}

impl SnapshotProvider for WindowsBackend {
    fn fetch_snapshot(&self) -> TaskmanResult<Snapshot> {
        Err(TaskmanError::UnsupportedAction(
            "windows backend is scaffolded but not implemented yet".to_owned(),
        ))
    }
}

impl ActionExecutor for WindowsBackend {
    fn run_action(&self, _request: ActionRequest) -> TaskmanResult<ActionOutcome> {
        Err(TaskmanError::UnsupportedAction(
            "windows actions are scaffolded but not implemented yet".to_owned(),
        ))
    }

    fn capabilities(&self) -> CapabilityMatrix {
        CapabilityMatrix::read_only()
    }
}
