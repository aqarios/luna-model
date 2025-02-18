use std::{cell::RefCell, rc::Rc};

use crate::core::Environment;

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Environment")]
#[derive(Deref, DerefMut)]
pub struct PyEnvironment(pub Rc<RefCell<Environment>>);

#[pymethods]
impl PyEnvironment {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment(Rc::new(RefCell::new(Environment::new()))))
    }
}
