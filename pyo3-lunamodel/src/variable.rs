use lunamodel_core::prelude::VarRef;
use lunamodel_python::ffi::CapsuleFFI;
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::LUNA_MODEL;

#[repr(transparent)]
/// A wrapper around a [`VarRef`] that can be converted to and from python with `pyo3`.
pub struct PyVariable(pub VarRef);

impl<'a, 'py> FromPyObject<'a, 'py> for PyVariable {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: (u32, Bound<'py, PyCapsule>) = if let Some(pye) = obj.getattr("_v").ok() {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let vref = VarRef::from_capsule(capsule)?;
        Ok(Self(vref))
    }
}

impl<'py> IntoPyObject<'py> for PyVariable {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        let lm = LUNA_MODEL.bind(py);
        let pyv = lm
            .getattr("_lm")?
            .getattr("PyVariable")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("Variable")?.call_method1("_from_pyvar", (pyv,))
    }
}
