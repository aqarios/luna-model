use lunamodel_core::prelude::*;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, types::PyAnyMethods};

use crate::LUNA_MODEL;

#[repr(transparent)]
#[derive(Debug, Clone)]
/// A wrapper around a [`ArcEnv`] that can be converted to and from python with `pyo3`.
pub struct PyEnvironment(pub ArcEnv);

impl<'a, 'py> FromPyObject<'a, 'py> for PyEnvironment {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let raw_ptr: usize = obj
            .getattr("_env")?
            .call_method0("_into_raw_ptr")?
            .extract()?;
        let arc: ArcEnv = ArcEnv::from_raw_ptr(raw_ptr.into());
        Ok(PyEnvironment(arc))
    }
}

impl<'py> IntoPyObject<'py> for PyEnvironment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let ptr: usize = self.0.into_raw_ptr().into();
        let lm = LUNA_MODEL.bind(py);
        lm.call_method1("Environment", (ptr,))
    }
}
