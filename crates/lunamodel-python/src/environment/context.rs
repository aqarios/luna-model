use pyo3::{Bound, PyAny, PyResult, pymethods};

use crate::exceptions::MultipleActiveEnvironmentsError;

use super::{PyEnvironment, ACTIVE_ENV};

#[pymethods]
impl PyEnvironment {
    /// Activate this environment for variable creation.
    ///
    /// Returns
    /// -------
    /// Environment
    ///     The current environment (self).
    ///
    /// Raises
    /// ------
    /// MultipleActiveEnvironmentsError
    ///     If another environment is already active.
    fn __enter__(&self) -> PyResult<Self> {
        ACTIVE_ENV.with(|curr| {
            let mut mc = curr.borrow_mut();
            if mc.is_some() {
                return Err(MultipleActiveEnvironmentsError::new_err(
                    "multiple active environments are not allowed",
                ));
            }
            *mc = Some(self.clone());
            Ok(())
        })?;
        Ok(self.clone())
    }

    /// Deactivate this environment.
    ///
    /// Called automatically at the end of a `with` block.
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
