use crate::{
    core::{
        environment::get_vref_by_name, ConcreteEnvironment, ConcreteMutRcEnvironment, Environment,
    },
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{prelude::*, types::PyBytes};
use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::{py_exceptions::MultipleActiveEnvironmentsError, py_var::PyVariable};

#[pyclass(unsendable, name = "Environment", module = "aqmodels")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyEnvironment(pub ConcreteMutRcEnvironment);

impl Into<ConcreteMutRcEnvironment> for PyEnvironment {
    fn into(self) -> ConcreteMutRcEnvironment {
        self.0
    }
}

impl PyEnvironment {
    pub fn new(env: ConcreteEnvironment) -> Self {
        Self(env.into())
    }
}

thread_local! {
    pub static CURRENT_ENV: RefCell<Option<PyEnvironment>> = RefCell::new(None);
}

#[pymethods]
impl PyEnvironment {
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(Environment::new()))
    }

    fn __enter__(&self) -> PyResult<Self> {
        CURRENT_ENV.with(|current| {
            let mut mut_curr = current.borrow_mut();
            if mut_curr.is_some() {
                return Err(MultipleActiveEnvironmentsError::new_err(
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
        format!("{:#?}", self.borrow())
    }

    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(PyVariable(Rc::new(get_vref_by_name(
            &name,
            Rc::clone(&self.0),
        )?)))
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .borrow()
                .deref()
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
            .into())
    }

    #[pyo3(signature=(compress=None, level=None))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<PyObject> {
        self.encode(py, compress, level)
    }

    #[staticmethod]
    fn decode(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(PyEnvironment::new(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        ))
    }

    #[staticmethod]
    fn deserialize(py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(py, data)
    }

    fn __eq__(&self, other: &PyEnvironment) -> bool {
        *self.borrow() == *other.borrow()
    }
}
