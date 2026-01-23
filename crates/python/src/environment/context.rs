use pyo3::{Bound, PyAny, PyResult, pymethods};

use lunamodel_error::py::PyMultipleActiveEnvironmentsError;
use lunamodel_unwind::unwindable;

use super::{ACTIVE_ENV, PyEnvironment};
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PyEnvironment {
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
