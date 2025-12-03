use lunamodel_types::Vtype;
use pyo3::prelude::*;

use super::PyVariable;
use crate::{PyEnvironment, bounds::PyBounds};

#[pymethods]
impl PyVariable {
    #[getter]
    fn id(&self) -> usize {
        self.v.id() as usize
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        Ok(self.v.name()?)
    }
    #[getter]
    fn bounds(&self) -> PyResult<PyBounds> {
        // todo: special bounds right away or rely on outside protocol?
        Ok(PyBounds(self.v.bounds()?.into()))
    }

    #[getter]
    fn vtype(&self) -> PyResult<Vtype> {
        Ok(self.v.vtype()?)
    }

    #[getter]
    fn environment(&self) -> PyEnvironment {
        PyEnvironment {
            env: self.v.env.clone(),
        }
    }

    fn __hash__(&self) -> PyResult<u64> {
        Ok(self.v.hash()?)
    }
}
