//! Context-manager support for Python environments.
//!
//! The binding layer keeps a thread-local "active environment" so constructors
//! for variables, expressions, and related wrappers can omit the environment
//! argument inside a `with Environment(): ...` block.

use pyo3::{Bound, PyAny, PyResult, pymethods};

use lunamodel_error::py::PyMultipleActiveEnvironmentsError;
use lunamodel_unwind::*;

use super::{ACTIVE_ENV, PyEnvironment};

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Enter this environment as the active thread-local construction context.
    ///
    /// Nested active environments are rejected because many Python constructors
    /// implicitly resolve through this context, and ambiguous scoping would make
    /// accidental cross-environment objects difficult to diagnose.
    fn __enter__(&self) -> PyResult<Self> {
        ACTIVE_ENV.with(|curr| {
            let mut mc = curr.borrow_mut();
            if mc.is_some() {
                return Err(PyMultipleActiveEnvironmentsError::new_err(
                    "multiple active environments are not allowed",
                ));
            }
            *mc = Some(self.clone());
            Ok(())
        })?;
        Ok(self.clone())
    }

    /// Clear the active thread-local construction context on block exit.
    fn __exit__(
        &self,
        _exc_type: &Bound<'_, PyAny>,
        _exc_value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        ACTIVE_ENV.with(|current| {
            *current.borrow_mut() = None;
        });
        Ok(())
    }
}
