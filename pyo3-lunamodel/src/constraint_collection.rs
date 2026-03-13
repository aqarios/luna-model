use lunamodel_python::{ffi::CapsuleFFI, prelude::PyConstraintCollectionContent as PyCCC};
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::LUNA_MODEL;

#[repr(transparent)]
/// A wrapper around a [`PyCCC`] that can be converted to and from python with `pyo3`.
pub struct PyConstraintCollection(pub PyCCC);

impl<'a, 'py> FromPyObject<'a, 'py> for PyConstraintCollection {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        let capsule: Bound<'py, PyCapsule> = if let Some(pye) = obj.getattr("_cc").ok() {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pyccc = PyCCC::from_capsule(capsule)?;
        Ok(Self(pyccc))
    }
}

impl<'py> IntoPyObject<'py> for PyConstraintCollection {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        let lm = LUNA_MODEL.bind(py);
        let pycc = lm
            .getattr("_lm")?
            .getattr("PyConstraintCollection")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("ConstraintCollection")?
            .call_method1("_from_pycc", (pycc,))
    }
}
