use lunamodel_python::{PyBoundsContent, ffi::CapsuleFFI};
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
/// A wrapper around [`BoundsContent`] that can be converted to and from python with `pyo3`.
pub struct PyBounds(pub PyBoundsContent);

impl<'a, 'py> FromPyObject<'a, 'py> for PyBounds {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Bounds")?;
        // check if it is the wrapper type or the PyBounds type from the crate.
        let capsule: Bound<'py, PyCapsule> = if let Ok(pye) = obj.getattr("_b") {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pyc = PyBoundsContent::from_capsule(capsule)?;
        Ok(PyBounds(pyc))
    }
}

impl<'py> IntoPyObject<'py> for PyBounds {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pye_capsule = self.0.to_capsule(py)?;
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = luna_model(py)?;
        let pye = lm
            .getattr("_lm")?
            .getattr("PyBounds")?
            .call_method1("_from_capsule", (pye_capsule,))?;
        lm.getattr("Bounds")?.call_method1("_from_pyb", (pye,))
    }
}
