use std::{cell::RefCell, rc::Rc};

use crate::core::{Environment, MultipleActiveEnvironmentsException};

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Environment")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyEnvironment(pub Rc<RefCell<Environment>>);

impl PyEnvironment {
    fn new() -> Self {
        Self(Rc::new(RefCell::new(Environment::new())))
    }
}

thread_local! {
    pub static CURRENT_ENV: RefCell<Option<PyEnvironment>> = RefCell::new(None);
}

#[pymethods]
impl PyEnvironment {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new())
    }

    fn __enter__(&self) -> PyResult<Self> {
        CURRENT_ENV.with(|current| {
            let mut mut_curr = current.borrow_mut();
            if mut_curr.is_some() {
                return Err(MultipleActiveEnvironmentsException::new_err(
                    "multiple active environments are not allowed",
                ));
            }
            *mut_curr = Some(self.clone());
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
        CURRENT_ENV.with(|current| {
            *current.borrow_mut() = None;
        });
        Ok(())
    }
}
