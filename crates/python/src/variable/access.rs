//! Accessors for Python variables.

use lunamodel_unwind::*;
use pyo3::prelude::*;

use super::PyVariable;
use crate::{PyEnvironment, bounds::PyBounds, types::PyVtype};

#[unwindable]
#[pymethods]
impl PyVariable {
    #[getter]
    fn id(&self) -> PyResult<usize> {
        self.v.check_living()?;
        Ok(self.v.id() as usize)
    }

    #[getter]
    fn name(&self) -> PyResult<String> {
        self.v.check_living()?;
        Ok(self.v.name()?)
    }
    #[getter]
    fn bounds(&self) -> PyResult<PyBounds> {
        self.v.check_living()?;
        Ok(self.v.bounds()?.into())
    }

    #[getter]
    fn vtype(&self) -> PyResult<PyVtype> {
        self.v.check_living()?;
        Ok(self.v.vtype()?.into())
    }

    #[getter]
    fn environment(&self) -> PyResult<PyEnvironment> {
        self.v.check_living()?;
        Ok(PyEnvironment {
            env: self.v.env.clone(),
        })
    }

    fn __hash__(&self) -> PyResult<u64> {
        self.v.check_living()?;
        Ok(self.v.hash()?)
    }
}
