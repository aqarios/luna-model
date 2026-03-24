use lunamodel_python::{PyUnbounded as PyU, ffi::CapsuleFFI};
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, types::PyAnyMethods};

use crate::{LUNA_MODEL, utils::TypeCheck};

#[repr(transparent)]
/// A matching type for [`PyU`] that can be converted to and from python with `pyo3`.
pub struct PyUnbounded;

impl<'a, 'py> FromPyObject<'a, 'py> for PyUnbounded {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Unbounded")?;
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: String = if let Some(pye) = obj.getattr("_val").ok() {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        // NOTE: This line makes sure the correct type was passed. Fails otherwise.
        let _ = PyU::from_capsule(capsule)?;
        Ok(PyUnbounded {})
    }
}

impl<'py> IntoPyObject<'py> for PyUnbounded {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pyunbounded: PyU = PyU {};
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = LUNA_MODEL.bind(py);
        lm.getattr("_lm")?
            .getattr("PyUnbounded")?
            .call_method1("_from_capsule", (pyunbounded.to_capsule(py)?,))
        // NOTE: We don't need this, since currently Unbounded is an alias of PyUnbounded.
        // lm.getattr("Sense")?.call_method1("_from_pysense", (pyv,))
    }
}
