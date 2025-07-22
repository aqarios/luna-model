use pyo3::exceptions::PyRuntimeError;
use pyo3::{PyResult};
use std::panic::{self, AssertUnwindSafe, PanicHookInfo};

pub fn unwind<T, F>(f: F) -> PyResult<T>
where
    F: FnOnce() -> PyResult<T>,
{
    let prev_hook = panic::take_hook();
    panic::set_hook(Box::new(|_info: &PanicHookInfo| {}));
    let result = panic::catch_unwind(AssertUnwindSafe(f));
    panic::set_hook(prev_hook);

    match result {
        Ok(inner) => inner,
        Err(payload) => {
            let msg = if let Some(s) = payload.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = payload.downcast_ref::<String>() {
                s.clone()
            } else {
                "rust panic occurred".to_string()
            };
            Err(PyRuntimeError::new_err(msg))
        }
    }
}
