use lunamodel_core::prelude::ArcEnv;
use lunamodel_python::PyEnvironment as PyE;
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{LUNA_MODEL, utils::TypeCheck};

#[repr(transparent)]
/// A wrapper around a [`ArcEnv`] that can be converted to and from python with `pyo3`.
pub struct PyEnvironment(pub ArcEnv);

impl<'a, 'py> FromPyObject<'a, 'py> for PyEnvironment {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Environment")?;
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: Bound<'py, PyCapsule> = if let Some(pye) = obj.getattr("_env").ok() {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pye = PyE::_from_capsule(&capsule)?;
        Ok(PyEnvironment(pye.env))
    }
}

impl<'py> IntoPyObject<'py> for PyEnvironment {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pye_capsule = PyE { env: self.0 }._to_capsule(py)?;
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = LUNA_MODEL.bind(py);
        let pye = lm
            .getattr("_lm")?
            .getattr("PyEnvironment")?
            .call_method1("_from_capsule", (pye_capsule,))?;
        lm.getattr("Environment")?
            .call_method1("_from_pyenv", (pye,))
    }
}
