use crate::{
    core::{Environment, MultipleActiveEnvironmentsException, VarId},
    serialization::{decode_environment, encode_environment},
};
use derive_more::{Deref, DerefMut};
use pyo3::{exceptions::PyRuntimeError, prelude::*, types::PyBytes};
use std::{cell::RefCell, rc::Rc};

#[pyclass(unsendable, name = "Environment")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyEnvironment(pub Rc<RefCell<Environment<VarId>>>);

impl Into<Rc<RefCell<Environment<VarId>>>> for PyEnvironment {
    fn into(self) -> Rc<RefCell<Environment<VarId>>> {
        self.0
    }
}

impl PyEnvironment {
    pub fn new() -> Self {
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

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.borrow())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        Ok(PyBytes::new(
            py,
            &encode_environment(&self.borrow(), compress.unwrap_or(level.is_some()), level)?,
        )
        .into())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        self.serialize(py, compress, level)
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        // todo, handle env
        let bytes: &[u8] = data.as_bytes(py);
        let env = decode_environment(bytes);
        match env {
            Ok(env) => Ok(PyEnvironment(Rc::new(RefCell::new(env)))),
            Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
        }
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::deserialize(py, data)
    }

    fn __eq__(&self, other: &PyEnvironment) -> bool {
        *self.borrow() == *other.borrow()
    }
}
