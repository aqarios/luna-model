use std::sync::Arc;

use lunamodel_core::prelude::Constraint;
use lunamodel_python::ffi::CapsuleFFI;
use parking_lot::RwLock;
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
/// A wrapper around a [`Constraint`] that can be converted to and from python with `pyo3`.
pub struct PyConstraint(pub Arc<RwLock<Constraint>>);

impl<'a, 'py> FromPyObject<'a, 'py> for PyConstraint {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Constraint")?;
        let capsule: Bound<'py, PyCapsule> = if let Ok(pye) = obj.getattr("_c") {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pyc = Arc::<RwLock<Constraint>>::from_capsule(capsule)?;
        Ok(Self(pyc))
    }
}

impl<'py> IntoPyObject<'py> for PyConstraint {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        let lm = luna_model(py)?;
        let pycc = lm
            .getattr("_lm")?
            .getattr("PyConstraint")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("Constraint")?.call_method1("_from_pyc", (pycc,))
    }
}
