use std::sync::Mutex;

use pyo3::create_exception;
use pyo3::exceptions::{PyPermissionError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use taskman_domain::{ActionKind, ActionRequest, CapabilityMatrix, Snapshot, TaskmanError};
use taskman_engine::TaskmanEngine;

create_exception!(
    taskman_native,
    TaskmanBridgeError,
    pyo3::exceptions::PyException
);

#[pyclass(name = "Engine")]
pub struct PyTaskmanEngine {
    inner: Mutex<TaskmanEngine>,
}

#[pymethods]
impl PyTaskmanEngine {
    #[new]
    fn new() -> PyResult<Self> {
        let inner = TaskmanEngine::new_default().map_err(map_taskman_error)?;
        Ok(Self {
            inner: Mutex::new(inner),
        })
    }

    fn fetch_snapshot<'py>(&self, py: Python<'py>) -> PyResult<Py<PyDict>> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("engine lock poisoned"))?;
        let snapshot = guard.fetch_snapshot().map_err(map_taskman_error)?;
        snapshot_to_python(py, snapshot)
    }

    fn get_capabilities<'py>(&self, py: Python<'py>) -> PyResult<Py<PyDict>> {
        let guard = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("engine lock poisoned"))?;
        capabilities_to_python(py, guard.capabilities())
    }

    #[pyo3(signature = (pid, action, priority=None, affinity=None))]
    fn run_action<'py>(
        &self,
        py: Python<'py>,
        pid: i32,
        action: &str,
        priority: Option<i32>,
        affinity: Option<Vec<usize>>,
    ) -> PyResult<Py<PyDict>> {
        let action = parse_action(action)?;
        let request = ActionRequest {
            pid,
            action,
            priority,
            affinity,
        };

        let guard = self
            .inner
            .lock()
            .map_err(|_| PyRuntimeError::new_err("engine lock poisoned"))?;
        let outcome = guard.run_action(request).map_err(map_taskman_error)?;

        let response = PyDict::new(py);
        response.set_item("ok", outcome.ok)?;
        response.set_item("message", outcome.message)?;
        Ok(response.unbind())
    }
}

#[pyfunction]
fn create_engine() -> PyResult<PyTaskmanEngine> {
    PyTaskmanEngine::new()
}

#[pyfunction]
fn __version__() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[pymodule]
fn taskman_native(_py: Python<'_>, module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyTaskmanEngine>()?;
    module.add_function(wrap_pyfunction!(create_engine, module)?)?;
    module.add_function(wrap_pyfunction!(__version__, module)?)?;
    module.add(
        "TaskmanBridgeError",
        module.py().get_type::<TaskmanBridgeError>(),
    )?;
    Ok(())
}

fn parse_action(action: &str) -> PyResult<ActionKind> {
    match action {
        "kill" => Ok(ActionKind::Kill),
        "suspend" => Ok(ActionKind::Suspend),
        "resume" => Ok(ActionKind::Resume),
        "set_priority" => Ok(ActionKind::SetPriority),
        "set_affinity" => Ok(ActionKind::SetAffinity),
        _ => Err(PyValueError::new_err(format!(
            "unknown action `{action}`; expected kill|suspend|resume|set_priority|set_affinity"
        ))),
    }
}

fn map_taskman_error(error: TaskmanError) -> PyErr {
    match error {
        TaskmanError::PermissionDenied(message) => PyPermissionError::new_err(message),
        TaskmanError::InvalidInput(message) => PyValueError::new_err(message),
        TaskmanError::UnsupportedAction(message) => TaskmanBridgeError::new_err(message),
        TaskmanError::ProcessNotFound(pid) => {
            TaskmanBridgeError::new_err(format!("process not found: pid={pid}"))
        }
        TaskmanError::Platform(message) | TaskmanError::Internal(message) => {
            PyRuntimeError::new_err(message)
        }
    }
}

fn capabilities_to_python<'py>(
    py: Python<'py>,
    capabilities: CapabilityMatrix,
) -> PyResult<Py<PyDict>> {
    let payload = PyDict::new(py);
    payload.set_item("can_kill", capabilities.can_kill)?;
    payload.set_item("can_suspend", capabilities.can_suspend)?;
    payload.set_item("can_resume", capabilities.can_resume)?;
    payload.set_item("can_set_priority", capabilities.can_set_priority)?;
    payload.set_item("can_set_affinity", capabilities.can_set_affinity)?;
    Ok(payload.unbind())
}

fn snapshot_to_python<'py>(py: Python<'py>, snapshot: Snapshot) -> PyResult<Py<PyDict>> {
    let root = PyDict::new(py);
    root.set_item("timestamp_ms", snapshot.timestamp_ms)?;

    let system = PyDict::new(py);
    system.set_item("cpu_usage_percent", snapshot.system.cpu_usage_percent)?;
    system.set_item("logical_cpu_count", snapshot.system.logical_cpu_count)?;
    system.set_item("total_memory_bytes", snapshot.system.total_memory_bytes)?;
    system.set_item("used_memory_bytes", snapshot.system.used_memory_bytes)?;
    system.set_item("total_swap_bytes", snapshot.system.total_swap_bytes)?;
    system.set_item("used_swap_bytes", snapshot.system.used_swap_bytes)?;
    system.set_item("uptime_seconds", snapshot.system.uptime_seconds)?;
    root.set_item("system", system)?;

    let processes = PyList::empty(py);
    for process in snapshot.processes {
        let row = PyDict::new(py);
        row.set_item("pid", process.pid)?;
        row.set_item("parent_pid", process.parent_pid)?;
        row.set_item("name", process.name)?;
        row.set_item("status", process.status)?;
        row.set_item("cpu_percent", process.cpu_percent)?;
        row.set_item("memory_bytes", process.memory_bytes)?;
        row.set_item("virtual_memory_bytes", process.virtual_memory_bytes)?;
        processes.append(row)?;
    }
    root.set_item("processes", processes)?;
    Ok(root.unbind())
}
